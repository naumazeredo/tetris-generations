use std::io;
use std::net::{SocketAddr, UdpSocket, ToSocketAddrs, Ipv4Addr};

use super::*;

#[derive(Debug)]
pub enum ClientError {
    ClientNotConnected,
    ClientAlreadyConnected,
    PacketNotFromServer,
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// @TODO this should be linked with the player account
pub type ClientId = u64;

#[derive(Debug)]
pub enum ClientEvent {
    ServerHeartbeat,
    ServerConnectionAccept,
    ServerConnectionReject(ConnectionRejectReason),
    ServerTimedOut,
    DisconnectedByServer,
    SentHeartbeat,
    SentConnectionRetry,
    Data(DataPayload),
}

#[derive(Clone, Debug)]
pub struct ServerConnection {
    server_addr: SocketAddr,
    heartbeat: ConnectionHeartbeat,
}

impl ServerConnection {
    pub fn server_addr(&self) -> SocketAddr { self.server_addr }
    pub fn heartbeat(&self) -> &ConnectionHeartbeat { &self.heartbeat }
}

#[derive(Copy, Clone, Debug)]
pub enum ConnectStep {
    SentRequest,
    SentChallengeResponse,
}

#[derive(Clone, Debug)]
pub enum ClientState {
    NotConnected,
    Connecting(ServerConnection, ConnectStep),
    Connected(ServerConnection),
}

#[derive(Debug)]
pub struct Client {
    id: ClientId,
    addr: SocketAddr,
    state: ClientState,
    socket: UdpSocket,
}

// Fields public interface
impl Client {
    pub fn new(id: ClientId) -> Result<Self, NetError> {
        match UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)) {
            Ok(s) => {
                s.set_nonblocking(true).unwrap();

                let addr = s.local_addr()?;

                Ok(Self {
                    state: ClientState::NotConnected,
                    socket: s,
                    id,
                    addr,
                })
            }

            Err(e) => Err(e.into()),
        }
    }

    pub fn id(&self) -> ClientId        { self.id }
    pub fn addr(&self) -> SocketAddr    { self.addr }
    pub fn state(&self) -> &ClientState { &self.state }
}

// Connections public interface
impl Client {
    pub fn next_event(&mut self) -> Result<Option<ClientEvent>, NetError> {
        if let ClientState::NotConnected = self.state {
            return Err(ClientError::ClientNotConnected.into());
        }

        let mut buffer = [0; NET_MAX_PACKET_SIZE];
        match self.socket.recv_from(&mut buffer) {
            Ok((len, addr)) => {
                let event = self.handle_packet(&addr, &buffer[..len]);
                if let Ok(Some(_)) = event {
                    return event;
                }
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {},
            Err(e) => return Err(e.into()),
        };

        match &mut self.state {
            ClientState::Connecting(conn, _) => {
                // Check if server timed out the connection request
                if conn.heartbeat.has_timed_out(NET_CONNECT_TIMEOUT_INTERVAL) {
                    // @TODO logging
                    println!("[net][client] server didn't respond connection request: timeout!");

                    // @TODO  TimedOut/NotConnected(TimedOut) instead of just NotConnected?
                    self.state = ClientState::NotConnected;
                    return Ok(Some(ClientEvent::ServerTimedOut));
                }

                // Retry if server didn't respond in a heartbeat
                if conn.heartbeat.should_retry_send(NET_CONNECT_RETRY_INTERVAL) {
                    // @TODO logging
                    println!("[net][client] server didn't respond connection request: retrying!");

                    // Retry connection (restart, even if already sent the challenge response)
                    let message = ConnectionRequest::build_message(self.id);
                    Self::send_message(message, &self.socket, conn)?;
                    conn.heartbeat.update_sent();

                    let state = std::mem::replace(&mut self.state, ClientState::NotConnected);
                    match state {
                        ClientState::Connecting(conn, _) =>
                            self.state = ClientState::Connecting(conn, ConnectStep::SentRequest),
                        _ => unreachable!(),
                    };

                    return Ok(Some(ClientEvent::SentConnectionRetry));
                }
            },

            ClientState::Connected(conn) => {
                // Tick server, in case heartbeat needed, or check if it timed out
                if conn.heartbeat.has_timed_out(NET_TIMEOUT_INTERVAL) {
                    // @TODO logging
                    println!("[net][client] server timed out!");

                    // @TODO  TimedOut/NotConnected(TimedOut) instead of just NotConnected?
                    self.state = ClientState::NotConnected;
                    return Ok(Some(ClientEvent::ServerTimedOut));
                }

                if conn.heartbeat.should_retry_send(NET_RETRY_INTERVAL) {
                    // @TODO logging
                    println!("[net][client] sending heartbeat!");

                    let message = Heartbeat::build_message();
                    Self::send_message(message, &self.socket, conn)?;
                    return Ok(Some(ClientEvent::SentHeartbeat));
                }
            },

            _ => unreachable!(),
        }

        Ok(None)
    }

    pub fn connect<A: ToSocketAddrs>(&mut self, server_addr: A) -> Result<(), NetError> {
        if let ClientState::Connected(_) = self.state {
            return Err(ClientError::ClientAlreadyConnected.into());
        }

        let server_addr = server_addr
            .to_socket_addrs()?
            .next()
            .ok_or(NetError::InvalidAddress)?;

        let mut conn = ServerConnection {
            server_addr,
            heartbeat: ConnectionHeartbeat::new(),
        };

        let message = ConnectionRequest::build_message(self.id);
        Self::send_message(message, &self.socket, &mut conn)?;

        self.state = ClientState::Connecting(conn, ConnectStep::SentRequest);

        Ok(())
    }

    pub fn disconnect(&mut self) -> Result<(), NetError> {
        let state = std::mem::replace(&mut self.state, ClientState::NotConnected);
        let mut conn = match state {
            ClientState::Connecting(conn, _) | ClientState::Connected(conn) => conn,
            _ => return Err(ClientError::ClientNotConnected.into()),
        };

        let message = DisconnectNotice::build_message(self.id);
        Self::send_message(message, &self.socket, &mut conn)?;

        Ok(())
    }

    pub fn send<S: Serialize>(&mut self, data: S) -> Result<(), NetError> {
        let conn = match &mut self.state {
            ClientState::Connected(conn) => conn,
            _ => return Err(ClientError::ClientNotConnected.into()),
        };

        let message = Data::build_message(data)?;
        Self::send_message(message, &self.socket, conn)
    }
}

// Private functions
impl Client {
    fn handle_packet(
        &mut self,
        addr: &SocketAddr,
        data: &[u8],
    ) -> Result<Option<ClientEvent>, NetError> {
        let message = Message::parse(data)?;
        //println!("[net][client] received message (len: {}):\n{:?}", data.len(), message);

        match &mut self.state {
            ClientState::Connecting(conn, _) => {
                if *addr != conn.server_addr {
                    return Err(ClientError::PacketNotFromServer.into());
                }

                // Don't update the heartbeat since it can be an incorrect message which will
                // maintain the connecting state alive longer than expected
                self.handle_connecting(message)
            }

            ClientState::Connected(conn) => {
                if *addr != conn.server_addr {
                    return Err(ClientError::PacketNotFromServer.into());
                }

                conn.heartbeat.update_recv();
                self.handle_connected(message)
            }

            _ => unreachable!(),
        }
    }

    fn handle_connecting(
        &mut self,
        message: Message,
    ) -> Result<Option<ClientEvent>, NetError> {
        let step = if let ClientState::Connecting(_, step) = self.state { step } else { unreachable!(); };

        match message.payload {
            MessagePayload::ConnectionAccept => {
                match step {
                    ConnectStep::SentRequest => {
                        // @XXX is there a better way of swapping between connecting and connected without
                        //      copying the server connection?
                        let state = std::mem::replace(&mut self.state, ClientState::NotConnected);
                        match state {
                            ClientState::Connecting(conn, _) => self.state = ClientState::Connected(conn),
                            _ => unreachable!(),
                        };

                        Ok(Some(ClientEvent::ServerConnectionAccept))
                    },

                    ConnectStep::SentChallengeResponse => todo!(),
                }
            }

            MessagePayload::ConnectionReject(reason) => {
                self.state = ClientState::NotConnected;
                Ok(Some(ClientEvent::ServerConnectionReject(reason)))
            }

            _ => {
                // @TODO logging
                println!("[net][client] connecting: received invalid message: {:?}", message);
                Ok(None)
            }
        }
    }

    fn handle_connected(
        &mut self,
        message: Message,
    ) -> Result<Option<ClientEvent>, NetError> {
        match message.payload {
            MessagePayload::Heartbeat => Ok(Some(ClientEvent::ServerHeartbeat)),

            MessagePayload::DisconnectNotice(id) if id == self.id => {
                // @TODO logging
                println!("[net][client] client got disconnect by server!");

                self.state = ClientState::NotConnected;
                Ok(Some(ClientEvent::DisconnectedByServer))
            }

            MessagePayload::Data(data_payload) => Ok(Some(ClientEvent::Data(data_payload))),

            _ => {
                // @TODO logging
                println!("[net][client] connected: received invalid message: {:?}", message);
                Ok(None)
            }
        }
    }

    fn send_message(
        message: Message,
        socket: &UdpSocket,
        connection: &mut ServerConnection,
    ) -> Result<(), NetError> {
        let (packet_data, packet_len) = message.create_packet()?;
        //println!("[net][client] sending mesage (len: {}):\n{:?}", packet_len, message);

        match socket.send_to(&packet_data[..packet_len], connection.server_addr) {
            Ok(len) => {
                //println!("[net][client] packet sent    : {} -> {:?}", connection.server_addr, message);

                assert!(len == packet_len);
                connection.heartbeat.update_sent();
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }
}

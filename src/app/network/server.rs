use std::io;
use std::net::{SocketAddr, UdpSocket, ToSocketAddrs};
//use byteorder::{LittleEndian, BigEndian, ReadBytesExt, WriteBytesExt};

use super::*;

#[derive(Debug)]
pub enum ServerError {
    PacketNotFromConnectedClient,
    ClientNotConnected,
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum ServerEvent {
    ClientHeartbeat(ClientId),
    ClientConnect(ClientId),
    ClientReject(ClientId, SocketAddr),
    ClientDisconnect(ClientId),
    ClientTimeOut(ClientId),
    SentHeartbeat(ClientId),
    Data(ClientId, DataPayload),
}

#[derive(Debug)]
pub struct ClientConnection {
    client_id: ClientId,
    client_addr: SocketAddr,
    heartbeat: ConnectionHeartbeat,
}

impl ClientConnection {
    pub fn client_id(&self) -> ClientId { self.client_id }
    pub fn addr(&self) -> SocketAddr    { self.client_addr }
    pub fn heartbeat(&self) -> &ConnectionHeartbeat { &self.heartbeat }
}

/*
enum ConnectionState {
    Connected,
}
*/

pub type ConnectionList = Vec<ClientConnection>;

#[derive(Debug)]
pub struct Server {
    socket: UdpSocket,
    listen_addr: SocketAddr,
    connections: ConnectionList,
    client_event_index: usize,
}

// Fields public interface
impl Server {
    pub fn new<A: ToSocketAddrs>(local_addr: A) -> Result<Self, NetError> {
        let bind_addr = local_addr
            .to_socket_addrs()?
            .next()
            .ok_or(NetError::InvalidAddress)?;

        match UdpSocket::bind(bind_addr) {
            Ok(s) => {
                // @TODO: logger
                println!("[network][server] UDP socket bound");
                s.set_nonblocking(true).unwrap();

                Ok(Self {
                    socket: s,
                    listen_addr:   bind_addr,
                    connections:   Vec::new(),
                    client_event_index: 0,
                })
            }

            Err(e) => Err(e.into()),
        }
    }

    pub fn addr(&self) -> SocketAddr             { self.listen_addr }
    pub fn connections(&self) -> &ConnectionList { &self.connections }
}

// Connections public interface
impl Server {
    pub fn next_event(
        &mut self,
    ) -> Result<Option<ServerEvent>, NetError> {
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

        // In case we plan to have many connetions (>8) or async next_event, we can use a queue of
        // timestamps to quickly check if any connection needs a heartbeat check
        for _ in 0..self.connections.len() {
            if self.client_event_index >= self.connections.len() {
                self.client_event_index = 0;
            }

            let mut conn = &mut self.connections[self.client_event_index];

            if conn.heartbeat.has_timed_out(NET_TIMEOUT_INTERVAL) {
                // @TODO logging
                println!("[net][server] client connection lost!");

                // @TODO maybe send the timeout?
                let client_id = conn.client_id;
                self.disconnect_client(client_id)?;
                return Ok(Some(ServerEvent::ClientTimeOut(client_id)));
            }

            if conn.heartbeat.should_retry_send(NET_RETRY_INTERVAL) {
                // @TODO logging
                //println!("[net][server] sending heartbeat!");

                let message = Heartbeat::build_message();
                Self::send_message(message, &self.socket, &mut conn)?;

                return Ok(Some(ServerEvent::SentHeartbeat(conn.client_id)));
            }
        }

        Ok(None)
    }

    pub fn disconnect_client(
        &mut self,
        client_id: ClientId,
    ) -> Result<(), NetError> {
        let index = self.connections.iter().position(|conns| conns.client_id == client_id)
            .ok_or(NetError::from(ServerError::PacketNotFromConnectedClient))?;

        let message = DisconnectNotice::build_message(client_id);
        Self::send_message(message, &self.socket, &mut self.connections[index])?;

        // Maintain order of connections. In case we want too many connections, we
        // should improve this somehow
        self.connections.remove(index);
        if index > self.client_event_index { self.client_event_index -= 1; }

        Ok(())
    }

    pub fn disconnect_all(&mut self) -> Result<(), NetError> {
        let connections = std::mem::take(&mut self.connections);
        let disconnects_result = connections
            .into_iter()
            .map(|mut conn| {
                let message = DisconnectNotice::build_message(conn.client_id);
                Self::send_message(message, &self.socket, &mut conn)
            })
            .collect();

        self.connections.clear();
        disconnects_result
    }

    pub fn send<S: Serialize>(&mut self, client_id: ClientId, data: S) -> Result<(), NetError> {
        let conn = self.connections
            .iter_mut()
            .find(|conns| conns.client_id == client_id)
            .ok_or(NetError::from(ServerError::ClientNotConnected))?;

        let message = Data::build_message(data)?;
        Self::send_message(message, &self.socket, conn)
    }

    pub fn broadcast<S: Serialize>(&mut self, data: S) -> Result<(), NetError> {
        let message = Data::build_message(data)?;

        let mut result = Ok(());
        for conn in self.connections.iter_mut() {
            match Self::send_message(message.clone(), &self.socket, conn) {
                Ok(_) => {},
                Err(e) => result = Err(e),
            }
        }
        result

        /*
        // @XXX Rust is being shitty here. It borrows the whole self as immutable instead of only
        //      self.connections. It makes sense, but it's annoying since it works with for, which
        //      also needs to hold the ownership of the iterator.
        self.connections
            .iter_mut()
            .map(|conn| {
                Self::send_message(message.clone(), &self.socket, conn)
            })
            .collect()
        */
    }
}

// Private functions
impl Server {
    fn handle_packet(
        &mut self,
        addr: &SocketAddr,
        data: &[u8],
    ) -> Result<Option<ServerEvent>, NetError> {
        let message = Message::parse(data)?;
        //println!("[net][server] received message (len: {}):\n{:?}", data.len(), message);

        match message.payload {
            MessagePayload::ConnectionRequest(client_id) => {
                //println!("[net][server] received connection request: {:?}", message);

                // Check if client is already connected (same address or same id)
                if self.connections
                    .iter_mut()
                    .find(|conn| conn.client_addr == *addr || conn.client_id == client_id)
                    .is_some()
                {
                    let message = ConnectionReject::build_message(
                        ConnectionRejectReason::AlreadyConnected
                    );
                    Self::send_message_to_addr(message, &self.socket, addr)?;
                    Ok(Some(ServerEvent::ClientReject(client_id, addr.clone())))
                } else {
                    let mut conn = Self::new_client(client_id, addr, &mut self.connections);
                    let message = ConnectionAccept::build_message();
                    Self::send_message(message, &self.socket, &mut conn)?;
                    Ok(Some(ServerEvent::ClientConnect(client_id)))
                }
            }

            MessagePayload::DisconnectNotice(client_id) => {
                let conn = Self::get_connection(&mut self.connections, addr)
                    .ok_or(NetError::from(ServerError::PacketNotFromConnectedClient))?;

                if conn.client_id == client_id {
                    self.disconnect_client(client_id)?;
                    Ok(Some(ServerEvent::ClientDisconnect(client_id)))
                } else {
                    // @TODO logging
                    println!(
                        "[net][server] received disconnected with wrong client id: {:?}",
                        message
                    );
                    Ok(None)
                }
            }

            MessagePayload::Heartbeat => {
                let conn = Self::get_connection(&mut self.connections, addr)
                    .ok_or(NetError::from(ServerError::PacketNotFromConnectedClient))?;

                conn.heartbeat.update_recv();
                Ok(Some(ServerEvent::ClientHeartbeat(conn.client_id)))
            }

            MessagePayload::Data(data_payload) => {
                let conn = Self::get_connection(&mut self.connections, addr)
                    .ok_or(NetError::from(ServerError::PacketNotFromConnectedClient))?;
                Ok(Some(ServerEvent::Data(conn.client_id, data_payload)))
            }

            _ => {
                // @TODO logging
                println!("[net][server] received invalid message: {:?}", message);
                Ok(None)
            }
        }
    }

    fn get_connection<'a>(
        connections: &'a mut ConnectionList,
        addr: &SocketAddr,
    ) -> Option<&'a mut ClientConnection> {
        connections
            .iter_mut()
            .find(|conn| conn.client_addr == *addr)
    }

    fn new_client<'a>(
        client_id: ClientId,
        addr: &SocketAddr,
        connections: &'a mut ConnectionList,
    ) -> &'a mut ClientConnection {

        let connection = ClientConnection {
            client_id,
            client_addr: *addr,
            heartbeat: ConnectionHeartbeat::new(),
        };

        connections.push(connection);
        connections.last_mut().unwrap()
    }

    fn send_message(
        message: Message,
        socket: &UdpSocket,
        connection: &mut ClientConnection,
    ) -> Result<(), NetError> {
        Self::send_message_to_addr(message, socket, &connection.client_addr)
            .and_then(|()| {
                //println!("[net][server] packet received: {} -> {:?}", connection.client_addr, message);
                connection.heartbeat.update_sent();
                Ok(())
            })
    }

    fn send_message_to_addr(
        message: Message,
        socket: &UdpSocket,
        addr: &SocketAddr,
    ) -> Result<(), NetError> {
        let (packet_data, packet_len) = message.create_packet()?;
        //println!("[net][server] sending message (len: {}):\n{:?}", packet_len, message);

        match socket.send_to(&packet_data[..packet_len], addr) {
            Ok(len) => { assert!(len == packet_len); Ok(()) }
            Err(e) => Err(e.into()),
        }
    }
}

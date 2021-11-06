mod client;
mod messages;
mod serialization;
mod server;

pub use client::*;
pub use messages::*;
pub use serialization::*;
pub use server::*;

use std::io;
use std::fmt;
use std::time::{Duration, Instant}; // @XXX should we use App real time?

use crate::app::ImDraw;

pub use serialization::*;

const NET_MAX_PACKET_SIZE : usize = 512; // @XXX this should be 256, but it's bigger since we didn't implement fragmentation
const NET_MAX_PAYLOAD_SIZE: usize = NET_MAX_PACKET_SIZE - 8; // This should always be divisible by 4
const NET_CHALLENGE_SIZE  : usize = 16;

const NET_RETRY_INTERVAL  : Duration = Duration::from_millis(100);
const NET_TIMEOUT_INTERVAL: Duration = Duration::from_millis(1_000);

const NET_CONNECT_RETRY_INTERVAL  : Duration = Duration::from_millis(500);
const NET_CONNECT_TIMEOUT_INTERVAL: Duration = Duration::from_millis(5_000);

#[derive(Debug)]
pub enum NetError {
    InvalidAddress,
    IoError(io::Error),
    SerializationError(SerializationError),
    ServerError(ServerError),
    ClientError(ClientError),
}

impl fmt::Display for NetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<io::Error> for NetError {
    fn from(error: io::Error) -> Self {
        NetError::IoError(error)
    }
}

impl From<SerializationError> for NetError {
    fn from(error: SerializationError) -> Self {
        NetError::SerializationError(error)
    }
}

impl From<ClientError> for NetError {
    fn from(error: ClientError) -> Self {
        NetError::ClientError(error)
    }
}

impl From<ServerError> for NetError {
    fn from(error: ServerError) -> Self {
        NetError::ServerError(error)
    }
}

impl std::error::Error for NetError {}

#[derive(Copy, Clone, Debug)]
pub struct ConnectionHeartbeat {
    last_sent: Instant,
    last_recv: Instant,
}

impl ConnectionHeartbeat {
    pub(super) fn new() -> Self {
        let now = Instant::now();
        Self {
            last_sent: now,
            last_recv: now,
        }
    }

    pub(super) fn update_sent(&mut self) {
        self.last_sent = Instant::now();
    }

    pub(super) fn update_recv(&mut self) {
        self.last_recv = Instant::now();
    }

    pub(super) fn should_retry_send(&self, interval: Duration) -> bool {
        self.last_sent.elapsed() >= interval
    }

    pub(super) fn has_timed_out(&self, interval: Duration) -> bool {
        self.last_recv.elapsed() >= interval
    }

    pub fn last_sent(&self) -> Instant { self.last_sent }
    pub fn last_recv(&self) -> Instant { self.last_recv }
}

impl_imdraw_todo!(Server);
impl_imdraw_todo!(Client);

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Copy, Clone)]
    struct MyData {
        i: i32,
        u: u32,
    }

    impl Serialize for MyData {
        fn serialize(&self, serializer: &mut Serializer) -> Result<(), SerializationError> {
            self.i.serialize(serializer)?;
            self.u.serialize(serializer)?;
            Ok(())
        }
    }

    impl Deserialize for MyData {
        fn deserialize(deserializer: &mut Deserializer) -> Result<Self, SerializationError> {
            let i = i32::deserialize(deserializer)?;
            let u = u32::deserialize(deserializer)?;
            Ok(MyData { i, u })
        }
    }

    fn server_next_event(server: &mut Server) -> Result<ServerEvent, NetError> {
        for _retries in 0..10 {
            match server.next_event() {
                Ok(None) => {},
                Ok(Some(v)) => return Ok(v),
                Err(err) => return Err(err),
            }
        }

        panic!("server did too many retries");
    }

    fn client_next_event(client: &mut Client) -> Result<ClientEvent, NetError> {
        for _retries in 0..10 {
            match client.next_event() {
                Ok(None) => {},
                Ok(Some(v)) => return Ok(v),
                Err(err) => return Err(err),
            }
        }

        panic!("client did too many retries");
    }



    #[test]
    fn client_server_normal_flow() -> Result<(), NetError> {
        let mut server = Server::new("127.0.0.1:42069")?;
        let mut client = Client::new(1)?;

        match client.state() {
            ClientState::NotConnected => println!("client not connected!"),
            _ => panic!("client not in a NotConnected state"),
        }

        // Try connect
        client.connect("127.0.0.1:42069")?;

        match client.state() {
            ClientState::Connecting(conn, _) => {
                println!("client connecting!");
                assert_eq!(server.addr(), conn.server_addr());
            },

            _ => panic!("client not in a Connecting state"),
        }

        let server_event = server_next_event(&mut server)?;
        match server_event {
            ServerEvent::ClientConnect(id) => assert_eq!(id, 1),
            _ => panic!("server event not ClientConnect"),
        }

        let client_event = client_next_event(&mut client)?;
        match client_event {
            ClientEvent::ServerConnectionAccept => {},
            _ => panic!("client event not ServerConnectionAccept"),
        }

        match client.state() {
            ClientState::Connected(conn) => {
                println!("client connected!");
                assert_eq!(server.addr(), conn.server_addr());
            },

            _ => panic!("client not in a Connected state"),
        }

        // Heartbeat
        println!("sleeping for {} ms...", NET_RETRY_INTERVAL.as_millis());
        std::thread::sleep(NET_RETRY_INTERVAL);

        let server_event = server_next_event(&mut server)?;
        match server_event {
            ServerEvent::SentHeartbeat(id) => assert_eq!(id, 1),
            _ => panic!("server event not SentHeartbeat"),
        }

        let client_event = client_next_event(&mut client)?;
        match client_event {
            ClientEvent::ServerHeartbeat => {},
            _ => panic!("client event not ServerHeartbeat"),
        }

        let client_event = client_next_event(&mut client)?;
        match client_event {
            ClientEvent::SentHeartbeat => {},
            _ => panic!("client event not SentHeartbeat"),
        }

        let server_event = server_next_event(&mut server)?;
        match server_event {
            ServerEvent::ClientHeartbeat(id) => assert_eq!(id, 1),
            _ => panic!("server event not ClientHeartbeat"),
        }

        // Send data
        let send_data = MyData { i: -42, u: 42 };
        client.send(send_data)?;

        let server_event = server_next_event(&mut server)?;
        match server_event {
            ServerEvent::Data(id, data_payload) => {
                println!("data_payload: {:?}", data_payload);
                assert_eq!(id, 1);

                let recv_data = MyData::parse(data_payload.data())?;
                assert_eq!(recv_data.i, send_data.i);
                assert_eq!(recv_data.u, send_data.u);
            }
            _ => panic!("server event not Data"),
        }

        let send_data = MyData { i: -43, u: 43 };
        server.send(1, send_data)?;

        let client_event = client_next_event(&mut client)?;
        match client_event {
            ClientEvent::Data(data_payload) => {
                println!("data_payload: {:?}", data_payload);
                let recv_data = MyData::parse(data_payload.data())?;
                assert_eq!(recv_data.i, send_data.i);
                assert_eq!(recv_data.u, send_data.u);
            }
            _ => panic!("client event not Data"),
        }

        // Disconnect
        client.disconnect()?;

        match client.state() {
            ClientState::NotConnected => println!("client not connected!"),
            _ => panic!("client not in a NotConnected state"),
        }

        let server_event = server_next_event(&mut server)?;
        match server_event {
            ServerEvent::ClientDisconnect(id) => assert_eq!(id, 1),
            _ => panic!("server event not ClientDisconnected"),
        }

        Ok(())
    }

    #[test]
    fn client_unavailable_server() -> Result<(), NetError> {
        let mut client = Client::new(1)?;

        match client.state() {
            ClientState::NotConnected => println!("client not connected!"),
            _ => panic!("client not in a NotConnected state"),
        }

        // Try connect
        client.connect("127.0.0.1:42069")?;

        match client.state() {
            ClientState::Connecting(conn, _) => {
                println!("client connecting!");
                assert_eq!(server.addr(), conn.server_addr());
            },

            _ => panic!("client not in a Connecting state"),
        }

        let client_event = client_next_event(&mut client)?;
        match client_event {
            ClientEvent::ServerConnectionAccept => {},
            _ => panic!("client event not ServerConnectionAccept"),
        }

        match client.state() {
            ClientState::Connected(conn) => {
                println!("client connected!");
                assert_eq!(server.addr(), conn.server_addr());
            },

            _ => panic!("client not in a Connected state"),
        }

        std::thread::sleep(NET_RETRY_INTERVAL);

        Ok(())
    }
}

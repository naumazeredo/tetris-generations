use super::*;

pub const PROTOCOL_ID: u32 = 0x2e413454;

#[derive(Clone, Debug)]
pub struct DataPayload {
    pub byte_count: u32,
    pub data: [u8; NET_MAX_PAYLOAD_SIZE],
}

impl Serialize for DataPayload {
    fn serialize(&self, serializer: &mut Serializer) -> Result<(), SerializationError> {
        if self.byte_count as usize >= NET_MAX_PAYLOAD_SIZE {
            return Err(SerializationError::ByteCountExceedsMaxSize);
        }

        serializer.serialize_u16(self.byte_count as u16)?;
        for i in 0..self.byte_count as usize {
            self.data[i].serialize(serializer)?;
        }

        Ok(())
    }
}

impl Deserialize for DataPayload {
    fn deserialize(deserializer: &mut Deserializer) -> Result<Self, SerializationError> {
        let byte_count = u16::deserialize(deserializer)?;
        if byte_count as usize>= NET_MAX_PAYLOAD_SIZE {
            return Err(SerializationError::ByteCountExceedsMaxSize);
        }

        let mut data = [0; NET_MAX_PAYLOAD_SIZE];
        for byte in data.iter_mut().take(byte_count as usize) {
            *byte = u8::deserialize(deserializer)?;
        }

        Ok(DataPayload{ byte_count: byte_count as u32, data })
    }
}

impl DataPayload {
    pub fn data(&self) -> &[u8] { &self.data[..self.byte_count as usize] }
}

type ChallengeData = [u8; NET_CHALLENGE_SIZE];

#[derive(Clone, Debug)]
pub enum MessagePayload {
    Heartbeat,

    ConnectionRequest(ClientId),
    ConnectionAccept,
    ConnectionReject(ConnectionRejectReason),
    DisconnectNotice(ClientId),

    ChallengeRequest(ChallengeData),
    ChallengeResponse(ClientId, ChallengeData),

    Data(DataPayload),
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ConnectionRejectReason {
    ServerFull,
    AlreadyConnected,
}

pub struct Heartbeat;
impl Heartbeat {
    pub fn build_message() -> Message {
        Message {
            protocol: PROTOCOL_ID,
            version: 1,
            payload: MessagePayload::Heartbeat,
        }
    }
}

pub struct ConnectionRequest;
impl ConnectionRequest {
    pub fn build_message(client_id: ClientId) -> Message {
        Message {
            protocol: PROTOCOL_ID,
            version: 1,
            payload: MessagePayload::ConnectionRequest(client_id),
        }
    }
}

pub struct ConnectionAccept;
impl ConnectionAccept {
    pub fn build_message() -> Message {
        Message {
            protocol: PROTOCOL_ID,
            version: 1,
            payload: MessagePayload::ConnectionAccept,
        }
    }
}

pub struct ConnectionReject;
impl ConnectionReject {
    pub fn build_message(reason: ConnectionRejectReason) -> Message {
        Message {
            protocol: PROTOCOL_ID,
            version: 1,
            payload: MessagePayload::ConnectionReject(reason),
        }
    }
}

pub struct DisconnectNotice;
impl DisconnectNotice {
    pub fn build_message(client_id: ClientId) -> Message {
        Message {
            protocol: PROTOCOL_ID,
            version: 1,
            payload: MessagePayload::DisconnectNotice(client_id),
        }
    }
}

pub struct ChallengeRequest;
impl ChallengeRequest {
    pub fn build_message(challenge_data: ChallengeData) -> Message {
        Message {
            protocol: PROTOCOL_ID,
            version: 1,
            payload: MessagePayload::ChallengeRequest(challenge_data),
        }
    }
}

pub struct ChallengeResponse;
impl ChallengeResponse {
    pub fn build_message(client_id: ClientId, challenge_data: ChallengeData) -> Message {
        Message {
            protocol: PROTOCOL_ID,
            version: 1,
            payload: MessagePayload::ChallengeResponse(client_id, challenge_data),
        }
    }
}

pub struct Data;
impl Data {
    pub fn build_message<S: Serialize>(data_payload: S) -> Result<Message, SerializationError> {
        let mut data = [0u32; NET_MAX_PAYLOAD_SIZE / 4];

        let mut serializer = Serializer::new(&mut data);
        data_payload.serialize(&mut serializer)?;
        let packet_size = serializer.finish()?;

        let byte_count = (packet_size * 4) as u32;
        if byte_count as usize >= NET_MAX_PAYLOAD_SIZE {
            return Err(SerializationError::ByteCountExceedsMaxSize);
        }

        let data = unsafe {
            std::mem::transmute::<
                [u32; NET_MAX_PAYLOAD_SIZE / 4],
                [u8; NET_MAX_PAYLOAD_SIZE]
            >(data)
        };

        Ok(Message {
            protocol: PROTOCOL_ID,
            version: 1,
            payload: MessagePayload::Data(
                DataPayload {
                    byte_count,
                    data,
                }
            ),
        })
    }

    pub fn build_message_raw(data: [u8; NET_MAX_PAYLOAD_SIZE], byte_count: usize) -> Message {
        Message {
            protocol: PROTOCOL_ID,
            version: 1,
            payload: MessagePayload::Data(
                DataPayload {
                    byte_count: byte_count as u32,
                    data,
                }
            ),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Message {
    pub protocol: u32,
    pub version: u8,
    pub payload: MessagePayload,
}

impl Serialize for Message {
    fn serialize(&self, serializer: &mut Serializer) -> Result<(), SerializationError> {
        self.protocol.serialize(serializer)?;
        self.version.serialize(serializer)?;

        match self.payload {
            MessagePayload::Heartbeat => serializer.serialize_u8(0x0),

            MessagePayload::ConnectionRequest(client_id) => {
                serializer.serialize_u8(0x1)?;
                client_id.serialize(serializer)?;
                Ok(())
            },
            MessagePayload::ConnectionAccept => serializer.serialize_u8(0x2),
            MessagePayload::ConnectionReject(reason) => {
                serializer.serialize_u8(0x3)?;

                let reason_u8 = match reason {
                    ConnectionRejectReason::ServerFull       => 0x0,
                    ConnectionRejectReason::AlreadyConnected => 0x1,
                };
                serializer.serialize_u8(reason_u8)?;
                Ok(())
            },
            MessagePayload::DisconnectNotice(client_id) => {
                serializer.serialize_u8(0x4)?;
                client_id.serialize(serializer)?;
                Ok(())
            },

            MessagePayload::ChallengeRequest(challenge_data) => {
                serializer.serialize_u8(0x5)?;
                challenge_data.serialize(serializer)?;
                Ok(())
            },
            MessagePayload::ChallengeResponse(client_id, challenge_data) => {
                serializer.serialize_u8(0x6)?;
                client_id.serialize(serializer)?;
                challenge_data.serialize(serializer)?;
                Ok(())
            },

            MessagePayload::Data(ref data_payload) => {
                serializer.serialize_u8(0x0a)?;
                data_payload.serialize(serializer)?;
                Ok(())
            },
        }
    }
}

impl Deserialize for Message {
    fn deserialize(deserializer: &mut Deserializer) -> Result<Self, SerializationError> {
        // header

        let protocol = u32::deserialize(deserializer)?;
        if protocol != PROTOCOL_ID { return Err(SerializationError::InvalidProtocol); }

        let version = u8::deserialize(deserializer)?;
        if version != 1 { return Err(SerializationError::InvalidVersion); }

        let payload = match deserializer.deserialize_u8()? {
            0x0 => MessagePayload::Heartbeat,

            0x1 => {
                let client_id = ClientId::deserialize(deserializer)?;
                MessagePayload::ConnectionRequest(client_id)
            },
            0x2 => MessagePayload::ConnectionAccept,
            0x3 => {
                let reason = match deserializer.deserialize_packed_u8::<0, 1>()? {
                    0x0 => ConnectionRejectReason::ServerFull,
                    0x1 => ConnectionRejectReason::AlreadyConnected,
                    _ => unreachable!(),
                };

                MessagePayload::ConnectionReject(reason)
            },
            0x4 => {
                let client_id = ClientId::deserialize(deserializer)?;
                MessagePayload::DisconnectNotice(client_id)
            },

            0x5 => {
                let challenge_data = ChallengeData::deserialize(deserializer)?;
                MessagePayload::ChallengeRequest(challenge_data)
            },
            0x6 => {
                let client_id = ClientId::deserialize(deserializer)?;
                let challenge_data = ChallengeData::deserialize(deserializer)?;
                MessagePayload::ChallengeResponse(client_id, challenge_data)
            }

            0x0a => {
                let data_payload = DataPayload::deserialize(deserializer)?;
                MessagePayload::Data(data_payload)
            }

            _ => return Err(SerializationError::ValueOutOfRange),
        };

        Ok(
            Self {
                protocol,
                version,
                payload,
            }
        )
    }
}

impl Message {
    // @Rename build_packet
    pub fn create_packet(&self) -> Result<([u8; NET_MAX_PACKET_SIZE], usize), SerializationError> {
        let mut data = [0u32; NET_MAX_PACKET_SIZE / 4];

        let mut serializer = Serializer::new(&mut data);
        self.serialize(&mut serializer)?;
        let packet_size = serializer.finish()?;

        let data = unsafe {
            std::mem::transmute::<[u32; NET_MAX_PACKET_SIZE / 4], [u8; NET_MAX_PACKET_SIZE]>(data)
        };

        Ok((data, packet_size * 4))
    }

    pub fn parse(packet: &[u8]) -> Result<Self, SerializationError> {
        // @Fix alignment issues?
        let data = unsafe {
            std::mem::transmute::<&[u8], &[u32]>(packet)
        };

        let mut deserializer = Deserializer::new(data);
        Message::deserialize(&mut deserializer)
    }
}

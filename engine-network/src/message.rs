//! Message types and traits for network communication.

use crate::error::{NetError, NetResult};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Message ID type
pub type MessageId = u64;

/// Message trait for network messages
pub trait NetMessage: Serialize + DeserializeOwned + Send + Sync + 'static {
    /// Get the message type ID (CRC32 hash of type name)
    fn type_id() -> u32 {
        let type_name = std::any::type_name::<Self>();
        let mut hasher = crc32fast::Hasher::new();
        hasher.update(type_name.as_bytes());
        hasher.finalize()
    }

    /// Serialize message to bytes
    fn encode(&self) -> NetResult<Vec<u8>> {
        bincode::serialize(self).map_err(|e| NetError::Serialization(e.to_string()))
    }

    /// Deserialize message from bytes
    fn decode(data: &[u8]) -> NetResult<Self> {
        // 限制反序列化大小以防止 DoS 攻击
        const MAX_MESSAGE_SIZE: usize = 10 * 1024 * 1024; // 10MB
        if data.len() > MAX_MESSAGE_SIZE {
            return Err(NetError::Deserialization("Message too large".to_string()));
        }
        bincode::deserialize(data).map_err(|e| NetError::Deserialization(e.to_string()))
    }
}

/// Serialization format enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SerializeFormat {
    /// MessagePack format (default)
    MessagePack,
    /// JSON format
    Json,
    /// Bincode format (fast binary)
    Bincode,
}

impl Default for SerializeFormat {
    fn default() -> Self {
        Self::Bincode
    }
}

/// Message wrapper for network transmission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Unique message ID
    pub id: MessageId,
    /// Message type ID
    pub type_id: u32,
    /// Timestamp when message was created
    pub timestamp: u64,
    /// Serialized payload
    pub payload: Vec<u8>,
}

impl Message {
    /// Create a new message
    pub fn new<M: NetMessage>(id: MessageId, message: &M) -> NetResult<Self> {
        let payload = message.encode()?;
        Ok(Self {
            id,
            type_id: M::type_id(),
            timestamp: current_timestamp(),
            payload,
        })
    }

    /// Create a new message with raw payload
    pub fn new_raw(id: MessageId, type_id: u32, payload: Vec<u8>) -> Self {
        Self {
            id,
            type_id,
            timestamp: current_timestamp(),
            payload,
        }
    }

    /// Deserialize the payload into a typed message
    pub fn decode<M: NetMessage>(&self) -> NetResult<M> {
        M::decode(&self.payload)
    }

    /// Serialize the message wrapper to bytes
    pub fn encode(&self) -> NetResult<Vec<u8>> {
        bincode::serialize(self).map_err(|e| NetError::Serialization(e.to_string()))
    }

    /// Deserialize message wrapper from bytes
    pub fn from_bytes(data: &[u8]) -> NetResult<Self> {
        // 限制反序列化大小以防止 DoS 攻击
        const MAX_MESSAGE_SIZE: usize = 10 * 1024 * 1024; // 10MB
        if data.len() > MAX_MESSAGE_SIZE {
            return Err(NetError::Deserialization("Message too large".to_string()));
        }
        bincode::deserialize(data).map_err(|e| NetError::Deserialization(e.to_string()))
    }

    /// Get message size
    pub fn size(&self) -> usize {
        std::mem::size_of::<MessageId>()
            + std::mem::size_of::<u32>()
            + std::mem::size_of::<u64>()
            + self.payload.len()
    }
}

/// Generate a unique message ID
pub fn generate_message_id() -> MessageId {
    let mut hasher = DefaultHasher::new();
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .hash(&mut hasher);
    std::thread::current().id().hash(&mut hasher);
    hasher.finish()
}

/// Get current timestamp in milliseconds
fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

/// Message builder for convenient message construction
pub struct MessageBuilder {
    id: Option<MessageId>,
    timestamp: Option<u64>,
}

impl MessageBuilder {
    /// Create a new message builder
    pub fn new() -> Self {
        Self {
            id: None,
            timestamp: None,
        }
    }

    /// Set custom message ID
    pub fn id(mut self, id: MessageId) -> Self {
        self.id = Some(id);
        self
    }

    /// Set custom timestamp
    pub fn timestamp(mut self, ts: u64) -> Self {
        self.timestamp = Some(ts);
        self
    }

    /// Build the message
    pub fn build<M: NetMessage>(self, message: &M) -> NetResult<Message> {
        let id = self.id.unwrap_or_else(generate_message_id);
        let payload = message.encode()?;
        Ok(Message {
            id,
            type_id: M::type_id(),
            timestamp: self.timestamp.unwrap_or_else(current_timestamp),
            payload,
        })
    }
}

impl Default for MessageBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Common network messages

/// Ping message for keepalive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingMessage {
    /// Client timestamp
    pub client_time: u64,
}

impl NetMessage for PingMessage {}

/// Pong message for keepalive response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PongMessage {
    /// Echoed client timestamp
    pub client_time: u64,
    /// Server timestamp
    pub server_time: u64,
}

impl NetMessage for PongMessage {}

/// Connection request message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectRequest {
    /// Client version
    pub version: String,
    /// Authentication token
    pub token: Option<String>,
    /// Client capabilities
    pub capabilities: Vec<String>,
}

impl NetMessage for ConnectRequest {}

/// Connection response message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectResponse {
    /// Assigned client ID
    pub client_id: u64,
    /// Connection accepted
    pub accepted: bool,
    /// Rejection reason if not accepted
    pub reason: Option<String>,
    /// Server capabilities
    pub capabilities: Vec<String>,
}

impl NetMessage for ConnectResponse {}

/// Disconnect notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisconnectMessage {
    /// Disconnect reason
    pub reason: String,
}

impl NetMessage for DisconnectMessage {}

/// Heartbeat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatMessage {
    /// Sequence number
    pub sequence: u32,
}

impl NetMessage for HeartbeatMessage {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct TestMessage {
        value: u32,
        text: String,
    }

    impl NetMessage for TestMessage {}

    #[test]
    fn test_message_creation() {
        let msg = TestMessage {
            value: 42,
            text: "hello".to_string(),
        };
        let message = Message::new(1, &msg).unwrap();

        assert_eq!(message.id, 1);
        assert_eq!(message.type_id, TestMessage::type_id());
        assert!(!message.payload.is_empty());
    }

    #[test]
    fn test_message_encode_decode() {
        let original = TestMessage {
            value: 42,
            text: "hello".to_string(),
        };
        let encoded = original.encode().unwrap();
        let decoded: TestMessage = TestMessage::decode(&encoded).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_message_builder() {
        let msg = TestMessage {
            value: 100,
            text: "test".to_string(),
        };
        let message = MessageBuilder::new().id(999).build(&msg).unwrap();

        assert_eq!(message.id, 999);
    }

    #[test]
    fn test_ping_pong_messages() {
        let ping = PingMessage { client_time: 1000 };
        let encoded = ping.encode().unwrap();
        let decoded: PingMessage = PingMessage::decode(&encoded).unwrap();
        assert_eq!(ping.client_time, decoded.client_time);

        let pong = PongMessage {
            client_time: 1000,
            server_time: 1005,
        };
        let encoded = pong.encode().unwrap();
        let decoded: PongMessage = PongMessage::decode(&encoded).unwrap();
        assert_eq!(pong.client_time, decoded.client_time);
        assert_eq!(pong.server_time, decoded.server_time);
    }
}

//! Packet handling and serialization.

use crate::error::{NetError, NetResult};
use serde::{Deserialize, Serialize};

/// Packet header containing metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketHeader {
    /// Message type ID (CRC32 hash)
    pub message_type: u32,
    /// Sequence number for ordering
    pub sequence: u32,
    /// Acknowledgment number
    pub ack: u32,
    /// Flags for packet properties
    pub flags: PacketFlags,
    /// Payload length. Stored as `u32` so it can represent the full
    /// `MAX_PACKET_SIZE` (1 MiB); a `u16` silently truncated any payload
    /// larger than 64 KiB, desynchronizing the header from the real length.
    pub payload_len: u32,
}

#[allow(clippy::derivable_impls)]
impl Default for PacketHeader {
    fn default() -> Self {
        Self {
            message_type: 0,
            sequence: 0,
            ack: 0,
            flags: PacketFlags::empty(),
            payload_len: 0,
        }
    }
}

bitflags::bitflags! {
    /// Packet flags for various properties
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct PacketFlags: u8 {
        /// Reliable delivery required
        const RELIABLE = 0x01;
        /// Ordered delivery required
        const ORDERED = 0x02;
        /// Compressed payload
        const COMPRESSED = 0x04;
        /// Encrypted payload
        const ENCRYPTED = 0x08;
        /// Acknowledgment packet
        const ACK = 0x10;
        /// Resend packet
        const RESEND = 0x20;
        /// Fragment packet
        const FRAGMENT = 0x40;
        /// End of fragmented sequence
        const FRAGMENT_END = 0x80;
    }
}

impl Default for PacketFlags {
    fn default() -> Self {
        Self::empty()
    }
}

/// Network packet containing header and payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packet {
    /// Packet header
    pub header: PacketHeader,
    /// Packet payload
    pub payload: Vec<u8>,
}

impl Packet {
    /// Create a new packet with the given message type and payload
    pub fn new(message_type: u32, payload: Vec<u8>) -> NetResult<Self> {
        let payload_len = payload.len();
        if payload_len > crate::MAX_PACKET_SIZE {
            return Err(NetError::PacketTooLarge(
                payload_len,
                crate::MAX_PACKET_SIZE,
            ));
        }

        Ok(Self {
            header: PacketHeader {
                message_type,
                payload_len: payload_len as u32,
                ..Default::default()
            },
            payload,
        })
    }

    /// Create a reliable packet
    pub fn reliable(mut self) -> Self {
        self.header.flags |= PacketFlags::RELIABLE;
        self
    }

    /// Create an ordered packet
    pub fn ordered(mut self) -> Self {
        self.header.flags |= PacketFlags::ORDERED;
        self
    }

    /// Set sequence number
    pub fn with_sequence(mut self, seq: u32) -> Self {
        self.header.sequence = seq;
        self
    }

    /// Set acknowledgment number
    pub fn with_ack(mut self, ack: u32) -> Self {
        self.header.ack = ack;
        self
    }

    /// Serialize packet to bytes
    pub fn serialize(&self) -> NetResult<Vec<u8>> {
        bincode::serialize(self).map_err(NetError::from)
    }

    /// Deserialize packet from bytes
    pub fn deserialize(data: &[u8]) -> NetResult<Self> {
        bincode::deserialize(data).map_err(|e| NetError::Deserialization(e.to_string()))
    }

    /// Check if packet is reliable
    pub fn is_reliable(&self) -> bool {
        self.header.flags.contains(PacketFlags::RELIABLE)
    }

    /// Check if packet is ordered
    pub fn is_ordered(&self) -> bool {
        self.header.flags.contains(PacketFlags::ORDERED)
    }

    /// Check if packet is compressed
    pub fn is_compressed(&self) -> bool {
        self.header.flags.contains(PacketFlags::COMPRESSED)
    }

    /// Check if packet is encrypted
    pub fn is_encrypted(&self) -> bool {
        self.header.flags.contains(PacketFlags::ENCRYPTED)
    }

    /// Check if this is an acknowledgment packet
    pub fn is_ack(&self) -> bool {
        self.header.flags.contains(PacketFlags::ACK)
    }

    /// Check if this is a fragment
    pub fn is_fragment(&self) -> bool {
        self.header.flags.contains(PacketFlags::FRAGMENT)
    }

    /// Get total packet size
    pub fn size(&self) -> usize {
        std::mem::size_of::<PacketHeader>() + self.payload.len()
    }
}

/// Packet builder for convenient packet construction
pub struct PacketBuilder {
    message_type: u32,
    payload: Vec<u8>,
    flags: PacketFlags,
    sequence: u32,
    ack: u32,
}

impl PacketBuilder {
    /// Create a new packet builder
    pub fn new(message_type: u32, payload: Vec<u8>) -> Self {
        Self {
            message_type,
            payload,
            flags: PacketFlags::empty(),
            sequence: 0,
            ack: 0,
        }
    }

    /// Make packet reliable
    pub fn reliable(mut self) -> Self {
        self.flags |= PacketFlags::RELIABLE;
        self
    }

    /// Make packet ordered
    pub fn ordered(mut self) -> Self {
        self.flags |= PacketFlags::ORDERED;
        self
    }

    /// Set sequence number
    pub fn sequence(mut self, seq: u32) -> Self {
        self.sequence = seq;
        self
    }

    /// Set acknowledgment number
    pub fn ack(mut self, ack: u32) -> Self {
        self.ack = ack;
        self
    }

    /// Build the packet
    pub fn build(self) -> NetResult<Packet> {
        let payload_len = self.payload.len();
        if payload_len > crate::MAX_PACKET_SIZE {
            return Err(NetError::PacketTooLarge(
                payload_len,
                crate::MAX_PACKET_SIZE,
            ));
        }

        Ok(Packet {
            header: PacketHeader {
                message_type: self.message_type,
                sequence: self.sequence,
                ack: self.ack,
                flags: self.flags,
                payload_len: payload_len as u32,
            },
            payload: self.payload,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_creation() {
        let payload = vec![1, 2, 3, 4, 5];
        let packet = Packet::new(42, payload.clone()).unwrap();
        assert_eq!(packet.header.message_type, 42);
        assert_eq!(packet.payload, payload);
        assert!(!packet.is_reliable());
    }

    #[test]
    fn test_packet_reliable() {
        let packet = Packet::new(42, vec![1, 2, 3]).unwrap().reliable();
        assert!(packet.is_reliable());
    }

    #[test]
    fn test_packet_ordered() {
        let packet = Packet::new(42, vec![1, 2, 3]).unwrap().ordered();
        assert!(packet.is_ordered());
    }

    #[test]
    fn test_packet_serialize_deserialize() {
        let original = Packet::new(42, vec![1, 2, 3, 4, 5])
            .unwrap()
            .reliable()
            .ordered()
            .with_sequence(10)
            .with_ack(5);

        let serialized = original.serialize().unwrap();
        let deserialized = Packet::deserialize(&serialized).unwrap();

        assert_eq!(
            original.header.message_type,
            deserialized.header.message_type
        );
        assert_eq!(original.header.sequence, deserialized.header.sequence);
        assert_eq!(original.header.ack, deserialized.header.ack);
        assert_eq!(original.payload, deserialized.payload);
        assert_eq!(original.header.flags, deserialized.header.flags);
    }

    #[test]
    fn test_packet_builder() {
        let packet = PacketBuilder::new(100, vec![1, 2, 3])
            .reliable()
            .ordered()
            .sequence(5)
            .ack(3)
            .build()
            .unwrap();

        assert!(packet.is_reliable());
        assert!(packet.is_ordered());
        assert_eq!(packet.header.sequence, 5);
        assert_eq!(packet.header.ack, 3);
    }

    /// Regression: `payload_len` was stored as `u16`, so any payload larger
    /// than 64 KiB (but still under `MAX_PACKET_SIZE` = 1 MiB) was silently
    /// truncated in the header while the real payload kept its full length,
    /// desynchronizing the wire frame from the actual data.
    #[test]
    fn test_packet_large_payload_len_not_truncated() {
        let size: usize = 70_000; // > u16::MAX, < MAX_PACKET_SIZE
        let payload = vec![0xABu8; size];

        let packet = Packet::new(42, payload.clone()).unwrap();
        assert_eq!(packet.header.payload_len as usize, size);
        assert_eq!(packet.payload.len(), size);

        // Round-trip through bincode to ensure the u32 field serializes
        // without truncation as well.
        let serialized = packet.serialize().unwrap();
        let deserialized = Packet::deserialize(&serialized).unwrap();
        assert_eq!(deserialized.header.payload_len as usize, size);
        assert_eq!(deserialized.payload.len(), size);

        // Builder path must behave identically.
        let built = PacketBuilder::new(7, payload.clone()).build().unwrap();
        assert_eq!(built.header.payload_len as usize, size);
    }
}

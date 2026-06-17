//! Network, hot update, and plugin system for the game engine.
//!
//! This crate provides comprehensive networking capabilities including:
//! - TCP/UDP/WebSocket transport layers
//! - Packet and message handling
//! - Plugin system for extensibility
//! - Network client and server implementations
//!
//! # Modules
//!
//! - [`error`] - Error types for network operations
//! - [`packet`] - Packet handling and serialization
//! - [`message`] - Message types and traits
//! - [`channel`] - Channel abstraction for network communication
//! - [`transport`] - Transport layer implementations (TCP, UDP, WebSocket)
//! - [`manager`] - Network manager for coordinating connections
//! - [`plugin`] - Plugin system for extensibility
//! - [`stats`] - Network statistics

pub mod channel;
pub mod error;
pub mod manager;
pub mod message;
pub mod packet;
pub mod plugin;
pub mod stats;
pub mod transport;

pub use channel::{Channel, ChannelConfig, ChannelId, NetChannel};
pub use error::{NetError, NetResult};
pub use manager::{NetworkClient, NetworkManager, NetworkServer};
pub use message::{Message, MessageId, NetMessage};
pub use packet::Packet;
pub use plugin::{Plugin, PluginContext, PluginManager, PluginState};
pub use stats::NetStats;
pub use transport::{NetworkTransport, TransportBuilder, TransportType, TlsConfig, TlsTcpTransport};

/// Default timeout for RPC operations (5 seconds)
pub const DEFAULT_RPC_TIMEOUT_MS: u64 = 5_000;

/// Default UDP sliding window size
pub const UDP_WINDOW_SIZE: usize = 32;

/// Default UDP retransmit timeout in milliseconds
pub const UDP_RETRANSMIT_TIMEOUT_MS: u64 = 200;

/// Default matchmaking timeout in milliseconds
pub const MATCHMAKING_TIMEOUT_MS: u64 = 60_000;

/// Maximum packet size (1 MB)
pub const MAX_PACKET_SIZE: usize = 1_048_576;

/// Default buffer size for network operations
pub const DEFAULT_BUFFER_SIZE: usize = 8192;

/// Network role enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum NetRole {
    /// Pure server mode
    Server,
    /// Pure client mode
    Client,
    /// Listen server (host)
    ListenServer,
    /// Standalone (offline) mode
    #[default]
    Standalone,
}

/// Client identifier type
pub type ClientId = u64;

/// Room identifier type
pub type RoomId = u64;

/// Queue identifier type
pub type QueueId = u64;

/// Engine version
pub const ENGINE_VERSION: &str = "0.1.0";

//! Error types for network operations.

use std::fmt;

/// Result type alias for network operations
pub type NetResult<T> = std::result::Result<T, NetError>;

/// Network error types
#[derive(Debug)]
pub enum NetError {
    /// Connection failed
    ConnectionFailed(String),
    /// Connection closed
    ConnectionClosed,
    /// Connection timeout
    Timeout,
    /// Address in use
    AddressInUse(String),
    /// Address not available
    AddressNotAvailable(String),
    /// Invalid address format
    InvalidAddress(String),
    /// Packet too large
    PacketTooLarge(usize, usize),
    /// Serialization error
    Serialization(String),
    /// Deserialization error
    Deserialization(String),
    /// Channel closed
    ChannelClosed,
    /// Channel full
    ChannelFull,
    /// Invalid channel ID
    InvalidChannelId(u64),
    /// Transport error
    Transport(String),
    /// IO error
    Io(std::io::Error),
    /// Plugin error
    Plugin(String),
    /// Not implemented
    NotImplemented,
    /// Invalid state
    InvalidState(String),
    /// Other error
    Other(String),
}

impl fmt::Display for NetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConnectionFailed(addr) => write!(f, "Failed to connect to {}", addr),
            Self::ConnectionClosed => write!(f, "Connection closed"),
            Self::Timeout => write!(f, "Operation timed out"),
            Self::AddressInUse(addr) => write!(f, "Address already in use: {}", addr),
            Self::AddressNotAvailable(addr) => write!(f, "Address not available: {}", addr),
            Self::InvalidAddress(addr) => write!(f, "Invalid address: {}", addr),
            Self::PacketTooLarge(size, max) => {
                write!(f, "Packet too large: {} bytes (max: {})", size, max)
            }
            Self::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            Self::Deserialization(msg) => write!(f, "Deserialization error: {}", msg),
            Self::ChannelClosed => write!(f, "Channel closed"),
            Self::ChannelFull => write!(f, "Channel full"),
            Self::InvalidChannelId(id) => write!(f, "Invalid channel ID: {}", id),
            Self::Transport(msg) => write!(f, "Transport error: {}", msg),
            Self::Io(err) => write!(f, "IO error: {}", err),
            Self::Plugin(msg) => write!(f, "Plugin error: {}", msg),
            Self::NotImplemented => write!(f, "Not implemented"),
            Self::InvalidState(state) => write!(f, "Invalid state: {}", state),
            Self::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for NetError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for NetError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<bincode::Error> for NetError {
    fn from(err: bincode::Error) -> Self {
        Self::Serialization(err.to_string())
    }
}
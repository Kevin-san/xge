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

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_connection_failed_display() {
        let err = NetError::ConnectionFailed("localhost:8080".to_string());
        let s = err.to_string();
        assert!(s.contains("Failed to connect"));
        assert!(s.contains("localhost:8080"));
    }

    #[test]
    fn test_connection_closed_display() {
        let err = NetError::ConnectionClosed;
        assert_eq!(err.to_string(), "Connection closed");
    }

    #[test]
    fn test_timeout_display() {
        let err = NetError::Timeout;
        assert_eq!(err.to_string(), "Operation timed out");
    }

    #[test]
    fn test_address_in_use_display() {
        let err = NetError::AddressInUse("127.0.0.1:8080".to_string());
        let s = err.to_string();
        assert!(s.contains("Address already in use"));
        assert!(s.contains("127.0.0.1:8080"));
    }

    #[test]
    fn test_address_not_available_display() {
        let err = NetError::AddressNotAvailable("invalid:port".to_string());
        let s = err.to_string();
        assert!(s.contains("Address not available"));
    }

    #[test]
    fn test_invalid_address_display() {
        let err = NetError::InvalidAddress("bad_address".to_string());
        let s = err.to_string();
        assert!(s.contains("Invalid address"));
    }

    #[test]
    fn test_packet_too_large_display() {
        let err = NetError::PacketTooLarge(10000, 5000);
        let s = err.to_string();
        assert!(s.contains("Packet too large"));
        assert!(s.contains("10000"));
        assert!(s.contains("5000"));
    }

    #[test]
    fn test_serialization_error_display() {
        let err = NetError::Serialization("encode failed".to_string());
        let s = err.to_string();
        assert!(s.contains("Serialization error"));
        assert!(s.contains("encode failed"));
    }

    #[test]
    fn test_deserialization_error_display() {
        let err = NetError::Deserialization("decode failed".to_string());
        let s = err.to_string();
        assert!(s.contains("Deserialization error"));
    }

    #[test]
    fn test_channel_closed_display() {
        let err = NetError::ChannelClosed;
        assert_eq!(err.to_string(), "Channel closed");
    }

    #[test]
    fn test_channel_full_display() {
        let err = NetError::ChannelFull;
        assert_eq!(err.to_string(), "Channel full");
    }

    #[test]
    fn test_invalid_channel_id_display() {
        let err = NetError::InvalidChannelId(12345);
        let s = err.to_string();
        assert!(s.contains("Invalid channel ID"));
        assert!(s.contains("12345"));
    }

    #[test]
    fn test_transport_error_display() {
        let err = NetError::Transport("socket error".to_string());
        let s = err.to_string();
        assert!(s.contains("Transport error"));
    }

    #[test]
    fn test_io_error_from() {
        let io_err = std::io::Error::new(std::io::ErrorKind::WouldBlock, "would block");
        let net_err: NetError = io_err.into();
        assert!(matches!(net_err, NetError::Io(_)));
    }

    #[test]
    fn test_io_error_source() {
        let io_err = std::io::Error::new(std::io::ErrorKind::WouldBlock, "would block");
        let net_err = NetError::Io(io_err);
        let source = net_err.source();
        assert!(source.is_some());
    }

    #[test]
    fn test_non_io_error_source() {
        let net_err = NetError::ConnectionClosed;
        let source = net_err.source();
        assert!(source.is_none());
    }

    #[test]
    fn test_plugin_error_display() {
        let err = NetError::Plugin("plugin failed".to_string());
        let s = err.to_string();
        assert!(s.contains("Plugin error"));
    }

    #[test]
    fn test_not_implemented_display() {
        let err = NetError::NotImplemented;
        assert_eq!(err.to_string(), "Not implemented");
    }

    #[test]
    fn test_invalid_state_display() {
        let err = NetError::InvalidState("disconnected".to_string());
        let s = err.to_string();
        assert!(s.contains("Invalid state"));
        assert!(s.contains("disconnected"));
    }

    #[test]
    fn test_other_error_display() {
        let err = NetError::Other("unknown error".to_string());
        let s = err.to_string();
        assert!(s.contains("Error:"));
        assert!(s.contains("unknown error"));
    }

    #[test]
    fn test_net_result_ok() {
        let result: NetResult<i32> = Ok(42);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_net_result_err() {
        let result: NetResult<i32> = Err(NetError::Timeout);
        assert!(result.is_err());
    }

    #[test]
    fn test_error_debug() {
        let err = NetError::ConnectionFailed("test".to_string());
        let s = format!("{:?}", err);
        assert!(s.contains("ConnectionFailed"));
    }
}
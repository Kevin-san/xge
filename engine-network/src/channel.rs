//! Channel abstraction for network communication.

use crate::error::{NetError, NetResult};
use crate::stats::NetStats;
use parking_lot::Mutex;
use std::collections::VecDeque;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

/// Channel ID type
pub type ChannelId = u64;

/// Channel configuration
#[derive(Debug, Clone, Copy)]
pub struct ChannelConfig {
    /// Maximum send queue size
    pub max_send_queue: usize,
    /// Maximum receive queue size
    pub max_recv_queue: usize,
    /// Send buffer size
    pub send_buffer_size: usize,
    /// Receive buffer size
    pub recv_buffer_size: usize,
    /// Enable reliable delivery
    pub reliable: bool,
    /// Enable ordered delivery
    pub ordered: bool,
    /// Timeout in milliseconds
    pub timeout_ms: u64,
}

impl Default for ChannelConfig {
    fn default() -> Self {
        Self {
            max_send_queue: 1024,
            max_recv_queue: 1024,
            send_buffer_size: crate::DEFAULT_BUFFER_SIZE,
            recv_buffer_size: crate::DEFAULT_BUFFER_SIZE,
            reliable: false,
            ordered: false,
            timeout_ms: crate::DEFAULT_RPC_TIMEOUT_MS,
        }
    }
}

/// Network channel trait
pub trait NetChannel: Send + Sync {
    /// Send data through the channel
    fn send(&self, data: &[u8]) -> NetResult<()>;

    /// Receive data from the channel
    fn recv(&self) -> NetResult<Option<Vec<u8>>>;

    /// Check if channel is connected
    fn is_connected(&self) -> bool;

    /// Get peer address
    fn peer_addr(&self) -> Option<SocketAddr>;

    /// Get local address
    fn local_addr(&self) -> Option<SocketAddr>;

    /// Close the channel
    fn close(&self) -> NetResult<()>;

    /// Get channel statistics
    fn stats(&self) -> NetStats;
}

/// In-memory channel for testing and local communication
pub struct MemoryChannel {
    id: ChannelId,
    config: ChannelConfig,
    send_queue: Mutex<VecDeque<Vec<u8>>>,
    recv_queue: Mutex<VecDeque<Vec<u8>>>,
    connected: AtomicBool,
    peer_addr: Option<SocketAddr>,
    local_addr: Option<SocketAddr>,
    bytes_in: AtomicU64,
    bytes_out: AtomicU64,
    msg_in: AtomicU64,
    msg_out: AtomicU64,
}

impl MemoryChannel {
    /// Create a new memory channel
    pub fn new(id: ChannelId, config: ChannelConfig) -> Self {
        Self {
            id,
            config,
            send_queue: Mutex::new(VecDeque::with_capacity(config.max_send_queue)),
            recv_queue: Mutex::new(VecDeque::with_capacity(config.max_recv_queue)),
            connected: AtomicBool::new(true),
            peer_addr: None,
            local_addr: None,
            bytes_in: AtomicU64::new(0),
            bytes_out: AtomicU64::new(0),
            msg_in: AtomicU64::new(0),
            msg_out: AtomicU64::new(0),
        }
    }

    /// Create a pair of connected memory channels
    pub fn create_pair(config: ChannelConfig) -> (Self, Self) {
        let channel_a = Self::new(1, config.clone());
        let channel_b = Self::new(2, config);

        // Link the channels
        channel_a.connected.store(true, Ordering::SeqCst);
        channel_b.connected.store(true, Ordering::SeqCst);

        (channel_a, channel_b)
    }

    /// Set peer address
    pub fn set_peer_addr(&mut self, addr: SocketAddr) {
        self.peer_addr = Some(addr);
    }

    /// Set local address
    pub fn set_local_addr(&mut self, addr: SocketAddr) {
        self.local_addr = Some(addr);
    }

    /// Transfer data from send queue to another channel's receive queue
    pub fn transfer_to(&self, other: &MemoryChannel) -> NetResult<()> {
        let mut send_queue = self.send_queue.lock();
        let mut recv_queue = other.recv_queue.lock();

        while let Some(data) = send_queue.pop_front() {
            if recv_queue.len() >= other.config.max_recv_queue {
                return Err(NetError::ChannelFull);
            }
            recv_queue.push_back(data);
        }

        Ok(())
    }

    /// Get channel ID
    pub fn id(&self) -> ChannelId {
        self.id
    }
}

impl NetChannel for MemoryChannel {
    fn send(&self, data: &[u8]) -> NetResult<()> {
        if !self.connected.load(Ordering::SeqCst) {
            return Err(NetError::ConnectionClosed);
        }

        let mut queue = self.send_queue.lock();
        if queue.len() >= self.config.max_send_queue {
            return Err(NetError::ChannelFull);
        }

        queue.push_back(data.to_vec());
        self.bytes_out.fetch_add(data.len() as u64, Ordering::SeqCst);
        self.msg_out.fetch_add(1, Ordering::SeqCst);

        Ok(())
    }

    fn recv(&self) -> NetResult<Option<Vec<u8>>> {
        if !self.connected.load(Ordering::SeqCst) {
            return Err(NetError::ConnectionClosed);
        }

        let mut queue = self.recv_queue.lock();
        if let Some(data) = queue.pop_front() {
            self.bytes_in.fetch_add(data.len() as u64, Ordering::SeqCst);
            self.msg_in.fetch_add(1, Ordering::SeqCst);
            Ok(Some(data))
        } else {
            Ok(None)
        }
    }

    fn is_connected(&self) -> bool {
        self.connected.load(Ordering::SeqCst)
    }

    fn peer_addr(&self) -> Option<SocketAddr> {
        self.peer_addr
    }

    fn local_addr(&self) -> Option<SocketAddr> {
        self.local_addr
    }

    fn close(&self) -> NetResult<()> {
        self.connected.store(false, Ordering::SeqCst);
        Ok(())
    }

    fn stats(&self) -> NetStats {
        NetStats {
            bytes_in: self.bytes_in.load(Ordering::SeqCst),
            bytes_out: self.bytes_out.load(Ordering::SeqCst),
            msg_in: self.msg_in.load(Ordering::SeqCst),
            msg_out: self.msg_out.load(Ordering::SeqCst),
            rtt_ms: 0, // Memory channels have zero RTT
        }
    }
}

/// Channel wrapper with additional functionality
pub struct Channel {
    inner: Arc<dyn NetChannel>,
    config: ChannelConfig,
}

impl Channel {
    /// Create a new channel wrapper
    pub fn new(inner: Arc<dyn NetChannel>, config: ChannelConfig) -> Self {
        Self { inner, config }
    }

    /// Get the inner channel reference
    pub fn inner(&self) -> &Arc<dyn NetChannel> {
        &self.inner
    }

    /// Get channel configuration
    pub fn config(&self) -> &ChannelConfig {
        &self.config
    }

    /// Send data with timeout
    pub fn send_timeout(&self, data: &[u8], timeout_ms: u64) -> NetResult<()> {
        // For now, just call send directly
        // In a full implementation, this would use async with timeout
        self.inner.send(data)
    }

    /// Receive data with timeout
    pub fn recv_timeout(&self, timeout_ms: u64) -> NetResult<Option<Vec<u8>>> {
        // For now, just call recv directly
        self.inner.recv()
    }
}

impl NetChannel for Channel {
    fn send(&self, data: &[u8]) -> NetResult<()> {
        self.inner.send(data)
    }

    fn recv(&self) -> NetResult<Option<Vec<u8>>> {
        self.inner.recv()
    }

    fn is_connected(&self) -> bool {
        self.inner.is_connected()
    }

    fn peer_addr(&self) -> Option<SocketAddr> {
        self.inner.peer_addr()
    }

    fn local_addr(&self) -> Option<SocketAddr> {
        self.inner.local_addr()
    }

    fn close(&self) -> NetResult<()> {
        self.inner.close()
    }

    fn stats(&self) -> NetStats {
        self.inner.stats()
    }
}

/// Reliability level for UDP channels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Reliability {
    /// Unreliable transmission
    Unreliable,
    /// Unreliable but sequenced
    UnreliableSequenced,
    /// Reliable transmission
    #[default]
    Reliable,
    /// Reliable and ordered
    ReliableOrdered,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_channel_send_recv() {
        let config = ChannelConfig::default();
        let channel = MemoryChannel::new(1, config);

        let data = vec![1, 2, 3, 4, 5];
        channel.send(&data).unwrap();

        // Verify stats were updated
        let stats = channel.stats();
        assert_eq!(stats.bytes_out, 5);
        assert_eq!(stats.msg_out, 1);

        // Manually add data to recv queue for testing
        {
            let mut recv_queue = channel.recv_queue.lock();
            recv_queue.push_back(data.clone());
        }

        let received = channel.recv().unwrap();
        assert_eq!(received, Some(data));

        // Verify stats were updated for receive
        let stats = channel.stats();
        assert_eq!(stats.bytes_in, 5);
        assert_eq!(stats.msg_in, 1);
    }

    #[test]
    fn test_memory_channel_pair() {
        let config = ChannelConfig::default();
        let (channel_a, channel_b) = MemoryChannel::create_pair(config);

        assert!(channel_a.is_connected());
        assert!(channel_b.is_connected());

        channel_a.send(&vec![1, 2, 3]).unwrap();
        channel_a.transfer_to(&channel_b).unwrap();

        let received = channel_b.recv().unwrap();
        assert_eq!(received, Some(vec![1, 2, 3]));
    }

    #[test]
    fn test_memory_channel_stats() {
        let config = ChannelConfig::default();
        let channel = MemoryChannel::new(1, config);

        channel.send(&vec![1, 2, 3]).unwrap();
        channel.send(&vec![4, 5, 6]).unwrap();

        let stats = channel.stats();
        assert_eq!(stats.bytes_out, 6);
        assert_eq!(stats.msg_out, 2);
    }

    #[test]
    fn test_memory_channel_close() {
        let config = ChannelConfig::default();
        let channel = MemoryChannel::new(1, config);

        assert!(channel.is_connected());
        channel.close().unwrap();
        assert!(!channel.is_connected());

        let result = channel.send(&vec![1, 2, 3]);
        assert!(result.is_err());
    }

    #[test]
    fn test_channel_config_default() {
        let config = ChannelConfig::default();
        assert_eq!(config.max_send_queue, 1024);
        assert_eq!(config.max_recv_queue, 1024);
        assert!(!config.reliable);
        assert!(!config.ordered);
    }
}
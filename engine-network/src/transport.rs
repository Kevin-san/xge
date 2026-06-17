//! Transport layer implementations (TCP, UDP, WebSocket).

use crate::channel::{ChannelConfig, NetChannel, Reliability};
use crate::error::{NetError, NetResult};
use crate::stats::NetStats;
use parking_lot::Mutex;
use std::collections::VecDeque;
use std::net::{SocketAddr, TcpStream, UdpSocket};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

/// Transport type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TransportType {
    /// TCP transport (reliable, ordered)
    #[default]
    Tcp,
    /// UDP transport (unreliable, can be configured)
    Udp,
    /// WebSocket transport
    WebSocket,
    /// In-memory transport (for testing)
    Memory,
}

/// Network transport trait
pub trait NetworkTransport: Send + Sync {
    /// Get transport type
    fn transport_type(&self) -> TransportType;

    /// Create a channel for communication
    fn create_channel(&self, config: ChannelConfig) -> NetResult<Arc<dyn NetChannel>>;

    /// Check if transport is active
    fn is_active(&self) -> bool;

    /// Get local address
    fn local_addr(&self) -> Option<SocketAddr>;

    /// Shutdown the transport
    fn shutdown(&self) -> NetResult<()>;
}

/// TCP transport implementation
pub struct TcpTransport {
    socket: Mutex<Option<TcpStream>>,
    local_addr: Option<SocketAddr>,
    peer_addr: Option<SocketAddr>,
    connected: AtomicBool,
    send_queue: Mutex<VecDeque<Vec<u8>>>,
    recv_queue: Mutex<VecDeque<Vec<u8>>>,
    bytes_in: AtomicU64,
    bytes_out: AtomicU64,
    msg_in: AtomicU64,
    msg_out: AtomicU64,
}

impl TcpTransport {
    /// Create a TCP transport by connecting to an address
    pub fn connect(addr: SocketAddr) -> NetResult<Self> {
        let socket = TcpStream::connect_timeout(
            &addr,
            Duration::from_millis(crate::DEFAULT_RPC_TIMEOUT_MS),
        )?;
        let local_addr = socket.local_addr()?;
        let peer_addr = socket.peer_addr()?;

        Ok(Self {
            socket: Mutex::new(Some(socket)),
            local_addr: Some(local_addr),
            peer_addr: Some(peer_addr),
            connected: AtomicBool::new(true),
            send_queue: Mutex::new(VecDeque::new()),
            recv_queue: Mutex::new(VecDeque::new()),
            bytes_in: AtomicU64::new(0),
            bytes_out: AtomicU64::new(0),
            msg_in: AtomicU64::new(0),
            msg_out: AtomicU64::new(0),
        })
    }

    /// Create a TCP transport from an existing stream
    pub fn from_stream(stream: TcpStream) -> NetResult<Self> {
        let local_addr = stream.local_addr()?;
        let peer_addr = stream.peer_addr()?;

        Ok(Self {
            socket: Mutex::new(Some(stream)),
            local_addr: Some(local_addr),
            peer_addr: Some(peer_addr),
            connected: AtomicBool::new(true),
            send_queue: Mutex::new(VecDeque::new()),
            recv_queue: Mutex::new(VecDeque::new()),
            bytes_in: AtomicU64::new(0),
            bytes_out: AtomicU64::new(0),
            msg_in: AtomicU64::new(0),
            msg_out: AtomicU64::new(0),
        })
    }

    /// Get peer address
    pub fn peer_addr(&self) -> Option<SocketAddr> {
        self.peer_addr
    }
}

impl NetChannel for TcpTransport {
    fn send(&self, data: &[u8]) -> NetResult<()> {
        if !self.connected.load(Ordering::SeqCst) {
            return Err(NetError::ConnectionClosed);
        }

        let mut socket = self.socket.lock();
        if let Some(stream) = socket.as_mut() {
            use std::io::Write;
            stream.write_all(data)?;
            stream.flush()?;
            self.bytes_out
                .fetch_add(data.len() as u64, Ordering::SeqCst);
            self.msg_out.fetch_add(1, Ordering::SeqCst);
            Ok(())
        } else {
            Err(NetError::ConnectionClosed)
        }
    }

    fn recv(&self) -> NetResult<Option<Vec<u8>>> {
        if !self.connected.load(Ordering::SeqCst) {
            return Err(NetError::ConnectionClosed);
        }

        // For now, use the recv_queue for testing
        // In a full implementation, this would read from the socket
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
        let mut socket = self.socket.lock();
        if let Some(stream) = socket.take() {
            // Close the stream by dropping it
            drop(stream);
        }
        Ok(())
    }

    fn stats(&self) -> NetStats {
        NetStats {
            bytes_in: self.bytes_in.load(Ordering::SeqCst),
            bytes_out: self.bytes_out.load(Ordering::SeqCst),
            msg_in: self.msg_in.load(Ordering::SeqCst),
            msg_out: self.msg_out.load(Ordering::SeqCst),
            rtt_ms: 0,
        }
    }
}

impl NetworkTransport for TcpTransport {
    fn transport_type(&self) -> TransportType {
        TransportType::Tcp
    }

    fn create_channel(&self, config: ChannelConfig) -> NetResult<Arc<dyn NetChannel>> {
        Ok(Arc::new(Self {
            socket: Mutex::new(None),
            local_addr: self.local_addr,
            peer_addr: self.peer_addr,
            connected: AtomicBool::new(self.connected.load(Ordering::SeqCst)),
            send_queue: Mutex::new(VecDeque::with_capacity(config.max_send_queue)),
            recv_queue: Mutex::new(VecDeque::with_capacity(config.max_recv_queue)),
            bytes_in: AtomicU64::new(0),
            bytes_out: AtomicU64::new(0),
            msg_in: AtomicU64::new(0),
            msg_out: AtomicU64::new(0),
        }))
    }

    fn is_active(&self) -> bool {
        self.connected.load(Ordering::SeqCst)
    }

    fn local_addr(&self) -> Option<SocketAddr> {
        self.local_addr
    }

    fn shutdown(&self) -> NetResult<()> {
        self.close()
    }
}

/// UDP transport implementation
pub struct UdpTransport {
    socket: Mutex<Option<UdpSocket>>,
    local_addr: Option<SocketAddr>,
    connected_addr: Mutex<Option<SocketAddr>>,
    send_queue: Mutex<VecDeque<(SocketAddr, Vec<u8>, Reliability)>>,
    recv_queue: Mutex<VecDeque<(SocketAddr, Vec<u8>)>>,
    connected: AtomicBool,
    bytes_in: AtomicU64,
    bytes_out: AtomicU64,
    msg_in: AtomicU64,
    msg_out: AtomicU64,
}

impl UdpTransport {
    /// Bind a UDP socket to an address
    pub fn bind(addr: SocketAddr) -> NetResult<Self> {
        let socket = UdpSocket::bind(addr)?;
        let local_addr = socket.local_addr()?;

        Ok(Self {
            socket: Mutex::new(Some(socket)),
            local_addr: Some(local_addr),
            connected_addr: Mutex::new(None),
            send_queue: Mutex::new(VecDeque::new()),
            recv_queue: Mutex::new(VecDeque::new()),
            connected: AtomicBool::new(true),
            bytes_in: AtomicU64::new(0),
            bytes_out: AtomicU64::new(0),
            msg_in: AtomicU64::new(0),
            msg_out: AtomicU64::new(0),
        })
    }

    /// Connect to a remote address (for send/recv semantics)
    pub fn connect(addr: SocketAddr) -> NetResult<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.connect(addr)?;
        let local_addr = socket.local_addr()?;

        Ok(Self {
            socket: Mutex::new(Some(socket)),
            local_addr: Some(local_addr),
            connected_addr: Mutex::new(Some(addr)),
            send_queue: Mutex::new(VecDeque::new()),
            recv_queue: Mutex::new(VecDeque::new()),
            connected: AtomicBool::new(true),
            bytes_in: AtomicU64::new(0),
            bytes_out: AtomicU64::new(0),
            msg_in: AtomicU64::new(0),
            msg_out: AtomicU64::new(0),
        })
    }

    /// Send data to a specific address with reliability setting
    pub fn send_to(
        &self,
        addr: SocketAddr,
        data: &[u8],
        reliability: Reliability,
    ) -> NetResult<()> {
        if !self.connected.load(Ordering::SeqCst) {
            return Err(NetError::ConnectionClosed);
        }

        let mut socket = self.socket.lock();
        if let Some(sock) = socket.as_ref() {
            sock.send_to(data, addr)?;
            self.bytes_out
                .fetch_add(data.len() as u64, Ordering::SeqCst);
            self.msg_out.fetch_add(1, Ordering::SeqCst);
            Ok(())
        } else {
            Err(NetError::ConnectionClosed)
        }
    }

    /// Receive data from any address
    pub fn recv_from(&self) -> NetResult<Option<(SocketAddr, Vec<u8>)>> {
        if !self.connected.load(Ordering::SeqCst) {
            return Err(NetError::ConnectionClosed);
        }

        let mut queue = self.recv_queue.lock();
        if let Some((addr, data)) = queue.pop_front() {
            self.bytes_in.fetch_add(data.len() as u64, Ordering::SeqCst);
            self.msg_in.fetch_add(1, Ordering::SeqCst);
            Ok(Some((addr, data)))
        } else {
            Ok(None)
        }
    }
}

impl NetChannel for UdpTransport {
    fn send(&self, data: &[u8]) -> NetResult<()> {
        let addr = *self.connected_addr.lock();
        if let Some(addr) = addr {
            self.send_to(addr, data, Reliability::default())
        } else {
            Err(NetError::InvalidState("UDP not connected".to_string()))
        }
    }

    fn recv(&self) -> NetResult<Option<Vec<u8>>> {
        self.recv_from().map(|opt| opt.map(|(_, data)| data))
    }

    fn is_connected(&self) -> bool {
        self.connected.load(Ordering::SeqCst)
    }

    fn peer_addr(&self) -> Option<SocketAddr> {
        *self.connected_addr.lock()
    }

    fn local_addr(&self) -> Option<SocketAddr> {
        self.local_addr
    }

    fn close(&self) -> NetResult<()> {
        self.connected.store(false, Ordering::SeqCst);
        let mut socket = self.socket.lock();
        socket.take();
        Ok(())
    }

    fn stats(&self) -> NetStats {
        NetStats {
            bytes_in: self.bytes_in.load(Ordering::SeqCst),
            bytes_out: self.bytes_out.load(Ordering::SeqCst),
            msg_in: self.msg_in.load(Ordering::SeqCst),
            msg_out: self.msg_out.load(Ordering::SeqCst),
            rtt_ms: 0,
        }
    }
}

impl NetworkTransport for UdpTransport {
    fn transport_type(&self) -> TransportType {
        TransportType::Udp
    }

    fn create_channel(&self, config: ChannelConfig) -> NetResult<Arc<dyn NetChannel>> {
        Ok(Arc::new(Self {
            socket: Mutex::new(None),
            local_addr: self.local_addr,
            connected_addr: Mutex::new(None),
            send_queue: Mutex::new(VecDeque::with_capacity(config.max_send_queue)),
            recv_queue: Mutex::new(VecDeque::with_capacity(config.max_recv_queue)),
            connected: AtomicBool::new(self.connected.load(Ordering::SeqCst)),
            bytes_in: AtomicU64::new(0),
            bytes_out: AtomicU64::new(0),
            msg_in: AtomicU64::new(0),
            msg_out: AtomicU64::new(0),
        }))
    }

    fn is_active(&self) -> bool {
        self.connected.load(Ordering::SeqCst)
    }

    fn local_addr(&self) -> Option<SocketAddr> {
        self.local_addr
    }

    fn shutdown(&self) -> NetResult<()> {
        self.close()
    }
}

/// WebSocket transport (stub implementation)
/// Full implementation would require async runtime and websocket library
pub struct WebSocketTransport {
    url: String,
    local_addr: Option<SocketAddr>,
    peer_addr: Option<SocketAddr>,
    connected: AtomicBool,
    send_queue: Mutex<VecDeque<Vec<u8>>>,
    recv_queue: Mutex<VecDeque<Vec<u8>>>,
    bytes_in: AtomicU64,
    bytes_out: AtomicU64,
    msg_in: AtomicU64,
    msg_out: AtomicU64,
}

impl WebSocketTransport {
    /// Create a WebSocket transport (stub)
    pub fn connect(url: &str) -> NetResult<Self> {
        // Stub implementation - would need tokio-tungstenite for real implementation
        Ok(Self {
            url: url.to_string(),
            local_addr: None,
            peer_addr: None,
            connected: AtomicBool::new(true),
            send_queue: Mutex::new(VecDeque::new()),
            recv_queue: Mutex::new(VecDeque::new()),
            bytes_in: AtomicU64::new(0),
            bytes_out: AtomicU64::new(0),
            msg_in: AtomicU64::new(0),
            msg_out: AtomicU64::new(0),
        })
    }

    /// Send text message
    pub fn send_text(&self, text: &str) -> NetResult<()> {
        self.send(text.as_bytes())
    }

    /// Send binary message
    pub fn send_binary(&self, data: &[u8]) -> NetResult<()> {
        self.send(data)
    }
}

impl NetChannel for WebSocketTransport {
    fn send(&self, data: &[u8]) -> NetResult<()> {
        if !self.connected.load(Ordering::SeqCst) {
            return Err(NetError::ConnectionClosed);
        }

        let mut queue = self.send_queue.lock();
        queue.push_back(data.to_vec());
        self.bytes_out
            .fetch_add(data.len() as u64, Ordering::SeqCst);
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
            rtt_ms: 0,
        }
    }
}

impl NetworkTransport for WebSocketTransport {
    fn transport_type(&self) -> TransportType {
        TransportType::WebSocket
    }

    fn create_channel(&self, config: ChannelConfig) -> NetResult<Arc<dyn NetChannel>> {
        Ok(Arc::new(Self {
            url: self.url.clone(),
            local_addr: self.local_addr,
            peer_addr: self.peer_addr,
            connected: AtomicBool::new(self.connected.load(Ordering::SeqCst)),
            send_queue: Mutex::new(VecDeque::with_capacity(config.max_send_queue)),
            recv_queue: Mutex::new(VecDeque::with_capacity(config.max_recv_queue)),
            bytes_in: AtomicU64::new(0),
            bytes_out: AtomicU64::new(0),
            msg_in: AtomicU64::new(0),
            msg_out: AtomicU64::new(0),
        }))
    }

    fn is_active(&self) -> bool {
        self.connected.load(Ordering::SeqCst)
    }

    fn local_addr(&self) -> Option<SocketAddr> {
        self.local_addr
    }

    fn shutdown(&self) -> NetResult<()> {
        self.close()
    }
}

/// Transport builder for creating transports with configuration
pub struct TransportBuilder {
    transport_type: TransportType,
    address: Option<SocketAddr>,
    url: Option<String>,
    config: ChannelConfig,
}

impl TransportBuilder {
    /// Create a new transport builder
    pub fn new(transport_type: TransportType) -> Self {
        Self {
            transport_type,
            address: None,
            url: None,
            config: ChannelConfig::default(),
        }
    }

    /// Set address for TCP/UDP
    pub fn address(mut self, addr: SocketAddr) -> Self {
        self.address = Some(addr);
        self
    }

    /// Set URL for WebSocket
    pub fn url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    /// Set channel configuration
    pub fn config(mut self, config: ChannelConfig) -> Self {
        self.config = config;
        self
    }

    /// Build the transport
    pub fn build(self) -> NetResult<Arc<dyn NetworkTransport>> {
        match self.transport_type {
            TransportType::Tcp => {
                let addr = self.address.ok_or_else(|| {
                    NetError::InvalidState("Address required for TCP transport".to_string())
                })?;
                Ok(Arc::new(TcpTransport::connect(addr)?))
            }
            TransportType::Udp => {
                let addr = self.address.ok_or_else(|| {
                    NetError::InvalidState("Address required for UDP transport".to_string())
                })?;
                Ok(Arc::new(UdpTransport::bind(addr)?))
            }
            TransportType::WebSocket => {
                let url = self.url.ok_or_else(|| {
                    NetError::InvalidState("URL required for WebSocket transport".to_string())
                })?;
                Ok(Arc::new(WebSocketTransport::connect(&url)?))
            }
            TransportType::Memory => Err(NetError::NotImplemented),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transport_type() {
        assert_eq!(TransportType::default(), TransportType::Tcp);
    }

    #[test]
    fn test_udp_transport_bind() {
        let addr = "127.0.0.1:0".parse().unwrap();
        let transport = UdpTransport::bind(addr).unwrap();
        assert!(transport.is_active());
        // Use NetChannel::local_addr explicitly
        assert!(crate::channel::NetChannel::local_addr(&transport).is_some());
    }

    #[test]
    fn test_websocket_transport_stub() {
        let transport = WebSocketTransport::connect("ws://localhost:8080").unwrap();
        assert!(transport.is_connected());
        transport.send(&vec![1, 2, 3]).unwrap();
        let stats = transport.stats();
        assert_eq!(stats.bytes_out, 3);
    }

    #[test]
    fn test_transport_builder() {
        let addr = "127.0.0.1:0".parse().unwrap();
        let transport = TransportBuilder::new(TransportType::Udp)
            .address(addr)
            .build()
            .unwrap();

        assert_eq!(transport.transport_type(), TransportType::Udp);
        assert!(transport.is_active());
    }
}

//! Transport layer implementations (TCP, UDP, WebSocket, TLS).
//!
//! TLS transport uses rustls for encrypted TCP connections.
//! Note: UDP does not support native TLS; DTLS would require a separate
//! datagram-oriented TLS implementation at the application layer.

use crate::channel::{ChannelConfig, NetChannel, Reliability};
use crate::error::{NetError, NetResult};
use crate::stats::NetStats;
use parking_lot::Mutex;
use rustls::{ClientConfig, RootCertStore};
use std::collections::VecDeque;
use std::fs;
use std::io::{Read, Write as IoWrite};
use std::net::{SocketAddr, TcpStream, UdpSocket};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

/// Helper to capture TLS write output into a byte vector.
struct TlsWriteCapture<'a> {
    buf: &'a mut Vec<u8>,
}

impl<'a> IoWrite for TlsWriteCapture<'a> {
    fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
        self.buf.extend_from_slice(data);
        Ok(data.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// Transport type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TransportType {
    /// TCP transport (reliable, ordered)
    #[default]
    Tcp,
    /// UDP transport (unreliable, can be configured)
    Udp,
    /// TLS transport (encrypted TCP)
    Tls,
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
        _reliability: Reliability,
    ) -> NetResult<()> {
        if !self.connected.load(Ordering::SeqCst) {
            return Err(NetError::ConnectionClosed);
        }

        let socket = self.socket.lock();
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

// ============================================================================
// TLS Transport (rustls 0.23)
// ============================================================================

/// TLS (encrypted TCP) transport configuration.
#[derive(Debug, Clone)]
pub struct TlsConfig {
    /// Domain name for certificate verification (SNI).
    domain: String,
    /// Path to a custom CA certificate (PEM format).
    ca_file: Option<String>,
    /// Path to a client certificate (PEM format).
    client_cert_file: Option<String>,
    /// Path to a client private key (PEM format).
    client_key_file: Option<String>,
}

impl TlsConfig {
    /// Create a new TLS config for a domain.
    pub fn new(domain: String) -> Self {
        Self {
            domain,
            ca_file: None,
            client_cert_file: None,
            client_key_file: None,
        }
    }

    /// Add a custom CA certificate file (PEM format).
    pub fn with_ca_file(mut self, path: String) -> Self {
        self.ca_file = Some(path);
        self
    }

    /// Add a client certificate for mTLS.
    pub fn with_client_cert(mut self, cert_path: String, key_path: String) -> Self {
        self.client_cert_file = Some(cert_path);
        self.client_key_file = Some(key_path);
        self
    }

    /// Get domain name.
    pub fn domain(&self) -> &str {
        &self.domain
    }

    /// Build a rustls ClientConfig from this configuration.
    /// Uses Mozilla's webpki-roots for CA verification by default.
    pub fn build_client_config(&self) -> NetResult<ClientConfig> {
        let mut root_store = RootCertStore::empty();
        // Add Mozilla's root CAs
        root_store.extend(
            webpki_roots::TLS_SERVER_ROOTS
                .iter()
                .cloned(),
        );

        // Load custom CA if provided
        if let Some(ca_path) = &self.ca_file {
            let ca_pem = fs::read(ca_path)
                .map_err(|e| NetError::Tls(format!("Failed to read CA file: {}", e)))?;
            for item in rustls_pemfile::certs(&mut ca_pem.as_slice()) {
                let der = item.map_err(|e| NetError::Tls(format!("Failed to parse CA cert: {}", e)))?;
                root_store.add(der)
                    .map_err(|e| NetError::Tls(format!("Failed to add CA cert: {}", e)))?;
            }
        }

        // Honor an mTLS client certificate/key when configured. Previously the
        // configured paths were silently ignored and `with_no_client_auth()` was
        // always used, downgrading deployments that rely on mutual TLS to
        // single-sided authentication.
        let config = match (&self.client_cert_file, &self.client_key_file) {
            (Some(cert_path), Some(key_path)) => {
                let cert_pem = fs::read(cert_path)
                    .map_err(|e| NetError::Tls(format!("Failed to read client cert: {}", e)))?;
                let certs: Vec<rustls::pki_types::CertificateDer<'static>> =
                    rustls_pemfile::certs(&mut cert_pem.as_slice())
                        .collect::<Result<_, _>>()
                        .map_err(|e| {
                            NetError::Tls(format!("Failed to parse client cert: {}", e))
                        })?;

                let key_pem = fs::read(key_path)
                    .map_err(|e| NetError::Tls(format!("Failed to read client key: {}", e)))?;
                let key = rustls_pemfile::private_key(&mut key_pem.as_slice())
                    .map_err(|e| NetError::Tls(format!("Failed to parse client key: {}", e)))?
                    .ok_or_else(|| {
                        NetError::Tls("No client private key found in key file".to_string())
                    })?;

                if certs.is_empty() {
                    return Err(NetError::Tls(
                        "No client certificate found in cert file".to_string(),
                    ));
                }
                ClientConfig::builder()
                    .with_root_certificates(root_store)
                    .with_client_auth_cert(certs, key)
                    .map_err(|e| {
                        NetError::Tls(format!("Failed to build mTLS client config: {}", e))
                    })?
            }
            _ => ClientConfig::builder()
                .with_root_certificates(root_store)
                .with_no_client_auth(),
        };

        Ok(config)
    }
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self::new("localhost".to_string())
    }
}

/// TLS-over-TCP transport using rustls.
/// Encrypts all data using TLS. Note: UDP does not support native TLS (DTLS
/// would require a separate datagram-oriented TLS implementation).
///
/// This transport uses the rustls `ClientConnection` with a synchronous wrapper
/// for blocking I/O. The underlying socket is set to blocking mode.
pub struct TlsTcpTransport {
    /// TLS client connection.
    conn: Arc<Mutex<rustls::client::ClientConnection>>,
    /// Raw underlying TCP stream.
    stream: Arc<Mutex<TcpStream>>,
    local_addr: Option<SocketAddr>,
    peer_addr: Option<SocketAddr>,
    connected: AtomicBool,
    send_queue: Mutex<VecDeque<Vec<u8>>>,
    #[allow(dead_code)]
    recv_queue: Mutex<VecDeque<Vec<u8>>>,
    bytes_in: AtomicU64,
    bytes_out: AtomicU64,
    msg_in: AtomicU64,
    msg_out: AtomicU64,
}

impl TlsTcpTransport {
    /// Connect to a TLS server at the given address.
    pub fn connect(addr: SocketAddr, config: TlsConfig) -> NetResult<Self> {
        let tcp = TcpStream::connect_timeout(
            &addr,
            Duration::from_millis(crate::DEFAULT_RPC_TIMEOUT_MS),
        )?;
        tcp.set_nonblocking(false)?;
        let local_addr = tcp.local_addr()?;
        let peer_addr = tcp.peer_addr()?;

        let client_config = config.build_client_config()?;
        let domain = rustls::pki_types::ServerName::try_from(config.domain.as_str())
            .map_err(|e| NetError::Tls(format!("Invalid domain: {}", e)))?
            .to_owned();

        let conn = rustls::client::ClientConnection::new(
            Arc::new(client_config),
            domain,
        )
        .map_err(|e| NetError::Tls(format!("Failed to create TLS session: {}", e)))?;

        let conn_arc = Arc::new(Mutex::new(conn));
        let stream_arc = Arc::new(Mutex::new(tcp));

        let transport = Self {
            conn: Arc::clone(&conn_arc),
            stream: Arc::clone(&stream_arc),
            local_addr: Some(local_addr),
            peer_addr: Some(peer_addr),
            connected: AtomicBool::new(true),
            send_queue: Mutex::new(VecDeque::new()),
            recv_queue: Mutex::new(VecDeque::new()),
            bytes_in: AtomicU64::new(0),
            bytes_out: AtomicU64::new(0),
            msg_in: AtomicU64::new(0),
            msg_out: AtomicU64::new(0),
        };

        // Complete TLS handshake
        transport.perform_handshake()?;

        Ok(transport)
    }

    /// Perform TLS handshake.
    fn perform_handshake(&self) -> NetResult<()> {
        let mut conn = self.conn.lock();
        let mut stream = self.stream.lock();

        loop {
            // Process any pending packets
            if let Err(e) = conn.process_new_packets() {
                return Err(NetError::Tls(format!("TLS handshake error: {}", e)));
            }

            if !conn.is_handshaking() {
                break;
            }

            // Write any pending TLS output to socket
            if conn.wants_write() {
                let mut out = Vec::new();
                {
                    let mut capturer = TlsWriteCapture { buf: &mut out };
                    conn.write_tls(&mut capturer)?;
                }
                if !out.is_empty() {
                    stream.write_all(&out)?;
                    stream.flush()?;
                }
            }

            // Read from socket into TLS connection
            if conn.wants_read() {
                let mut read_buf = [0u8; 8192];
                match stream.read(&mut read_buf) {
                    Ok(0) => {
                        // Peer closed the TCP connection mid-handshake. Breaking
                        // out and returning Ok would let the caller believe a
                        // secure TLS session (and thus certificate verification)
                        // had completed when it never did.
                        return Err(NetError::Tls(
                            "Connection closed by peer during TLS handshake".to_string(),
                        ));
                    }
                    Ok(n) => {
                        conn.read_tls(&mut &read_buf[..n])?;
                    }
                    Err(e) => {
                        if e.kind() == std::io::ErrorKind::WouldBlock {
                            continue;
                        }
                        return Err(NetError::Tls(format!("Socket read error: {}", e)));
                    }
                }
            }
        }
        Ok(())
    }

    /// Read pending TLS data from the socket and process it.
    fn do_io(&self) -> NetResult<()> {
        let mut conn = self.conn.lock();
        let mut stream = self.stream.lock();

        // Read from socket into TLS connection
        if conn.wants_read() {
            let mut buf = [0u8; 16384];
            match stream.read(&mut buf) {
                Ok(0) => {
                    // Peer closed the connection. Mark the transport as
                    // disconnected so is_connected()/recv reflect reality
                    // instead of silently reporting "no data yet" forever.
                    self.connected.store(false, Ordering::SeqCst);
                }
                Ok(n) => {
                    conn.read_tls(&mut &buf[..n])?;
                }
                Err(e) => {
                    if e.kind() != std::io::ErrorKind::WouldBlock {
                        return Err(NetError::Tls(format!("Socket read error: {}", e)));
                    }
                }
            }
        }

        // Process any new packets
        conn.process_new_packets()
            .map_err(|e| NetError::Tls(format!("TLS process error: {}", e)))?;

        // Write any pending TLS data to socket
        if conn.wants_write() {
            let mut out = Vec::new();
            {
                let mut capturer = TlsWriteCapture { buf: &mut out };
                conn.write_tls(&mut capturer)?;
            }
            if !out.is_empty() {
                stream.write_all(&out)?;
                stream.flush()?;
            }
        }

        Ok(())
    }
}

impl NetChannel for TlsTcpTransport {
    fn send(&self, data: &[u8]) -> NetResult<()> {
        if !self.connected.load(Ordering::SeqCst) {
            return Err(NetError::ConnectionClosed);
        }

        {
            let mut queue = self.send_queue.lock();
            queue.push_back(data.to_vec());
        }

        // Write all queued data through TLS
        let mut queue = self.send_queue.lock();
        while let Some(chunk) = queue.pop_front() {
            let mut conn = self.conn.lock();
            let mut stream = self.stream.lock();

            // Write to TLS connection's write buffer
            conn.writer().write_all(&chunk)
                .map_err(|e| NetError::Tls(format!("TLS write error: {}", e)))?;
            self.bytes_out.fetch_add(chunk.len() as u64, Ordering::SeqCst);
            self.msg_out.fetch_add(1, Ordering::SeqCst);

            // Write pending TLS data to socket
            let mut out = Vec::new();
            {
                let mut capturer = TlsWriteCapture { buf: &mut out };
                conn.write_tls(&mut capturer)?;
            }
            if !out.is_empty() {
                stream.write_all(&out)?;
                stream.flush()?;
            }
        }

        Ok(())
    }

    fn recv(&self) -> NetResult<Option<Vec<u8>>> {
        if !self.connected.load(Ordering::SeqCst) {
            return Err(NetError::ConnectionClosed);
        }

        // Try to read more data
        let _ = self.do_io();

        // Read from TLS connection
        let mut buf = Vec::new();
        {
            let mut conn = self.conn.lock();
            let mut reader = conn.reader();
            match reader.read_to_end(&mut buf) {
                Ok(0) => {}
                Ok(n) => {
                    self.bytes_in.fetch_add(n as u64, Ordering::SeqCst);
                    self.msg_in.fetch_add(1, Ordering::SeqCst);
                }
                Err(e) => {
                    if e.kind() != std::io::ErrorKind::WouldBlock {
                        return Err(NetError::Tls(format!("TLS read error: {}", e)));
                    }
                }
            }
        }

        if buf.is_empty() {
            Ok(None)
        } else {
            Ok(Some(buf))
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
        let mut conn = self.conn.lock();
        conn.send_close_notify();
        let mut stream = self.stream.lock();
        let mut out = Vec::new();
        {
            let mut capturer = TlsWriteCapture { buf: &mut out };
            let _ = conn.write_tls(&mut capturer);
        }
        if !out.is_empty() {
            let _ = stream.write_all(&out);
            let _ = stream.flush();
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

impl NetworkTransport for TlsTcpTransport {
    fn transport_type(&self) -> TransportType {
        TransportType::Tls
    }

    fn create_channel(&self, config: ChannelConfig) -> NetResult<Arc<dyn NetChannel>> {
        Ok(Arc::new(Self {
            conn: Arc::clone(&self.conn),
            stream: Arc::clone(&self.stream),
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
    tls_config: Option<TlsConfig>,
}

impl TransportBuilder {
    /// Create a new transport builder
    pub fn new(transport_type: TransportType) -> Self {
        Self {
            transport_type,
            address: None,
            url: None,
            config: ChannelConfig::default(),
            tls_config: None,
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

    /// Set TLS configuration
    pub fn tls_config(mut self, tls_config: TlsConfig) -> Self {
        self.tls_config = Some(tls_config);
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
            TransportType::Tls => {
                let addr = self.address.ok_or_else(|| {
                    NetError::InvalidState("Address required for TLS transport".to_string())
                })?;
                let tls_cfg = self.tls_config.unwrap_or_default();
                Ok(Arc::new(TlsTcpTransport::connect(addr, tls_cfg)?))
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
        transport.send(&[1, 2, 3]).unwrap();
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

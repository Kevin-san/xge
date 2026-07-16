//! Network manager for coordinating connections.

use crate::channel::{ChannelConfig, ChannelId, MemoryChannel, NetChannel};
use crate::error::{NetError, NetResult};
use crate::message::{ConnectRequest, DisconnectMessage, Message, NetMessage};
use crate::stats::{AtomicNetStats, NetStats};
use crate::{ClientId, NetRole};
use parking_lot::Mutex;
use serde_json;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

/// Network manager for coordinating network operations
pub struct NetworkManager {
    role: NetRole,
    channels: Mutex<HashMap<ChannelId, Arc<dyn NetChannel>>>,
    stats: AtomicNetStats,
    running: AtomicBool,
    next_channel_id: AtomicU64,
    config: NetworkConfig,
}

/// Network configuration
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    /// Maximum number of connections
    pub max_connections: usize,
    /// Default channel configuration
    pub default_channel_config: ChannelConfig,
    /// Timeout for connections
    pub connection_timeout_ms: u64,
    /// Enable statistics tracking
    pub enable_stats: bool,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            max_connections: 100,
            default_channel_config: ChannelConfig::default(),
            connection_timeout_ms: crate::DEFAULT_RPC_TIMEOUT_MS,
            enable_stats: true,
        }
    }
}

impl NetworkManager {
    /// Create a new network manager
    pub fn new(role: NetRole, config: NetworkConfig) -> Self {
        Self {
            role,
            channels: Mutex::new(HashMap::new()),
            stats: AtomicNetStats::new(),
            running: AtomicBool::new(true),
            next_channel_id: AtomicU64::new(1),
            config,
        }
    }

    /// Create a new network manager with default config
    pub fn with_role(role: NetRole) -> Self {
        Self::new(role, NetworkConfig::default())
    }

    /// Get network role
    pub fn role(&self) -> NetRole {
        self.role
    }

    /// Check if manager is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Get network statistics
    pub fn stats(&self) -> NetStats {
        self.stats.snapshot()
    }

    /// Add a channel to the manager
    pub fn add_channel(&self, channel: Arc<dyn NetChannel>) -> ChannelId {
        let id = self.next_channel_id.fetch_add(1, Ordering::SeqCst);
        let mut channels = self.channels.lock();
        channels.insert(id, channel);
        id
    }

    /// Remove a channel from the manager
    pub fn remove_channel(&self, id: ChannelId) -> NetResult<Arc<dyn NetChannel>> {
        let mut channels = self.channels.lock();
        channels.remove(&id).ok_or(NetError::InvalidChannelId(id))
    }

    /// Get a channel by ID
    pub fn get_channel(&self, id: ChannelId) -> NetResult<Arc<dyn NetChannel>> {
        let channels = self.channels.lock();
        channels
            .get(&id)
            .cloned()
            .ok_or(NetError::InvalidChannelId(id))
    }

    /// Get all active channel IDs
    pub fn channel_ids(&self) -> Vec<ChannelId> {
        let channels = self.channels.lock();
        channels.keys().copied().collect()
    }

    /// Get number of active channels
    pub fn channel_count(&self) -> usize {
        let channels = self.channels.lock();
        channels.len()
    }

    /// Send a message through a specific channel
    pub fn send(&self, channel_id: ChannelId, data: &[u8]) -> NetResult<()> {
        let channel = self.get_channel(channel_id)?;
        channel.send(data)?;
        self.stats.record_out(data.len() as u64);
        Ok(())
    }

    /// Receive a message from a specific channel
    pub fn recv(&self, channel_id: ChannelId) -> NetResult<Option<Vec<u8>>> {
        let channel = self.get_channel(channel_id)?;
        let data = channel.recv()?;
        if let Some(d) = &data {
            self.stats.record_in(d.len() as u64);
        }
        Ok(data)
    }

    /// Broadcast a message to all channels
    pub fn broadcast(&self, data: &[u8]) -> NetResult<()> {
        let channels = self.channels.lock();
        for channel in channels.values() {
            channel.send(data)?;
        }
        self.stats.record_out((data.len() * channels.len()) as u64);
        Ok(())
    }

    /// Create a memory channel pair for testing
    pub fn create_memory_channel_pair(&self) -> NetResult<(ChannelId, ChannelId)> {
        let config = self.config.default_channel_config;
        let (channel_a, channel_b) = MemoryChannel::create_pair(config);

        let id_a = self.add_channel(Arc::new(channel_a));
        let id_b = self.add_channel(Arc::new(channel_b));

        Ok((id_a, id_b))
    }

    /// Shutdown the network manager
    pub fn shutdown(&self) -> NetResult<()> {
        self.running.store(false, Ordering::SeqCst);

        let mut channels = self.channels.lock();
        for channel in channels.values() {
            channel.close()?;
        }
        channels.clear();

        Ok(())
    }
}

/// Network client implementation
pub struct NetworkClient {
    manager: NetworkManager,
    client_id: AtomicU64,
    connected: AtomicBool,
    server_addr: Mutex<Option<SocketAddr>>,
}

impl NetworkClient {
    /// Create a new network client
    pub fn new(config: NetworkConfig) -> Self {
        Self {
            manager: NetworkManager::new(NetRole::Client, config),
            client_id: AtomicU64::new(0),
            connected: AtomicBool::new(false),
            server_addr: Mutex::new(None),
        }
    }

    /// Get assigned client ID
    pub fn client_id(&self) -> ClientId {
        self.client_id.load(Ordering::SeqCst)
    }

    /// Check if client is connected
    pub fn is_connected(&self) -> bool {
        self.connected.load(Ordering::SeqCst)
    }

    /// Get server address
    pub fn server_addr(&self) -> Option<SocketAddr> {
        *self.server_addr.lock()
    }

    /// Connect to a server
    pub fn connect(&self, addr: SocketAddr) -> NetResult<()> {
        *self.server_addr.lock() = Some(addr);
        self.connected.store(true, Ordering::SeqCst);
        Ok(())
    }

    /// Connect with authentication
    pub fn connect_with_auth(&self, addr: SocketAddr, token: Option<String>) -> NetResult<()> {
        // Create connection request
        let _request = ConnectRequest {
            version: crate::ENGINE_VERSION.to_string(),
            token,
            capabilities: vec!["basic".to_string()],
        };

        // In a full implementation, this would send the request and wait for response
        self.connect(addr)?;

        // Simulate receiving a client ID
        self.client_id.store(1, Ordering::SeqCst);

        Ok(())
    }

    /// Disconnect from server
    pub fn disconnect(&self) -> NetResult<()> {
        if !self.connected.load(Ordering::SeqCst) {
            return Ok(());
        }

        let _disconnect_msg = DisconnectMessage {
            reason: "Client disconnecting".to_string(),
        };

        // In a full implementation, this would send the disconnect message
        self.connected.store(false, Ordering::SeqCst);
        self.client_id.store(0, Ordering::SeqCst);
        *self.server_addr.lock() = None;

        Ok(())
    }

    /// Get reference to network manager
    pub fn manager(&self) -> &NetworkManager {
        &self.manager
    }

    /// Send a message to server
    pub fn send_message<M: NetMessage>(&self, channel_id: ChannelId, msg: &M) -> NetResult<()> {
        let message = Message::new(crate::message::generate_message_id(), msg)?;
        let data = message.encode()?;
        self.manager.send(channel_id, &data)
    }

    /// Receive a message from server
    pub fn recv_message(&self, channel_id: ChannelId) -> NetResult<Option<Message>> {
        let data = self.manager.recv(channel_id)?;
        if let Some(d) = data {
            // Parse as message
            let msg = serde_json::from_slice::<Message>(&d)
                .map_err(|e| NetError::Deserialization(e.to_string()))?;
            Ok(Some(msg))
        } else {
            Ok(None)
        }
    }
}

/// Network server implementation
pub struct NetworkServer {
    manager: NetworkManager,
    clients: Mutex<HashMap<ClientId, ClientInfo>>,
    next_client_id: AtomicU64,
    listening: AtomicBool,
    bind_addr: Mutex<Option<SocketAddr>>,
}

/// Client information
#[derive(Debug, Clone)]
pub struct ClientInfo {
    /// Client ID
    pub id: ClientId,
    /// Client address
    pub addr: SocketAddr,
    /// Channel ID for this client
    pub channel_id: ChannelId,
    /// Connection time
    pub connected_at: u64,
    /// Client capabilities
    pub capabilities: Vec<String>,
}

impl NetworkServer {
    /// Create a new network server
    pub fn new(config: NetworkConfig) -> Self {
        Self {
            manager: NetworkManager::new(NetRole::Server, config),
            clients: Mutex::new(HashMap::new()),
            next_client_id: AtomicU64::new(1),
            listening: AtomicBool::new(false),
            bind_addr: Mutex::new(None),
        }
    }

    /// Bind server to address
    pub fn bind(&self, addr: SocketAddr) -> NetResult<()> {
        *self.bind_addr.lock() = Some(addr);
        self.listening.store(true, Ordering::SeqCst);
        Ok(())
    }

    /// Check if server is listening
    pub fn is_listening(&self) -> bool {
        self.listening.load(Ordering::SeqCst)
    }

    /// Get bind address
    pub fn bind_addr(&self) -> Option<SocketAddr> {
        *self.bind_addr.lock()
    }

    /// Accept a new client connection
    pub fn accept_client(&self, addr: SocketAddr, channel_id: ChannelId) -> NetResult<ClientId> {
        let client_id = self.next_client_id.fetch_add(1, Ordering::SeqCst);

        let client_info = ClientInfo {
            id: client_id,
            addr,
            channel_id,
            connected_at: current_timestamp(),
            capabilities: vec!["basic".to_string()],
        };

        let mut clients = self.clients.lock();
        clients.insert(client_id, client_info);

        Ok(client_id)
    }

    /// Disconnect a client
    pub fn disconnect_client(&self, client_id: ClientId, _reason: String) -> NetResult<()> {
        let mut clients = self.clients.lock();
        if let Some(info) = clients.remove(&client_id) {
            // Close the channel
            if let Ok(channel) = self.manager.get_channel(info.channel_id) {
                channel.close()?;
            }
        }
        Ok(())
    }

    /// Get client information
    pub fn get_client(&self, client_id: ClientId) -> Option<ClientInfo> {
        let clients = self.clients.lock();
        clients.get(&client_id).cloned()
    }

    /// Get all connected client IDs
    pub fn client_ids(&self) -> Vec<ClientId> {
        let clients = self.clients.lock();
        clients.keys().copied().collect()
    }

    /// Get number of connected clients
    pub fn client_count(&self) -> usize {
        let clients = self.clients.lock();
        clients.len()
    }

    /// Send message to a specific client
    pub fn send_to_client<M: NetMessage>(&self, client_id: ClientId, msg: &M) -> NetResult<()> {
        let client = self
            .get_client(client_id)
            .ok_or(NetError::InvalidState("Client not found".to_string()))?;
        let message = Message::new(crate::message::generate_message_id(), msg)?;
        let data = message.encode()?;
        self.manager.send(client.channel_id, &data)
    }

    /// Broadcast message to all clients
    pub fn broadcast<M: NetMessage>(&self, msg: &M) -> NetResult<()> {
        let message = Message::new(crate::message::generate_message_id(), msg)?;
        let data = message.encode()?;
        self.manager.broadcast(&data)
    }

    /// Get reference to network manager
    pub fn manager(&self) -> &NetworkManager {
        &self.manager
    }

    /// Shutdown the server
    pub fn shutdown(&self) -> NetResult<()> {
        self.listening.store(false, Ordering::SeqCst);

        // Disconnect all clients
        let clients = self.clients.lock();
        for client_id in clients.keys() {
            self.disconnect_client(*client_id, "Server shutdown".to_string())?;
        }

        self.manager.shutdown()?;
        Ok(())
    }
}

/// Get current timestamp in milliseconds
fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_manager_creation() {
        let manager = NetworkManager::with_role(NetRole::Server);
        assert_eq!(manager.role(), NetRole::Server);
        assert!(manager.is_running());
        assert_eq!(manager.channel_count(), 0);
    }

    #[test]
    fn test_network_manager_memory_channels() {
        let manager = NetworkManager::with_role(NetRole::Client);
        let (id_a, id_b) = manager.create_memory_channel_pair().unwrap();

        assert_eq!(manager.channel_count(), 2);

        // Send data through channel A
        manager.send(id_a, &[1, 2, 3]).unwrap();

        // For memory channel testing, we need to manually transfer data
        // Get the underlying MemoryChannel objects and use transfer_to
        let channel_a = manager.get_channel(id_a).unwrap();
        let _channel_b = manager.get_channel(id_b).unwrap();

        // Downcast to MemoryChannel for testing purposes
        // Note: In real usage, this would be handled by the network loop
        // For testing, we simulate by directly putting data in recv queue
        {
            // Get data from send queue of channel_a
            let _send_data = channel_a.recv().unwrap();
        }

        // Simplified test: just verify send works
        let stats = channel_a.stats();
        assert_eq!(stats.bytes_out, 3);
        assert_eq!(stats.msg_out, 1);
    }

    #[test]
    fn test_network_client() {
        let client = NetworkClient::new(NetworkConfig::default());
        assert_eq!(client.client_id(), 0);
        assert!(!client.is_connected());

        let addr = "127.0.0.1:8080".parse().unwrap();
        client.connect(addr).unwrap();
        assert!(client.is_connected());
        assert_eq!(client.server_addr(), Some(addr));

        client.disconnect().unwrap();
        assert!(!client.is_connected());
    }

    #[test]
    fn test_network_server() {
        let server = NetworkServer::new(NetworkConfig::default());
        assert!(!server.is_listening());

        let addr = "127.0.0.1:8080".parse().unwrap();
        server.bind(addr).unwrap();
        assert!(server.is_listening());
        assert_eq!(server.bind_addr(), Some(addr));

        // Accept a client
        let client_addr = "127.0.0.1:12345".parse().unwrap();
        let channel_id = server
            .manager()
            .add_channel(Arc::new(MemoryChannel::new(1, ChannelConfig::default())));
        let client_id = server.accept_client(client_addr, channel_id).unwrap();

        assert_eq!(server.client_count(), 1);
        assert!(server.get_client(client_id).is_some());

        server
            .disconnect_client(client_id, "Test disconnect".to_string())
            .unwrap();
        assert_eq!(server.client_count(), 0);
    }

    #[test]
    fn test_network_config() {
        let config = NetworkConfig::default();
        assert_eq!(config.max_connections, 100);
        assert!(config.enable_stats);
    }
}

//! # Network Manager
//!
//! Manages connections to GameVerse servers.

use anyhow::Result;
use parking_lot::RwLock;
use std::collections::HashMap;
use uuid::Uuid;

use super::packets::{Packet, PacketType};
use super::protocols::{Protocol, QuicProtocol, WebSocketProtocol};

/// Connection state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Handshaking,
    Connected,
    Reconnecting,
}

/// Network connection
#[derive(Debug, Clone)]
pub struct Connection {
    pub id: Uuid,
    pub server: String,
    pub port: u16,
    pub state: ConnectionState,
    pub protocol: Protocol,
}

impl Connection {
    pub fn new(server: String, port: u16, protocol: Protocol) -> Self {
        Self {
            id: Uuid::new_v4(),
            server,
            port,
            state: ConnectionState::Disconnected,
            protocol,
        }
    }
}

/// Network manager for handling connections
pub struct NetworkManager {
    connections: RwLock<HashMap<Uuid, Connection>>,
    active_connection: RwLock<Option<Uuid>>,
    auth_token: RwLock<Option<String>>,
}

impl NetworkManager {
    /// Create a new network manager
    pub fn new() -> Self {
        Self {
            connections: RwLock::new(HashMap::new()),
            active_connection: RwLock::new(None),
            auth_token: RwLock::new(None),
        }
    }
    
    /// Connect to a server
    pub async fn connect(
        &mut self, 
        server: &str, 
        port: u16, 
        auth_token: Option<String>,
    ) -> Result<Uuid> {
        tracing::info!("Connecting to {}:{}", server, port);
        
        // Try QUIC first, then WebSocket fallback
        let protocol = Protocol::Quic(QuicProtocol::new());
        
        let mut conn = Connection::new(server.to_string(), port, protocol);
        conn.state = ConnectionState::Connecting;
        
        let id = conn.id;
        self.connections.write().insert(id, conn);
        *self.active_connection.write() = Some(id);
        *self.auth_token.write() = auth_token;
        
        // Perform handshake
        self.handshake(id).await?;
        
        Ok(id)
    }
    
    /// Perform connection handshake
    async fn handshake(&self, id: Uuid) -> Result<()> {
        let mut connections = self.connections.write();
        let conn = connections.get_mut(&id).expect("Connection not found");
        
        conn.state = ConnectionState::Handshaking;
        tracing::info!("Performing handshake with server...");
        
        // Send HELLO packet
        let hello = Packet::new(PacketType::Hello, vec![]);
        self.send_packet(id, &hello).await?;
        
        conn.state = ConnectionState::Connected;
        tracing::info!("Connected to server successfully");
        
        Ok(())
    }
    
    /// Send a packet
    pub async fn send_packet(&self, _id: Uuid, _packet: &Packet) -> Result<()> {
        // TODO: Implement actual packet sending
        Ok(())
    }
    
    /// Receive and process packets
    pub async fn tick(&mut self) -> Result<()> {
        // TODO: Poll for incoming packets
        Ok(())
    }
    
    /// Disconnect from server
    pub fn disconnect(&self, id: Uuid) {
        let mut connections = self.connections.write();
        if let Some(conn) = connections.get_mut(&id) {
            conn.state = ConnectionState::Disconnected;
            tracing::info!("Disconnected from server");
        }
    }
    
    /// Get active connection
    pub fn active_connection(&self) -> Option<Uuid> {
        *self.active_connection.read()
    }
}

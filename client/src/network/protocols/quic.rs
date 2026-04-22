//! # QUIC Protocol
//!
//! QUIC transport implementation using quinn.

use anyhow::Result;
use quinn::{Endpoint, ClientConfig, Connection};
use std::net::SocketAddr;

/// QUIC protocol implementation
pub struct QuicProtocol {
    endpoint: Option<Endpoint>,
    connection: Option<Connection>,
    remote_addr: Option<SocketAddr>,
}

impl QuicProtocol {
    /// Create a new QUIC protocol handler
    pub fn new() -> Self {
        Self {
            endpoint: None,
            connection: None,
            remote_addr: None,
        }
    }
    
    /// Connect to remote server
    pub async fn connect(&mut self, addr: &str, port: u16) -> Result<()> {
        let remote: SocketAddr = format!("{}:{}", addr, port).parse()?;
        
        let mut endpoint = Endpoint::client("[::]:0".parse()?)?;
        
        let config = ClientConfig::default();
        endpoint.set_config(config)?;
        
        let conn = endpoint.connect(remote, "gameverse")?.await?;
        
        self.endpoint = Some(endpoint);
        self.connection = Some(conn);
        self.remote_addr = Some(remote);
        
        tracing::debug!("QUIC connection established to {}:{}", addr, port);
        
        Ok(())
    }
    
    /// Send data
    pub async fn send(&self, data: &[u8]) -> Result<()> {
        if let Some(ref conn) = self.connection {
            let stream = conn.open_stream().await?;
            stream.write_all(data).await?;
        }
        Ok(())
    }
    
    /// Receive data
    pub async fn recv(&self, buffer: &mut [u8]) -> Result<usize> {
        if let Some(ref conn) = self.connection {
            let stream = conn.accept_stream().await?;
            let n = stream.read(buffer).await?;
            Ok(n.unwrap_or(0))
        } else {
            Ok(0)
        }
    }
    
    /// Check if connected
    pub fn is_connected(&self) -> bool {
        self.connection
            .as_ref()
            .map(|c| c.connection_state().established())
            .unwrap_or(false)
    }
}

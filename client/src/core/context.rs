//! # Client Context
//!
//! Global client state management.

use crate::network::NetworkManager;
use crate::resources::ResourceManager;
use crate::scripting::ScriptManager;
use crate::ui::UIRenderer;
use crate::game::GameIntegration;

use anyhow::Result;
use parking_lot::RwLock;
use std::sync::Arc;

/// Client configuration
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Server address to connect to
    pub server_address: String,
    /// Server port
    pub server_port: u16,
    /// Authentication token
    pub auth_token: Option<String>,
    /// Resource path to load
    pub resource_path: Option<String>,
    /// Enable debug logging
    pub debug: bool,
}

/// Main client context
pub struct Client {
    config: ClientConfig,
    network: Arc<RwLock<NetworkManager>>,
    resources: Arc<RwLock<ResourceManager>>,
    scripts: Arc<RwLock<ScriptManager>>,
    ui: Arc<RwLock<UIRenderer>>,
    game: Arc<RwLock<GameIntegration>>,
    running: Arc<RwLock<bool>>,
}

impl Client {
    /// Create a new client instance
    pub fn new(config: ClientConfig) -> Result<Self> {
        tracing::info!("Initializing GameVerse Client v{}", crate::VERSION);
        
        let client = Self {
            config,
            network: Arc::new(RwLock::new(NetworkManager::new())),
            resources: Arc::new(RwLock::new(ResourceManager::new()?)),
            scripts: Arc::new(RwLock::new(ScriptManager::new()?)),
            ui: Arc::new(RwLock::new(UIRenderer::new()?)),
            game: Arc::new(RwLock::new(GameIntegration::new()?)),
            running: Arc::new(RwLock::new(false)),
        };
        
        Ok(client)
    }
    
    /// Run the client main loop
    pub async fn run(&self) -> Result<()> {
        *self.running.write() = true;
        
        tracing::info!("🎮 GameVerse Client running");
        
        // Connect to server
        {
            let mut network = self.network.write();
            network.connect(
                &self.config.server_address,
                self.config.server_port,
                self.config.auth_token.clone(),
            ).await?;
        }
        
        // Load resources
        if let Some(ref path) = self.config.resource_path {
            let mut resources = self.resources.write();
            resources.load_path(path)?;
        }
        
        // Main loop
        while *self.running.read() {
            self.tick().await?;
            tokio::time::sleep(tokio::time::Duration::from_millis(16)).await;
        }
        
        Ok(())
    }
    
    /// Main tick
    async fn tick(&self) -> Result<()> {
        // Network tick
        {
            let mut network = self.network.write();
            network.tick().await?;
        }
        
        // Scripts tick
        {
            let mut scripts = self.scripts.write();
            scripts.tick()?;
        }
        
        // UI tick
        {
            let mut ui = self.ui.write();
            ui.tick()?;
        }
        
        Ok(())
    }
    
    /// Stop the client
    pub fn shutdown(&self) {
        tracing::info!("Shutting down GameVerse Client");
        *self.running.write() = false;
    }
}

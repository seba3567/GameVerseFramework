//! # NUI Server Module
//!
//! Handles NUI (New UI) communication between client WebView and server scripts.
//! Provides protocol compatibility with FiveM's NUI messaging system.

use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

/// NUI Message from client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NUIMessage {
    /// Message type (e.g., "postMessage")
    pub msg_type: String,
    /// Message data (JSON)
    pub data: serde_json::Value,
}

/// NUI Registration for a resource
#[derive(Debug, Clone)]
pub struct NUIRegistration {
    /// Resource name
    pub resource: String,
    /// Base URL path
    pub url: String,
    /// Whether to capture inputs
    pub capture_inputs: bool,
}

/// Handler for NUI messages
pub type NUIHandler = Box<dyn Fn(NUIMessage) -> Result<()> + Send + Sync>;

/// NUI Server - handles NUI routing for FiveM-compatible scripts
pub struct NUIServer {
    /// Registered NUIs
    registrations: RwLock<HashMap<String, NUIRegistration>>,
    /// Message handlers
    handlers: RwLock<Vec<NUIHandler>>,
    /// Client message receivers (player_id -> tx)
    client_receivers: RwLock<HashMap<u32, mpsc::UnboundedReceiver<NUIMessage>>>,
    /// Active NUI pages by player
    active_nuis: RwLock<HashMap<u32, String>>,
}

impl NUIServer {
    /// Create a new NUI server
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            registrations: RwLock::new(HashMap::new()),
            handlers: RwLock::new(Vec::new()),
            client_receivers: RwLock::new(HashMap::new()),
            active_nuis: RwLock::new(HashMap::new()),
        })
    }
    
    /// Register a NUI page for a resource
    pub fn register(&self, reg: NUIRegistration) -> Result<()> {
        tracing::info!("NUI registering: {} -> {}", reg.resource, reg.url);
        self.registrations.write().insert(reg.resource.clone(), reg);
        Ok(())
    }
    
    /// Unregister a NUI
    pub fn unregister(&self, resource: &str) -> Result<()> {
        tracing::info!("NUI unregistering: {}", resource);
        self.registrations.write().remove(resource);
        Ok(())
    }
    
    /// Get registration for resource
    pub fn get_registration(&self, resource: &str) -> Option<NUIRegistration> {
        self.registrations.read().get(resource).cloned()
    }
    
    /// Register a message handler
    pub fn on_message<H>(&self, handler: H)
    where
        H: Fn(NUIMessage) -> Result<()> + Send + Sync + 'static,
    {
        self.handlers.write().push(Box::new(handler));
    }
    
    /// Send message to all handlers
    fn dispatch_message(&self, msg: NUIMessage) {
        let handlers = self.handlers.read().clone();
        for handler in handlers {
            if let Err(e) = handler(msg.clone()) {
                tracing::error!("NUI handler error: {}", e);
            }
        }
    }
    
    /// Handle incoming message from a client
    pub fn handle_client_message(&self, player_id: u32, msg: NUIMessage) {
        tracing::debug!("NUI message from player {}: {:?}", player_id, msg.msg_type);
        self.dispatch_message(msg);
    }
    
    /// Set active NUI for a player
    pub fn set_active_nui(&self, player_id: u32, resource: Option<&str>) {
        if let Some(res) = resource {
            self.active_nuis.write().insert(player_id, res.to_string());
        } else {
            self.active_nuis.write().remove(&player_id);
        }
    }
    
    /// Get active NUI for a player
    pub fn get_active_nui(&self, player_id: u32) -> Option<String> {
        self.active_nuis.read().get(&player_id).cloned()
    }
    
    /// Get all registrations
    pub fn all_registrations(&self) -> Vec<NUIRegistration> {
        self.registrations.read().values().cloned().collect()
    }
}

/// Response back to NUI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NUIResponse {
    /// Success
    pub success: bool,
    /// Response data
    pub data: serde_json::Value,
}

/// FiveM-compatible NUI URL scheme handler
impl NUIServer {
    /// Build nui:// URL for a resource
    pub fn build_nui_url(&self, resource: &str, path: &str) -> String {
        format!("/nui/{}/{}", resource, path)
    }
    
    /// Parse nui:// URL
    pub fn parse_nui_url(&self, url: &str) -> Option<(String, String)> {
        // Expected format: /nui/{resource}/{path}
        let parts: Vec<&str> = url.trim_start_matches("/nui/").split('/').collect();
        if parts.len() >= 2 {
            Some((parts[0].to_string(), parts[1..].join("/")))
        } else {
            None
        }
    }
    
    /// Check if URL is a NUI URL
    pub fn is_nui_url(&self, url: &str) -> bool {
        url.starts_with("/nui/") || url.starts_with("nui://")
    }
    
    /// Route request to appropriate handler
    pub async fn route_request(&self, url: &str, player_id: u32) -> Result<NUIResponse> {
        if let Some((resource, path)) = self.parse_nui_url(url) {
            if let Some(reg) = self.get_registration(&resource) {
                tracing::debug!(
                    "Routing NUI request: {} -> {}/{} (player {})",
                    url, resource, path, player_id
                );
                
                // For now, return success - actual file serving handled by static file module
                return Ok(NUIResponse {
                    success: true,
                    data: serde_json::json!({
                        "resource": resource,
                        "path": path,
                        "url": format!("{}/{}", reg.url, path),
                    }),
                });
            }
        }
        
        Ok(NUIResponse {
            success: false,
            data: serde_json::json!({"error": "NUI not found"}),
        })
    }
}

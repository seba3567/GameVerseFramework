//! # Event System
//!
//! Pub/sub event system for client-server communication.

use std::collections::HashMap;
use parking_lot::RwLock;
use anyhow::Result;
use uuid::Uuid;

/// Event listener callback
pub type EventCallback = Box<dyn Fn(&[u8]) + Send + Sync>;

/// Event bus for pub/sub communication
pub struct EventBus {
    listeners: RwLock<HashMap<String, Vec<(Uuid, EventCallback)>>>,
}

impl EventBus {
    /// Create a new event bus
    pub fn new() -> Self {
        Self {
            listeners: RwLock::new(HashMap::new()),
        }
    }
    
    /// Subscribe to an event
    pub fn on(&self, event: &str, callback: EventCallback) -> Uuid {
        let id = Uuid::new_v4();
        let mut listeners = self.listeners.write();
        listeners.entry(event.to_string())
            .or_default()
            .push((id, callback));
        id
    }
    
    /// Unsubscribe from an event
    pub fn off(&self, event: &str, id: Uuid) {
        let mut listeners = self.listeners.write();
        if let Some(callbacks) = listeners.get_mut(event) {
            callbacks.retain(|(cb_id, _)| *cb_id != id);
        }
    }
    
    /// Emit an event
    pub fn emit(&self, event: &str, data: &[u8]) {
        let listeners = self.listeners.read();
        if let Some(callbacks) = listeners.get(event) {
            for (_, callback) in callbacks {
                callback(data);
            }
        }
    }
    
    /// Emit to server
    pub async fn emit_server(&self, _event: &str, _data: &[u8]) -> Result<()> {
        // TODO: Send to network manager
        Ok(())
    }
}

// Default event types
#[derive(Debug, Clone)]
pub enum ClientEvent {
    /// Connected to server
    Connect { server: String, port: u16 },
    /// Disconnected from server
    Disconnect { reason: String },
    /// Resource started loading
    ResourceStart { name: String },
    /// Resource loaded
    ResourceReady { name: String },
    /// Resource stopped
    ResourceStop { name: String },
    /// Main tick
    Tick { delta_ms: u64 },
    /// Server sent an event
    ServerEvent { name: String, data: Vec<u8> },
    /// UI ready
    UIReady,
    /// Game ready
    GameReady,
}

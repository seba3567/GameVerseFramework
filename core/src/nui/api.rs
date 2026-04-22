//! # NUI API for Scripts
//!
//! Lua and TypeScript API for NUI operations.

use super::server::{NUIServer, NUIMessage, NUIRegistration};
use std::sync::Arc;

/// Lua bindings for NUI
pub mod lua {
    use super::*;
    
    /// Register NUI from Lua
    pub fn register_nui_lua(s: &NUIServer, resource: &str, url: &str) -> Result<()> {
        s.register(NUIRegistration {
            resource: resource.to_string(),
            url: url.to_string(),
            capture_inputs: true,
        })
    }
    
    /// Send NUI message from Lua to client
    pub fn send_to_nui(s: &NUIServer, player_id: u32, msg_type: &str, data: serde_json::Value) -> Result<()> {
        // TODO: route to specific player's WebView
        tracing::debug!("Sending to NUI player {}: {} {:?}", player_id, msg_type, data);
        Ok(())
    }
    
    /// Set focus for NUI
    pub fn set_nui_focus(s: &NUIServer, player_id: u32, focused: bool) -> Result<()> {
        s.set_active_nui(player_id, if focused { Some("focused") } else { None });
        Ok(())
    }
}

/// TypeScript bindings for NUI  
pub mod typescript {
    use super::*;
    
    /// Register NUI from TypeScript
    pub fn register_nui(s: &NUIServer, resource: &str, url: &str) -> Result<()> {
        s.register(NUIRegistration {
            resource: resource.to_string(),
            url: url.to_string(),
            capture_inputs: true,
        })
    }
    
    /// Send message to client NUI
    pub fn sendToNUI(player_id: u32, msg: NUIMessage) -> Result<()> {
        tracing::debug!("TS: Send to NUI player {}: {:?}", player_id, msg.msg_type);
        Ok(())
    }
    
    /// Set NUI focus
    pub fn setFocus(player_id: u32, focused: bool) -> Result<()> {
        tracing::debug!("TS: NUI focus {} = {}", player_id, focused);
        Ok(())
    }
    
    /// Create NUI callback
    pub fn onNUIMessage<F>(s: &NUIServer, callback: F)
    where
        F: Fn(NUIMessage) -> Result<()> + Send + Sync + 'static,
    {
        s.on_message(callback);
    }
}

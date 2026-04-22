//! # UI Renderer
//!
//! Cross-platform UI rendering using Tauri webview.

use anyhow::Result;
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

use super::cef_bridge::{CefBridge, NUIMessage};
use super::webrenderer::WebRenderer;

/// Message sent to UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIMessage {
    /// Target ("nui", "hud", "debug")
    pub target: String,
    /// Message type
    pub msg_type: String,
    /// Message data
    pub data: serde_json::Value,
}

/// UI Renderer - coordinates CEF/WebRenderer and NUI
pub struct UIRenderer {
    /// CEF bridge for NUI
    cef: Arc<CefBridge>,
    /// Web renderer fallback
    web: Arc<WebRenderer>,
    /// Whether initialized
    initialized: bool,
}

impl UIRenderer {
    /// Create a new UI renderer
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            cef: CefBridge::new(),
            web: WebRenderer::new(),
            initialized: false,
        })
    }
    
    /// Get CEF bridge reference
    pub fn cef(&self) -> Arc<CefBridge> {
        self.cef.clone()
    }
    
    /// Initialize renderer
    pub fn initialize(&mut self) -> anyhow::Result<()> {
        if self.initialized {
            return Ok(());
        }
        
        tracing::info!("Initializing UI renderer");
        
        self.cef.initialize()?;
        self.web.initialize()?;
        
        self.initialized = true;
        Ok(())
    }
    
    /// Render a tick
    pub fn tick(&self) -> anyhow::Result<()> {
        if !self.initialized {
            return Ok(());
        }
        
        // Process any pending scripts
        Ok(())
    }
    
    /// Send message to UI
    pub fn send_message(&self, target: &str, msg: &str) -> anyhow::Result<()> {
        match target {
            "nui" => {
                let data: serde_json::Value = serde_json::from_str(msg)?;
                self.cef.post_message(NUIMessage {
                    msg_type: "postMessage".to_string(),
                    data,
                })?;
            }
            _ => {
                tracing::warn!("Unknown UI target: {}", target);
            }
        }
        Ok(())
    }
    
    /// Load HTML content
    pub fn load_html(&self, html: &str) -> anyhow::Result<()> {
        self.web.load_html(html)
    }
    
    /// Load URL
    pub fn load_url(&self, url: &str) -> anyhow::Result<()> {
        self.cef.load_url(url)
    }
    
    /// Evaluate JavaScript
    pub fn eval_js(&self, js: &str) -> anyhow::Result<serde_json::Value> {
        self.web.eval_js(js)
    }
}
    
    /// Send message to UI
    pub fn send_message(&self, _target: &str, _message: &str) -> anyhow::Result<()> {
        Ok(())
    }
    
    /// Load HTML content
    pub fn load_html(&mut self, _html: &str) -> anyhow::Result<()> {
        Ok(())
    }
    
    /// Evaluate JavaScript
    pub fn eval_js(&mut self, _js: &str) -> anyhow::Result<()> {
        Ok(())
    }
}

// Placeholder for CEF integration
mod cef {
    pub struct Bridge;
}

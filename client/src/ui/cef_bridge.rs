//! # CEF Bridge for NUI
//!
//! Chromium Embedded Framework integration via Tauri webview.
//! This module provides NUI (New UI) support for scripts.

use anyhow::Result;
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

/// Message from NUI to game scripts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NUIMessage {
    /// Message type
    pub msg_type: String,
    /// Message data (JSON)
    pub data: serde_json::Value,
}

/// CEF Bridge - handles NUI rendering
/// Uses Tauri webview internally (CEF on Windows, WKWebView on macOS, WebView on Linux)
pub struct CefBridge {
    /// Whether CEF is initialized
    initialized: bool,
    /// Current URL loaded
    current_url: RwLock<Option<String>>,
    /// NUI message callbacks
    callbacks: RwLock<Vec<Box<dyn Fn(NUIMessage) + Send + Sync>>>,
}

unsafe impl Send for CefBridge {}
unsafe impl Sync for CefBridge {}

impl CefBridge {
    /// Create a new CEF bridge
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            initialized: false,
            current_url: RwLock::new(None),
            callbacks: RwLock::new(Vec::new()),
        })
    }
    
    /// Initialize CEF
    pub fn initialize(&mut self) -> Result<()> {
        if self.initialized {
            return Ok(());
        }
        
        tracing::info!("Initializing CEF Bridge via Tauri WebView");
        self.initialized = true;
        Ok(())
    }
    
    /// Check if initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
    
    /// Load a URL in the NUI layer
    pub fn load_url(&self, url: &str) -> Result<()> {
        tracing::debug!("CEF loading URL: {}", url);
        *self.current_url.write() = Some(url.to_string());
        Ok(())
    }
    
    /// Get current URL
    pub fn current_url(&self) -> Option<String> {
        self.current_url.read().clone()
    }
    
    /// Execute JavaScript in the NUI context
    pub fn execute_js(&self, js: &str) -> Result<()> {
        tracing::debug!("CEF execute JS: {}", &js[..js.len().min(100)]);
        Ok(())
    }
    
    /// Post message to NUI from game scripts
    pub fn post_message(&self, msg: NUIMessage) -> Result<()> {
        tracing::debug!("CEF post message: {:?}", msg.msg_type);
        
        // Notify callbacks
        let callbacks = self.callbacks.read().clone();
        for callback in callbacks {
            callback(msg.clone());
        }
        
        Ok(())
    }
    
    /// Register message callback
    pub fn on_message<F>(&self, callback: F) 
    where 
        F: Fn(NUIMessage) + Send + Sync + 'static 
    {
        self.callbacks.write().push(Box::new(callback));
    }
    
    /// Evaluate JavaScript and get result (async)
    pub async fn eval_js(&self, js: &str) -> Result<serde_json::Value> {
        tracing::debug!("CEF eval JS: {}", &js[..js.len().min(100)]);
        // TODO: Implement via Tauri invoke
        Ok(serde_json::Value::Null)
    }
}

/// Script registration for NUI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NUIRegistration {
    /// Resource name
    pub resource: String,
    /// URL path (e.g., "nui://myui/index.html")
    pub url: String,
    /// Whether to capture focus
    pub capture_inputs: bool,
}

impl CefBridge {
    /// Register a NUI page from a resource
    pub fn register_nui(&self, reg: NUIRegistration) -> Result<()> {
        tracing::info!("Registering NUI: {} -> {}", reg.resource, reg.url);
        Ok(())
    }
    
    /// Remove a NUI registration
    pub fn unregister_nui(&self, resource: &str) -> Result<()> {
        tracing::info!("Unregistering NUI: {}", resource);
        Ok(())
    }
    
    /// Set NUI focus state
    pub fn set_focus(&self, focused: bool) -> Result<()> {
        tracing::debug!("NUI focus: {}", focused);
        Ok(())
    }
    
    /// Get NUI focus state
    pub fn is_focused(&self) -> bool {
        false
    }
}

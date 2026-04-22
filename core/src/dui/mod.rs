//! # DUI - Dynamic UI
//!
//! Alternative to NUI for creating UI directly from scripts.
//! DUI uses game-native rendering (like GTA V text/draw commands) instead of HTML.
//!
//! **DUI is not FiveM-compatible** - it's a GameVerse-native feature.
//! Use DUI when you want lightweight, fast UI without the overhead of a WebView.
//!
//! ## DUI vs NUI
//!
//! | Feature | DUI | NUI |
//! |---------|-----|-----|
//! | Rendering | Game primitives | HTML/CSS/JS |
//! | Performance | Faster | Slower |
//! | Compatibility | GameVerse only | FiveM compatible |
//! | Flexibility | Limited | Unlimited |
//!
//! ## Usage
//!
//! ```lua
//! -- Create a simple text label
//! local label = DUI.Create("label", {
//!     text = "Hello World",
//!     x = 0.5,
//!     y = 0.5,
//!     scale = 0.5,
//!     color = {255, 255, 255, 255}
//! })
//!
//! -- Remove it
//! DUI.Destroy(label)
//! ```

use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

/// DUI Element types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DUIElementType {
    /// Text label
    Label,
    /// Rectangle/box
    Rectangle,
    /// Sprite/image
    Sprite,
    /// Button
    Button,
    /// Input field
    Input,
    /// Progress bar
    ProgressBar,
    /// Custom draw
    Custom,
}

/// DUI Element configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DUIElementConfig {
    /// Element type
    pub element_type: DUIElementType,
    /// Position X (0.0 - 1.0 normalized)
    pub x: f32,
    /// Position Y (0.0 - 1.0 normalized)
    pub y: f32,
    /// Width (0.0 - 1.0 normalized, -1 for auto)
    pub width: f32,
    /// Height (0.0 - 1.0 normalized, -1 for auto)
    pub height: f32,
    /// Rotation in degrees
    pub rotation: f32,
    /// RGBA color
    pub color: [u8; 4],
    /// Element-specific data (text, sprite path, etc.)
    pub data: serde_json::Value,
    /// Visibility
    pub visible: bool,
    /// Z-order (higher = on top)
    pub z_order: i32,
    /// Children elements
    pub children: Vec<String>,
}

/// DUI Element handle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DUIHandle(pub u64);

/// DUI Manager - handles dynamic UI elements
pub struct DUI {
    /// Active elements
    elements: RwLock<HashMap<u64, DUIElementConfig>>,
    /// Next handle ID
    next_id: RwLock<u64>,
    /// DUI enabled state
    enabled: RwLock<bool>,
}

impl DUI {
    /// Create new DUI manager
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            elements: RwLock::new(HashMap::new()),
            next_id: RwLock::new(1),
            enabled: RwLock::new(false),
        })
    }
    
    /// Enable DUI rendering
    pub fn enable(&self) {
        *self.enabled.write() = true;
        tracing::debug!("DUI enabled");
    }
    
    /// Disable DUI rendering
    pub fn disable(&self) {
        *self.enabled.write() = false;
        tracing::debug!("DUI disabled");
    }
    
    /// Check if DUI is enabled
    pub fn is_enabled(&self) -> bool {
        *self.enabled.read()
    }
    
    /// Create a new DUI element
    pub fn create(&self, element_type: DUIElementType, config: DUIElementConfig) -> DUIHandle {
        let id = {
            let mut next = self.next_id.write();
            let id = *next;
            *next += 1;
            id
        };
        
        let mut full_config = config;
        full_config.element_type = element_type;
        full_config.visible = true;
        
        self.elements.write().insert(id, full_config);
        tracing::debug!("Created DUI element: {:?}", element_type);
        
        DUIHandle(id)
    }
    
    /// Destroy a DUI element
    pub fn destroy(&self, handle: DUIHandle) -> Result<()> {
        if self.elements.write().remove(&handle.0).is_some() {
            tracing::debug!("Destroyed DUI element: {:?}", handle);
            Ok(())
        } else {
            anyhow::bail!("DUI element not found: {:?}", handle)
        }
    }
    
    /// Update element properties
    pub fn update(&self, handle: DUIHandle, config: Partial<DUIElementConfig>) -> Result<()> {
        let mut elements = self.elements.write();
        if let Some(element) = elements.get_mut(&handle.0) {
            // Apply partial update
            if let Some(x) = config.x { element.x = x; }
            if let Some(y) = config.y { element.y = y; }
            if let Some(width) = config.width { element.width = width; }
            if let Some(height) = config.height { element.height = height; }
            if let Some(rotation) = config.rotation { element.rotation = rotation; }
            if let Some(color) = config.color { element.color = color; }
            if let Some(visible) = config.visible { element.visible = visible; }
            if let Some(z_order) = config.z_order { element.z_order = z_order; }
            tracing::debug!("Updated DUI element: {:?}", handle);
            Ok(())
        } else {
            anyhow::bail!("DUI element not found: {:?}", handle)
        }
    }
    
    /// Set element position
    pub fn set_position(&self, handle: DUIHandle, x: f32, y: f32) -> Result<()> {
        self.update(handle, DUIElementConfig { x, y, ..Default::default() })
    }
    
    /// Set element color
    pub fn set_color(&self, handle: DUIHandle, color: [u8; 4]) -> Result<()> {
        self.update(handle, DUIElementConfig { color, ..Default::default() })
    }
    
    /// Set element visibility
    pub fn set_visible(&self, handle: DUIHandle, visible: bool) -> Result<()> {
        self.update(handle, DUIElementConfig { visible, ..Default::default() })
    }
    
    /// Get all visible elements for rendering
    pub fn get_visible_elements(&self) -> Vec<(u64, DUIElementConfig)> {
        self.elements
            .read()
            .iter()
            .filter(|(_, e)| e.visible)
            .map(|(k, v)| (*k, v.clone()))
            .collect()
    }
    
    /// Get element count
    pub fn element_count(&self) -> usize {
        self.elements.read().len()
    }
    
    /// Clear all elements
    pub fn clear(&self) {
        self.elements.write().clear();
        tracing::debug!("Cleared all DUI elements");
    }
}

impl Default for DUIElementConfig {
    fn default() -> Self {
        Self {
            element_type: DUIElementType::Rectangle,
            x: 0.0,
            y: 0.0,
            width: 0.1,
            height: 0.05,
            rotation: 0.0,
            color: [255, 255, 255, 255],
            data: serde_json::json!({}),
            visible: true,
            z_order: 0,
            children: Vec::new(),
        }
    }
}

impl Partial<DUIElementConfig> {
    /// Create partial config for updates
    pub fn new() -> Self {
        Self::default()
    }
}

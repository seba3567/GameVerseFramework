//! # DUI Lua API
//!
//! Lua bindings for DUI (Dynamic UI).

use super::{DUI, DUIHandle, DUIElementType, DUIElementConfig};
use std::sync::Arc;
use serde_json::Value;

/// Lua API for DUI
pub struct DUILua {
    /// DUI manager
    dui: Arc<DUI>,
}

impl DUILua {
    /// Create new DUI Lua API
    pub fn new(dui: Arc<DUI>) -> Self {
        Self { dui }
    }
    
    /// Enable DUI
    pub fn enable(&self) {
        self.dui.enable();
    }
    
    /// Disable DUI
    pub fn disable(&self) {
        self.dui.disable();
    }
    
    /// Create element
    pub fn create(&self, element_type: &str, data: Value) -> DUIHandle {
        let et = match element_type {
            "label" => DUIElementType::Label,
            "rectangle" | "rect" => DUIElementType::Rectangle,
            "sprite" => DUIElementType::Sprite,
            "button" => DUIElementType::Button,
            "input" => DUIElementType::Input,
            "progress" | "progressbar" => DUIElementType::ProgressBar,
            "custom" => DUIElementType::Custom,
            _ => DUIElementType::Rectangle,
        };
        
        let mut config = DUIElementConfig::default();
        config.data = data;
        
        self.dui.create(et, config)
    }
    
    /// Destroy element
    pub fn destroy(&self, handle: DUIHandle) -> Result<(), String> {
        self.dui.destroy(handle).map_err(|e| e.to_string())
    }
    
    /// Update element
    pub fn update(&self, handle: DUIHandle, data: Value) -> Result<(), String> {
        let mut config = DUIElementConfig::default();
        if let Some(x) = data.get("x").and_then(|v| v.as_f64()) {
            config.x = x as f32;
        }
        if let Some(y) = data.get("y").and_then(|v| v.as_f64()) {
            config.y = y as f32;
        }
        if let Some(width) = data.get("width").and_then(|v| v.as_f64()) {
            config.width = width as f32;
        }
        if let Some(height) = data.get("height").and_then(|v| v.as_f64()) {
            config.height = height as f32;
        }
        if let Some(color) = data.get("color").and_then(|v| v.as_array()) {
            if color.len() >= 4 {
                config.color = [
                    color[0].as_u64().unwrap_or(255) as u8,
                    color[1].as_u64().unwrap_or(255) as u8,
                    color[2].as_u64().unwrap_or(255) as u8,
                    color[3].as_u64().unwrap_or(255) as u8,
                ];
            }
        }
        if let Some(visible) = data.get("visible").and_then(|v| v.as_bool()) {
            config.visible = visible;
        }
        
        self.dui.update(handle, config).map_err(|e| e.to_string())
    }
    
    /// Set position
    pub fn set_position(&self, handle: DUIHandle, x: f32, y: f32) -> Result<(), String> {
        self.dui.set_position(handle, x, y).map_err(|e| e.to_string())
    }
    
    /// Set color
    pub fn set_color(&self, handle: DUIHandle, r: u8, g: u8, b: u8, a: u8) -> Result<(), String> {
        self.dui.set_color(handle, [r, g, b, a]).map_err(|e| e.to_string())
    }
    
    /// Set visible
    pub fn set_visible(&self, handle: DUIHandle, visible: bool) -> Result<(), String> {
        self.dui.set_visible(handle, visible).map_err(|e| e.to_string())
    }
    
    /// Clear all
    pub fn clear(&self) {
        self.dui.clear();
    }
    
    /// Get element count
    pub fn count(&self) -> usize {
        self.dui.element_count()
    }
}

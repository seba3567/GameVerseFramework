//! # Resource Manifest
//!
//! Parsing and handling fxmanifest.lua files.

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Resource manifest (fxmanifest.lua)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceManifest {
    /// Resource name
    pub name: String,
    /// Resource version
    pub version: String,
    /// Resource author
    pub author: Option<String>,
    /// Resource description
    pub description: Option<String>,
    /// Client scripts
    pub client_scripts: Vec<String>,
    /// Server scripts
    pub server_scripts: Vec<String>,
    /// Shared scripts
    pub shared_scripts: Vec<String>,
    /// UI files
    pub ui_page: Option<String>,
    /// Dependencies
    pub dependencies: Vec<String>,
    /// Native requirements
    pub natives: Option<Vec<String>>,
    /// Game types supported
    pub game_types: Vec<String>,
}

impl ResourceManifest {
    /// Create a default manifest
    pub fn default() -> Self {
        Self {
            name: "unknown".to_string(),
            version: "1.0.0".to_string(),
            author: None,
            description: None,
            client_scripts: Vec::new(),
            server_scripts: Vec::new(),
            shared_scripts: Vec::new(),
            ui_page: None,
            dependencies: Vec::new(),
            natives: None,
            game_types: vec!["gta5".to_string(), "rdr3".to_string()],
        }
    }
}

/// Parse fxmanifest.lua file
pub fn parse_manifest(path: &Path) -> anyhow::Result<ResourceManifest> {
    let content = std::fs::read_to_string(path)?;
    parse_manifest_content(&content)
}

/// Parse fxmanifest.lua content
pub fn parse_manifest_content(content: &str) -> anyhow::Result<ResourceManifest> {
    // Simple parser for fxmanifest.lua
    // Format: fx_version 'oman'
    // game 'gta5'
    // name 'My Resource'
    // description 'Description'
    // version '1.0.0'
    // author 'Author'
    // client_scripts { 'client.lua' }
    
    let mut manifest = ResourceManifest::default();
    
    for line in content.lines() {
        let line = line.trim();
        
        if line.starts_with("name ") || line.starts_with("name'") {
            if let Some(name) = extract_quoted_value(line) {
                manifest.name = name;
            }
        } else if line.starts_with("version ") {
            if let Some(version) = extract_quoted_value(line) {
                manifest.version = version;
            }
        } else if line.starts_with("author ") {
            manifest.author = extract_quoted_value(line);
        } else if line.starts_with("description ") {
            manifest.description = extract_quoted_value(line);
        } else if line.contains("client_scripts") {
            manifest.client_scripts = extract_array_values(line);
        } else if line.contains("server_scripts") {
            manifest.server_scripts = extract_array_values(line);
        } else if line.contains("ui_page") {
            if let Some(page) = extract_quoted_value(line) {
                manifest.ui_page = Some(page);
            }
        }
    }
    
    Ok(manifest)
}

/// Extract quoted value from line
fn extract_quoted_value(line: &str) -> Option<String> {
    line.split('\'')
        .nth(1)
        .map(|s| s.to_string())
}

/// Extract array values from line
fn extract_array_values(line: &str) -> Vec<String> {
    let mut values = Vec::new();
    let content = line.chars().filter(|c| !c.is_whitespace()).collect::<String>();
    
    // Find content between braces
    if let Some(start) = content.find('{') {
        if let Some(end) = content.find('}') {
            let array_content = &content[start+1..end];
            for value in array_content.split(',') {
                let v = value.trim().trim_matches('\'');
                if !v.is_empty() {
                    values.push(v.to_string());
                }
            }
        }
    }
    
    values
}

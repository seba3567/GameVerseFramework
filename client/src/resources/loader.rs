//! # Resource Loader
//!
//! Loads and manages FiveM-style resources.

use anyhow::Result;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::path::Path;

use super::manifest::ResourceManifest;

/// Loaded resource
pub struct Resource {
    pub name: String,
    pub path: String,
    pub manifest: ResourceManifest,
}

/// Resource manager for loading/unloading resources
pub struct ResourceManager {
    resources: RwLock<HashMap<String, Resource>>,
    resource_path: RwLock<Option<String>>,
}

impl ResourceManager {
    /// Create a new resource manager
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            resources: RwLock::new(HashMap::new()),
            resource_path: RwLock::new(None),
        })
    }
    
    /// Set resource load path
    pub fn load_path(&mut self, path: &str) -> Result<(), anyhow::Error> {
        *self.resource_path.write() = Some(path.to_string());
        
        // Scan and load resources from path
        let path = Path::new(path);
        if path.is_dir() {
            self.scan_directory(path)?;
        }
        
        Ok(())
    }
    
    /// Scan directory for resources
    fn scan_directory(&mut self, path: &Path) -> Result<(), anyhow::Error> {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    // Check for fxmanifest.lua
                    let manifest_path = entry_path.join("fxmanifest.lua");
                    if manifest_path.exists() {
                        self.load_resource(&entry_path)?;
                    }
                }
            }
        }
        Ok(())
    }
    
    /// Load a single resource
    pub fn load_resource(&mut self, path: &Path) -> Result<(), anyhow::Error> {
        let manifest_path = path.join("fxmanifest.lua");
        let manifest = super::manifest::parse_manifest(&manifest_path)?;
        
        let name = manifest.name.clone();
        let resource = Resource {
            name: name.clone(),
            path: path.to_string_lossy().to_string(),
            manifest,
        };
        
        tracing::info!("Loading resource: {}", name);
        self.resources.write().insert(name, resource);
        
        Ok(())
    }
    
    /// Unload a resource
    pub fn unload_resource(&mut self, name: &str) -> Result<(), anyhow::Error> {
        tracing::info!("Unloading resource: {}", name);
        self.resources.write().remove(name);
        Ok(())
    }
    
    /// Get a resource by name
    pub fn get_resource(&self, name: &str) -> Option<Resource> {
        self.resources.read().get(name).cloned()
    }
    
    /// List all loaded resources
    pub fn list_resources(&self) -> Vec<String> {
        self.resources.read().keys().cloned().collect()
    }
}

//! # WebAssembly Runtime
//!
//! WebAssembly runtime via Wasmtime.

use anyhow::Result;
use wasmtime::{Engine, Store, Module, Instance};
use parking_lot::RwLock;
use std::collections::HashMap;

/// WebAssembly runtime for client scripts
pub struct WasmRuntime {
    engine: Engine,
    store: Store<()>,
    modules: RwLock<HashMap<String, Instance>>,
}

impl WasmRuntime {
    /// Create a new WebAssembly runtime
    pub fn new() -> anyhow::Result<Self> {
        let engine = Engine::new_default()?;
        let store = Store::new(&engine, ());
        
        Ok(Self {
            engine,
            store,
            modules: RwLock::new(HashMap::new()),
        })
    }
    
    /// Instantiate a WASM module
    pub fn instantiate(&mut self, name: &str, bytes: &[u8]) -> anyhow::Result<()> {
        let module = Module::new(&self.engine, bytes)?;
        let instance = Instance::new(&mut self.store, &module, &[])?;
        
        self.modules.write().insert(name.to_string(), instance);
        
        tracing::debug!("Loaded WASM module: {}", name);
        Ok(())
    }
    
    /// Execute a tick
    pub fn tick(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
    
    /// Call a function in a module
    pub fn call(&mut self, module: &str, func: &str) -> anyhow::Result<()> {
        let modules = self.modules.read();
        if let Some(instance) = modules.get(module) {
            if let Some(func) = instance.get_func(&mut self.store, func) {
                func.call(&mut self.store, &[], &mut [])?;
            }
        }
        Ok(())
    }
}

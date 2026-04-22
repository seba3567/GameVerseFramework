//! # Lua Runtime
//!
//! Lua/LuaJIT scripting support via mlua.

use mlua::{Lua, Result as LuaResult};
use parking_lot::RwLock;
use std::collections::HashMap;

/// Lua runtime for client scripts
pub struct LuaRuntime {
    lua: Lua,
    scripts: RwLock<HashMap<String, mlua::Thread>>,
}

impl LuaRuntime {
    /// Create a new Lua runtime
    pub fn new() -> anyhow::Result<Self> {
        let lua = Lua::new();
        Ok(Self {
            lua,
            scripts: RwLock::new(HashMap::new()),
        })
    }
    
    /// Execute a script
    pub fn execute(&mut self, name: &str, source: &str) -> anyhow::Result<()> {
        let globals = self.lua.globals();
        
        // Register client API
        let client_api = self.lua.create_table()?;
        client_api.set("trigger", self.lua.create_function(|_, event: String, data: String| {
            tracing::debug!("Client trigger: {} with data: {}", event, data);
            Ok(())
        }))?;
        client_api.set("emit", self.lua.create_function(|_, event: String, data: String| {
            tracing::debug!("Client emit: {} with data: {}", event, data);
            Ok(())
        }))?;
        globals.set("Client", client_api)?;
        
        // Compile and store script
        let chunk = self.lua.load(source).into_thread()?;
        self.scripts.write().insert(name.to_string(), chunk);
        
        tracing::debug!("Loaded Lua script: {}", name);
        Ok(())
    }
    
    /// Trigger an event in Lua scripts
    pub fn trigger_event(&mut self, event: &str, data: &[u8]) -> anyhow::Result<()> {
        let globals = self.lua.globals();
        let event_handler: mlua::Function = globals.get("HandleEvent")?;
        
        let lua_data = String::from_utf8_lossy(data).to_string();
        event_handler.call((event, lua_data))?;
        
        Ok(())
    }
    
    /// Execute a tick
    pub fn tick(&mut self) -> anyhow::Result<()> {
        // Resume any suspended threads
        let scripts = self.scripts.read();
        for (_, thread) in scripts.iter() {
            if let Ok(mlua::ThreadStatus::Resumed) = thread.status() {
                // Thread still running
            }
        }
        Ok(())
    }
}

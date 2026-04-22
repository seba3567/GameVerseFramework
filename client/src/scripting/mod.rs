//! # Scripting Module
//!
//! Multi-language script runtime support.

pub mod lua_runtime;
pub mod ts_runtime;
pub mod wasm_runtime;

pub use lua_runtime::LuaRuntime;
pub use ts_runtime::TsRuntime;
pub use wasm_runtime::WasmRuntime;

/// Script manager for handling multiple runtimes
pub struct ScriptManager {
    lua: Option<LuaRuntime>,
    ts: Option<TsRuntime>,
    wasm: Option<WasmRuntime>,
}

impl ScriptManager {
    /// Create a new script manager
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            lua: Some(LuaRuntime::new()?),
            ts: None, // Deno integration requires more setup
            wasm: Some(WasmRuntime::new()?),
        })
    }
    
    /// Execute a tick on all runtimes
    pub fn tick(&mut self) -> anyhow::Result<()> {
        if let Some(ref mut lua) = self.lua {
            lua.tick()?;
        }
        if let Some(ref mut wasm) = self.wasm {
            wasm.tick()?;
        }
        Ok(())
    }
    
    /// Execute a server event
    pub fn handle_server_event(&mut self, event: &str, data: &[u8]) -> anyhow::Result<()> {
        if let Some(ref mut lua) = self.lua {
            lua.trigger_event(event, data)?;
        }
        Ok(())
    }
    
    /// Load a Lua script
    pub fn load_lua(&mut self, name: &str, source: &str) -> anyhow::Result<()> {
        if let Some(ref mut lua) = self.lua {
            lua.execute(name, source)?;
        }
        Ok(())
    }
    
    /// Load a WASM module
    pub fn load_wasm(&mut self, name: &str, bytes: &[u8]) -> anyhow::Result<()> {
        if let Some(ref mut wasm) = self.wasm {
            wasm.instantiate(name, bytes)?;
        }
        Ok(())
    }
}

//! # Native Functions
//!
//! Game native function bindings.

use std::collections::HashMap;
use parking_lot::RwLock;

/// Native function hash type
pub type NativeHash = u64;

/// Native function signature
pub type NativeFn = fn(Vec<NativeArg>) -> NativeResult;

/// Native function argument
#[derive(Debug, Clone)]
pub enum NativeArg {
    Int(i64),
    Float(f64),
    String(String),
    Vector3(f64, f64, f64),
    Bool(bool),
}

/// Native function result
#[derive(Debug, Clone)]
pub struct NativeResult {
    pub ret: Option<NativeArg>,
}

/// Native function registry
pub struct NativeRegistry {
    natives: RwLock<HashMap<NativeHash, NativeFn>>,
}

impl NativeRegistry {
    /// Create a new native registry
    pub fn new() -> Self {
        Self {
            natives: RwLock::new(HashMap::new()),
        }
    }
    
    /// Register a native function
    pub fn register(&self, hash: NativeHash, func: NativeFn) {
        self.natives.write().insert(hash, func);
    }
    
    /// Call a native function
    pub fn call(&self, hash: NativeHash, args: Vec<NativeArg>) -> NativeResult {
        let natives = self.natives.read();
        if let Some(func) = natives.get(&hash) {
            func(args)
        } else {
            tracing::warn!("Native function {:x} not found", hash);
            NativeResult { ret: None }
        }
    }
    
    /// Get native hash from name
    pub fn hash_function(&self, name: &str) -> NativeHash {
        // Simple hash function for native names
        let mut hash: u64 = 0xCBF43CEE;
        for c in name.as_bytes() {
            hash = hash.rotate_left(7) ^ (*c as u64);
        }
        hash
    }
}

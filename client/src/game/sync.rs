//! # Client-Side Synchronization
//!
//! Client-side prediction and entity synchronization.

use anyhow::Result;
use std::collections::HashMap;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

/// Entity synchronization state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySyncState {
    pub entity_id: u32,
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub velocity: [f32; 3],
    pub timestamp: u64,
}

/// Client-side prediction manager
pub struct SyncManager {
    local_entities: RwLock<HashMap<u32, EntitySyncState>>,
    remote_entities: RwLock<HashMap<u32, EntitySyncState>>,
}

impl SyncManager {
    /// Create a new sync manager
    pub fn new() -> Self {
        Self {
            local_entities: RwLock::new(HashMap::new()),
            remote_entities: RwLock::new(HashMap::new()),
        }
    }
    
    /// Update local player state
    pub fn update_local(&self, entity_id: u32, state: EntitySyncState) {
        self.local_entities.write().insert(entity_id, state);
    }
    
    /// Update remote entity state
    pub fn update_remote(&self, entity_id: u32, state: EntitySyncState) {
        self.remote_entities.write().insert(entity_id, state);
    }
    
    /// Get interpolated state for rendering
    pub fn get_interpolated_state(&self, entity_id: u32, time: u64) -> Option<EntitySyncState> {
        let remote = self.remote_entities.read();
        remote.get(&entity_id).cloned()
    }
    
    /// Apply server corrections
    pub fn apply_correction(&self, entity_id: u32, state: &EntitySyncState) {
        tracing::debug!("Applying server correction for entity {}", entity_id);
        self.remote_entities.write().insert(entity_id, state.clone());
    }
}

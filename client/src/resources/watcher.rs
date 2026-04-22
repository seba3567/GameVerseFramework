//! # Resource Watcher
//!
//! Hot-reload file watching for resources.

use anyhow::Result;
use notify::{Watcher, RecursiveMode, Event, EventKind};
use std::path::Path;
use std::sync::mpsc::channel;
use parking_lot::RwLock;

/// File watcher for hot-reload
pub struct ResourceWatcher {
    watcher: RwLock<Option<notify::RecommendedWatcher>>,
    watched_paths: RwLock<Vec<String>>,
}

impl ResourceWatcher {
    /// Create a new resource watcher
    pub fn new() -> Self {
        Self {
            watcher: RwLock::new(None),
            watched_paths: RwLock::new(Vec::new()),
        }
    }
    
    /// Start watching a path
    pub fn watch(&mut self, path: &str) -> Result<(), anyhow::Error> {
        let (tx, rx) = channel();
        
        let mut watcher = notify::recommended_watcher(move |res: Result<Event, _>| {
            if let Ok(event) = res {
                let _ = tx.send(event);
            }
        })?;
        
        watcher.watch(Path::new(path), RecursiveMode::Recursive)?;
        
        *self.watcher.write() = Some(watcher);
        self.watched_paths.write().push(path.to_string());
        
        // Spawn event handler thread
        std::thread::spawn(move || {
            while let Ok(event) = rx.recv() {
                Self::handle_event(event);
            }
        });
        
        tracing::info!("Watching resource path: {}", path);
        
        Ok(())
    }
    
    /// Handle file system event
    fn handle_event(event: Event) {
        match event.kind {
            EventKind::Modify(_) => {
                tracing::debug!("Resource file modified, triggering hot-reload");
                // TODO: Trigger hot-reload callback
            }
            EventKind::Create(_) => {
                tracing::debug!("New resource file created");
            }
            EventKind::Remove(_) => {
                tracing::debug!("Resource file removed");
            }
            _ => {}
        }
    }
    
    /// Stop watching
    pub fn unwatch(&mut self, path: &str) {
        if let Some(ref mut watcher) = *self.watcher.write() {
            let _ = watcher.unwatch(Path::new(path));
            self.watched_paths.write().retain(|p| p != path);
        }
    }
}

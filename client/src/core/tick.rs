//! # Tick System
//!
//! Main game loop coordination.

use std::time::{Duration, Instant};

/// Tick manager for frame timing
pub struct TickManager {
    last_tick: Instant,
    target_fps: u32,
    delta_ms: u64,
}

impl TickManager {
    /// Create a new tick manager
    pub fn new(target_fps: u32) -> Self {
        Self {
            last_tick: Instant::now(),
            target_fps,
            delta_ms: 0,
        }
    }
    
    /// Wait for next tick
    pub fn wait_next(&mut self) {
        let frame_duration = Duration::from_secs_f64(1.0 / self.target_fps as f64);
        let elapsed = self.last_tick.elapsed();
        
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
        
        self.delta_ms = self.last_tick.elapsed().as_millis() as u64;
        self.last_tick = Instant::now();
    }
    
    /// Get delta time in milliseconds
    pub fn delta_ms(&self) -> u64 {
        self.delta_ms
    }
}

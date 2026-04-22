//! # Client Core
//!
//! Central client state management and initialization.

pub mod context;
pub mod events;
pub mod tick;

pub use context::Client;
pub use context::ClientConfig;

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

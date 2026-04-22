//! # Network Module
//!
//! Network connection management and protocols.

pub mod manager;
pub mod protocols;
pub mod packets;

pub use manager::NetworkManager;

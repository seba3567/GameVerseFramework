//! # GameVerse Client Library
//!
//! Entry point for the GameVerse client library.
//! This allows embedding the client in other applications.

pub mod core;
pub mod network;
pub mod scripting;
pub mod ui;
pub mod resources;
pub mod game;
pub mod launcher;

pub use crate::core::{Client, ClientConfig};
pub use crate::launcher::{GameLauncher, LauncherConfig};

use anyhow::Result;

/// Initialize and run the client
pub async fn run(config: ClientConfig) -> Result<()> {
    let client = Client::new(config)?;
    client.run().await
}

/// Launch game and connect to server
pub async fn launch_game(config: LauncherConfig) -> Result<u32> {
    let launcher = GameLauncher::new(config)?;
    launcher.launch().await
}

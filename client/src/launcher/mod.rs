//! # GameVerse Game Launcher
//!
//! Automatically launches GTA V / RDR2 with GameVerse injection.

pub mod injector;
pub mod game_detector;
pub mod launcher;

pub use launcher::GameLauncher;

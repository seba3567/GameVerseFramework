//! # GameVerse Client
//!
//! Main executable entry point for the GameVerse client.

mod core;
mod network;
mod scripting;
mod ui;
mod resources;
mod game;
mod launcher;

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
#[command(name = "gameverse_client")]
#[command(about = "GameVerse Client - Connect to GameVerse servers")]
struct Args {
    /// Server address to connect to
    #[arg(short, long, default_value = "localhost")]
    server: String,
    
    /// Server port
    #[arg(short, long, default_value = "8080")]
    port: u16,
    
    /// Authentication token
    #[arg(short, long)]
    token: Option<String>,
    
    /// Resource path to load
    #[arg(short, long)]
    resource: Option<String>,
    
    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,
    
    /// Game to launch (gta5 or rdr3)
    #[arg(short, long, default_value = "gta5")]
    game: String,
    
    /// Launch game automatically (auto-inject and connect)
    #[arg(short, long)]
    launch: bool,
    
    /// Custom game path
    #[arg(short, long)]
    game_path: Option<String>,
    
    /// Skip intro videos
    #[arg(long)]
    skip_intro: bool,
    
    /// Windowed mode
    #[arg(long)]
    windowed: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));
    
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();
    
    let args = Args::parse();
    
    if args.launch {
        // Auto-launch mode: launch game, inject DLL, connect
        tracing::info!("🎮 GameVerse Client launching in auto mode...");
        
        let game_id = match args.game.to_lowercase().as_str() {
            "gta5" | "gta" => launcher::game_detector::GameId::GTAV,
            "rdr3" | "rdr" => launcher::game_detector::GameId::RDR3,
            _ => {
                tracing::error!("Unknown game: {}", args.game);
                anyhow::bail!("Unknown game: {}. Use 'gta5' or 'rdr3'", args.game);
            }
        };
        
        let config = launcher::LauncherConfig {
            game_id,
            server_address: args.server,
            server_port: args.port,
            auth_token: args.token,
            custom_path: args.game_path,
            injection_dll: None, // Auto-detect
            windowed: args.windowed,
            skip_intro: args.skip_intro,
        };
        
        let launcher = launcher::GameLauncher::new(config)?;
        let pid = launcher.launch().await?;
        
        tracing::info!("✅ Game launched with PID {}", pid);
        tracing::info!("GameVerse DLL injected and connecting to {}:{}", args.server, args.port);
        
        // Keep running until game exits
        tokio::signal::ctrl_c().await?;
        
        tracing::info!("Shutting down...");
        
    } else {
        // Normal mode: connect to existing server (for development)
        tracing::info!("🎮 GameVerse Client starting...");
        tracing::info!("Connecting to {}:{}", args.server, args.port);
        
        let config = core::ClientConfig {
            server_address: args.server,
            server_port: args.port,
            auth_token: args.token,
            resource_path: args.resource,
            debug: args.debug,
        };
        
        let client = core::Client::new(config)?;
        client.run().await?;
    }
    
    Ok(())
}

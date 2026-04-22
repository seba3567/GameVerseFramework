//! # GameVerse Client UI - Tauri Backend
//!
//! Tauri commands for the GameVerse client UI.

#![warn(missing_docs)]

use serde::{Deserialize, Serialize};
use tauri::Manager;

/// Launch configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchConfig {
    /// Server address
    pub server: String,
    /// Server port
    pub port: u16,
    /// Auth token
    pub token: Option<String>,
    /// Game ID (gta5 or rdr3)
    pub game_id: String,
    /// Windowed mode
    pub windowed: bool,
    /// Skip intro videos
    pub skip_intro: bool,
}

/// Connection status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStatus {
    /// Connected state
    pub connected: bool,
    /// Server name
    pub server_name: Option<String>,
    /// Player count
    pub players: Option<u32>,
    /// Max players
    pub max_players: Option<u32>,
    /// Ping in ms
    pub ping: Option<u32>,
}

/// Game info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameInfo {
    /// Game ID
    pub id: String,
    /// Game name
    pub name: String,
    /// Installation path
    pub path: Option<String>,
    /// Detected
    pub detected: bool,
}

/// Tauri commands
#[tauri::command]
pub async fn launch_game(config: LaunchConfig) -> Result<u32, String> {
    tracing::info!("Launching game: {:?}", config);
    // TODO: Call into gameverse-client library to launch
    Ok(0) // PID
}

#[tauri::command]
pub async fn inject_dll(pid: u32, dll_path: String) -> Result<bool, String> {
    tracing::info!("Injecting DLL into PID {}: {}", pid, dll_path);
    // TODO: Call into gameverse-client library to inject
    Ok(true)
}

#[tauri::command]
pub async fn get_connection_status() -> Result<ConnectionStatus, String> {
    Ok(ConnectionStatus {
        connected: false,
        server_name: None,
        players: None,
        max_players: None,
        ping: None,
    })
}

#[tauri::command]
pub async fn connect_to_server(config: LaunchConfig) -> Result<ConnectionStatus, String> {
    tracing::info!("Connecting to server: {}:{}", config.server, config.port);
    // TODO: Call into gameverse-client library to connect
    Ok(ConnectionStatus {
        connected: true,
        server_name: Some(config.server.clone()),
        players: Some(0),
        max_players: Some(64),
        ping: Some(0),
    })
}

#[tauri::command]
pub async fn disconnect() -> Result<(), String> {
    tracing::info!("Disconnecting from server");
    // TODO: Call into gameverse-client library to disconnect
    Ok(())
}

#[tauri::command]
pub async fn detect_games() -> Result<Vec<GameInfo>, String> {
    tracing::info!("Detecting installed games");
    // TODO: Call into gameverse-client library to detect games
    Ok(vec![
        GameInfo {
            id: "gta5".to_string(),
            name: "Grand Theft Auto V".to_string(),
            path: None,
            detected: false,
        },
        GameInfo {
            id: "rdr3".to_string(),
            name: "Red Dead Redemption 2".to_string(),
            path: None,
            detected: false,
        },
    ])
}

#[tauri::command]
pub async fn get_game_path(game_id: String) -> Result<Option<String>, String> {
    tracing::info!("Getting game path for: {}", game_id);
    // TODO: Call into gameverse-client library to detect game path
    Ok(None)
}

/// Initialize Tauri application
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_process::init())
        .invoke_handler(tauri::generate_handler![
            launch_game,
            inject_dll,
            get_connection_status,
            connect_to_server,
            disconnect,
            detect_games,
            get_game_path,
        ])
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            #[cfg(debug_assertions)]
            window.open_devtools();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

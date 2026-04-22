//! # Game Launcher
//!
//! Automatic game launcher with injection.

use anyhow::Result;
use std::path::PathBuf;
use std::sync::Arc;
use parking_lot::RwLock;

use super::game_detector::{GameDetector, GameInstall, GameId};
use super::injector::{Injector, InjectionStatus};

/// Launcher configuration
#[derive(Debug, Clone)]
pub struct LauncherConfig {
    /// Game to launch
    pub game_id: GameId,
    /// Server to connect to
    pub server_address: String,
    /// Server port
    pub server_port: u16,
    /// Auth token
    pub auth_token: Option<String>,
    /// Custom game path (optional)
    pub custom_path: Option<String>,
    /// DLL to inject (auto-detected if not provided)
    pub injection_dll: Option<String>,
    /// Launch in windowed mode
    pub windowed: bool,
    /// Skip intro videos
    pub skip_intro: bool,
}

/// Launcher state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LauncherState {
    Idle,
    DetectingGame,
    StartingGame,
    Injecting,
    WaitingForInjection,
    Connected,
    Error,
}

/// Main game launcher
pub struct GameLauncher {
    config: LauncherConfig,
    detector: Arc<GameDetector>,
    injector: Arc<Injector>,
    state: RwLock<LauncherState>,
    game_install: RwLock<Option<GameInstall>>,
}

impl GameLauncher {
    /// Create a new game launcher
    pub fn new(config: LauncherConfig) -> Result<Self> {
        Ok(Self {
            config,
            detector: Arc::new(GameDetector::new()),
            injector: Arc::new(Injector::new()),
            state: RwLock::new(LauncherState::Idle),
            game_install: RwLock::new(None),
        })
    }
    
    /// Launch game with automatic injection
    pub async fn launch(&self) -> Result<u32> {
        *self.state.write() = LauncherState::DetectingGame;
        
        // 1. Detect game
        let install = self.detect_game()?;
        *self.game_install.write() = Some(install.clone());
        
        // 2. Prepare injection
        let dll_path = self.prepare_injection(&install)?;
        
        // 3. Start game
        *self.state.write() = LauncherState::StartingGame;
        let pid = self.start_game(&install)?;
        
        // 4. Inject DLL
        *self.state.write() = LauncherState::Injecting;
        std::thread::sleep(std::time::Duration::from_secs(3)); // Wait for game init
        
        self.injector.inject_by_pid(pid)?;
        
        *self.state.write() = LauncherState::WaitingForInjection;
        tracing::info!("Waiting for DLL to initialize...");
        
        // Wait for injection to complete
        loop {
            match self.injector.status() {
                InjectionStatus::Injected => break,
                InjectionStatus::Failed => {
                    *self.state.write() = LauncherState::Error;
                    anyhow::bail!("Injection failed");
                }
                _ => {}
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        
        *self.state.write() = LauncherState::Connected;
        tracing::info!("Game launched and injected successfully!");
        
        Ok(pid)
    }
    
    /// Detect game installation
    fn detect_game(&self) -> Result<GameInstall> {
        // Check custom path first
        if let Some(ref path) = self.config.custom_path {
            let game_name = match self.config.game_id {
                GameId::GTAV => "Grand Theft Auto V",
                GameId::RDR3 => "Red Dead Redemption 2",
            };
            
            let exe = format!("{}\\GTA5.exe", path);
            if std::path::Path::new(&exe).exists() {
                return Ok(GameInstall {
                    id: self.config.game_id,
                    name: game_name.to_string(),
                    path: path.clone(),
                    exe,
                    game_version: "1.0".to_string(),
                });
            }
        }
        
        // Auto-detect
        let games = self.detector.detect()?;
        
        games
            .into_iter()
            .find(|g| g.id == self.config.game_id)
            .ok_or_else(|| anyhow::anyhow!("Game {} not found", self.config.game_id))
    }
    
    /// Prepare DLL for injection
    fn prepare_injection(&self, _install: &GameInstall) -> Result<String> {
        // Get DLL path
        let dll_path = if let Some(ref dll) = self.config.injection_dll {
            dll.clone()
        } else {
            // Look for game_hook.dll next to executable
            let exe_dir = std::path::Path::new(&install.path)
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_default();
            
            let dll = exe_dir.join("game_hook.dll");
            if dll.exists() {
                dll.to_string_lossy().to_string()
            } else {
                // Look in current directory
                let current_dll = std::env::current_dir()?
                    .join("game_hook.dll");
                
                if current_dll.exists() {
                    current_dll.to_string_lossy().to_string()
                } else {
                    anyhow::bail!("game_hook.dll not found. Build the game injection layer first.")
                }
            }
        };
        
        tracing::info!("Using injection DLL: {}", dll_path);
        self.injector.set_dll(&dll_path);
        
        Ok(dll_path)
    }
    
    /// Start the game process
    fn start_game(&self, install: &GameInstall) -> Result<u32> {
        let mut args = vec![
            "-noBrowser".to_string(),
            "-skipPrivacy".to_string(),
        ];
        
        if self.config.windowed {
            args.push("-windowed".to_string());
        }
        
        if self.config.skip_intro {
            args.push("-skipintro".to_string());
        }
        
        // Add GameVerse connection info as environment/data
        let server_info = format!("{}:{}", self.config.server_address, self.config.server_port);
        
        tracing::info!("Starting game: {}", install.exe);
        
        let child = std::process::Command::new(&install.exe)
            .args(&args)
            .spawn()?;
        
        let pid = child.id();
        
        tracing::info!("Game started with PID {}", pid);
        
        Ok(pid)
    }
    
    /// Get launcher state
    pub fn state(&self) -> LauncherState {
        *self.state.read()
    }
    
    /// Get detected game info
    pub fn game_install(&self) -> Option<GameInstall> {
        self.game_install.read().clone()
    }
}

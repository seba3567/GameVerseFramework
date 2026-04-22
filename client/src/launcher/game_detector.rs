//! # Game Detector
//!
//! Detects installed games and their paths.

use anyhow::Result;
use std::collections::HashMap;
use parking_lot::RwLock;

/// Supported games
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameId {
    GTAV,
    RDR3,
}

/// Game installation info
#[derive(Debug, Clone)]
pub struct GameInstall {
    pub id: GameId,
    pub name: String,
    pub path: String,
    pub exe: String,
    pub game_version: String,
}

/// Game registry/store detector
pub struct GameDetector {
    installs: RwLock<HashMap<GameId, GameInstall>>,
}

impl GameDetector {
    /// Create a new game detector
    pub fn new() -> Self {
        Self {
            installs: RwLock::new(HashMap::new()),
        }
    }
    
    /// Auto-detect installed games
    pub fn detect(&self) -> Result<Vec<GameInstall>> {
        let mut found = Vec::new();
        
        // Check Steam
        if let Some(install) = self.detect_steam_game(GameId::GTAV) {
            tracing::info!("Found GTA V (Steam) at: {}", install.path);
            found.push(install);
        }
        
        // Check Epic Games
        if let Some(install) = self.detect_epic_game(GameId::GTAV) {
            tracing::info!("Found GTA V (Epic) at: {}", install.path);
            found.push(install);
        }
        
        // Check Rockstar
        if let Some(install) = self.detect_rockstar_game(GameId::RDR3) {
            tracing::info!("Found RDR3 (Rockstar) at: {}", install.path);
            found.push(install);
        }
        
        Ok(found)
    }
    
    /// Detect Steam game
    fn detect_steam_game(&self, game_id: GameId) -> Option<GameInstall> {
        let steam_ids = HashMap::from([
            (GameId::GTAV, 2715900u32), // GTA V Steam
        ]);
        
        let game_name = match game_id {
            GameId::GTAV => "Grand Theft Auto V",
            GameId::RDR3 => "Red Dead Redemption 2",
        };
        
        // Try default Steam paths
        let paths = vec![
            format!("C:\\Program Files (x86)\\Steam\\steamapps\\common\\{}", game_name),
            format!("C:\\Program Files\\Steam\\steamapps\\common\\{}", game_name),
        ];
        
        for path in paths {
            let gta5_exe = format!("{}\\{}.exe", path, game_name.split_whitespace().collect::<Vec<_>>().join(""));
            if std::path::Path::new(&gta5_exe).exists() {
                return Some(GameInstall {
                    id: game_id,
                    name: game_name.to_string(),
                    path,
                    exe: gta5_exe,
                    game_version: "1.0".to_string(),
                });
            }
        }
        
        None
    }
    
    /// Detect Epic Games game
    fn detect_epic_game(&self, game_id: GameId) -> Option<GameInstall> {
        let game_name = match game_id {
            GameId::GTAV => "Grand Theft Auto V",
            GameId::RDR3 => "Red Dead Redemption 2",
        };
        
        let path = format!("C:\\Program Files\\Epic Games\\{}", game_name);
        let exe = format!("{}\\GTA5.exe", path);
        
        if std::path::Path::new(&exe).exists() {
            Some(GameInstall {
                id: game_id,
                name: game_name.to_string(),
                path,
                exe,
                game_version: "1.0".to_string(),
            })
        } else {
            None
        }
    }
    
    /// Detect Rockstar Games game
    fn detect_rockstar_game(&self, game_id: GameId) -> Option<GameInstall> {
        let game_name = match game_id {
            GameId::GTAV => "Grand Theft Auto V",
            GameId::RDR3 => "Red Dead Redemption 2",
        };
        
        let paths = vec![
            format!("C:\\Program Files\\Rockstar Games\\{}", game_name),
            format!("C:\\Games\\{}", game_name),
        ];
        
        for path in &paths {
            let exe = format!("{}\\GTA5.exe", path);
            if std::path::Path::new(&exe).exists() {
                return Some(GameInstall {
                    id: game_id,
                    name: game_name.to_string(),
                    path: path.clone(),
                    exe,
                    game_version: "1.0".to_string(),
                });
            }
        }
        
        None
    }
    
    /// Get game by ID
    pub fn get_game(&self, game_id: GameId) -> Option<GameInstall> {
        self.installs.read().get(&game_id).cloned()
    }
}

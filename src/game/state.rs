//! Game state management and serialization

use serde::{Deserialize, Serialize};
use crate::game::entities::AircraftType;
use std::collections::{HashMap, HashSet};

/// Upgrade identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UpgradeId(pub u32);

/// Complete game state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameState {
    pub current_run: Option<RunState>,
    pub meta_progression: MetaProgression,
    pub settings: GameSettings,
    pub statistics: GameStatistics,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            current_run: None,
            meta_progression: MetaProgression::new(),
            settings: GameSettings::default(),
            statistics: GameStatistics::new(),
        }
    }
    
    pub fn serialize_to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
    
    pub fn deserialize_from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

/// Current run state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunState {
    pub seed: u64,
    pub aircraft: AircraftType,
    pub zone: u32,
    pub score: u64,
    pub time_elapsed: f32,
    pub current_health: i32,
    pub max_health: i32,
}

impl RunState {
    pub fn new(seed: u64, aircraft: AircraftType) -> Self {
        RunState {
            seed,
            aircraft,
            zone: 0,
            score: 0,
            time_elapsed: 0.0,
            current_health: 100,
            max_health: 100,
        }
    }
}

/// Meta-progression system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaProgression {
    pub squadron_xp: u32,
    pub squadron_level: u32,
    pub unlocked_aircraft: HashSet<AircraftType>,
    pub total_score: u64,
    pub total_runs: u32,
}

impl MetaProgression {
    pub fn new() -> Self {
        let mut unlocked = HashSet::new();
        unlocked.insert(AircraftType::Spitfire);
        
        MetaProgression {
            squadron_xp: 0,
            squadron_level: 1,
            unlocked_aircraft: unlocked,
            total_score: 0,
            total_runs: 0,
        }
    }
    
    pub fn add_xp(&mut self, amount: u32) {
        self.squadron_xp += amount;
        self.check_level_up();
    }
    
    fn check_level_up(&mut self) {
        let required_xp = self.squadron_level * 1000;
        if self.squadron_xp >= required_xp {
            self.squadron_xp -= required_xp;
            self.squadron_level += 1;
        }
    }
    
    pub fn unlock_aircraft(&mut self, aircraft: AircraftType) {
        self.unlocked_aircraft.insert(aircraft);
    }
    
    pub fn is_aircraft_unlocked(&self, aircraft: AircraftType) -> bool {
        self.unlocked_aircraft.contains(&aircraft)
    }
}

/// Game settings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameSettings {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub graphics_quality: GraphicsQuality,
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            master_volume: 0.8,
            music_volume: 0.7,
            sfx_volume: 0.9,
            graphics_quality: GraphicsQuality::High,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GraphicsQuality {
    Low,
    Medium,
    High,
    Ultra,
}

/// Game statistics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameStatistics {
    pub total_playtime: f32,
    pub enemies_defeated: u32,
    pub highest_score: u64,
    pub highest_zone: u32,
}

impl GameStatistics {
    pub fn new() -> Self {
        GameStatistics {
            total_playtime: 0.0,
            enemies_defeated: 0,
            highest_score: 0,
            highest_zone: 0,
        }
    }
    
    pub fn update_from_run(&mut self, run: &RunState) {
        if run.score > self.highest_score {
            self.highest_score = run.score;
        }
        if run.zone > self.highest_zone {
            self.highest_zone = run.zone;
        }
        self.total_playtime += run.time_elapsed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[test]
    fn test_game_state_creation() {
        let state = GameState::new();
        assert!(state.current_run.is_none());
        assert_eq!(state.meta_progression.squadron_level, 1);
    }
    
    #[wasm_bindgen_test]
    fn test_game_state_creation_wasm() {
        let state = GameState::new();
        assert!(state.current_run.is_none());
    }
    
    #[test]
    fn test_game_state_serialization() {
        let state = GameState::new();
        let json = state.serialize_to_json().unwrap();
        assert!(!json.is_empty());
    }
    
    #[wasm_bindgen_test]
    fn test_game_state_serialization_wasm() {
        let state = GameState::new();
        let json = state.serialize_to_json().unwrap();
        assert!(!json.is_empty());
    }
    
    #[test]
    fn test_game_state_deserialization() {
        let original = GameState::new();
        let json = original.serialize_to_json().unwrap();
        let deserialized = GameState::deserialize_from_json(&json).unwrap();
        
        assert_eq!(original, deserialized);
    }
    
    #[wasm_bindgen_test]
    fn test_game_state_deserialization_wasm() {
        let original = GameState::new();
        let json = original.serialize_to_json().unwrap();
        let deserialized = GameState::deserialize_from_json(&json).unwrap();
        
        assert_eq!(original, deserialized);
    }
    
    #[test]
    fn test_run_state_creation() {
        let run = RunState::new(12345, AircraftType::Spitfire);
        assert_eq!(run.seed, 12345);
        assert_eq!(run.aircraft, AircraftType::Spitfire);
        assert_eq!(run.score, 0);
    }
    
    #[wasm_bindgen_test]
    fn test_run_state_creation_wasm() {
        let run = RunState::new(12345, AircraftType::Spitfire);
        assert_eq!(run.seed, 12345);
    }
    
    #[test]
    fn test_meta_progression_xp() {
        let mut meta = MetaProgression::new();
        assert_eq!(meta.squadron_level, 1);
        
        meta.add_xp(1000);
        assert_eq!(meta.squadron_level, 2);
        assert_eq!(meta.squadron_xp, 0);
    }
    
    #[wasm_bindgen_test]
    fn test_meta_progression_xp_wasm() {
        let mut meta = MetaProgression::new();
        meta.add_xp(1000);
        assert_eq!(meta.squadron_level, 2);
    }
    
    #[test]
    fn test_aircraft_unlocking() {
        let mut meta = MetaProgression::new();
        assert!(meta.is_aircraft_unlocked(AircraftType::Spitfire));
        assert!(!meta.is_aircraft_unlocked(AircraftType::Mustang));
        
        meta.unlock_aircraft(AircraftType::Mustang);
        assert!(meta.is_aircraft_unlocked(AircraftType::Mustang));
    }
    
    #[wasm_bindgen_test]
    fn test_aircraft_unlocking_wasm() {
        let mut meta = MetaProgression::new();
        assert!(meta.is_aircraft_unlocked(AircraftType::Spitfire));
        
        meta.unlock_aircraft(AircraftType::Mustang);
        assert!(meta.is_aircraft_unlocked(AircraftType::Mustang));
    }
    
    #[test]
    fn test_statistics_update() {
        let mut stats = GameStatistics::new();
        let run = RunState {
            seed: 123,
            aircraft: AircraftType::Spitfire,
            zone: 5,
            score: 1000,
            time_elapsed: 120.0,
            current_health: 50,
            max_health: 100,
        };
        
        stats.update_from_run(&run);
        assert_eq!(stats.highest_score, 1000);
        assert_eq!(stats.highest_zone, 5);
        assert_eq!(stats.total_playtime, 120.0);
    }
    
    #[wasm_bindgen_test]
    fn test_statistics_update_wasm() {
        let mut stats = GameStatistics::new();
        let run = RunState {
            seed: 123,
            aircraft: AircraftType::Spitfire,
            zone: 5,
            score: 1000,
            time_elapsed: 120.0,
            current_health: 50,
            max_health: 100,
        };
        
        stats.update_from_run(&run);
        assert_eq!(stats.highest_score, 1000);
    }
    
    #[test]
    fn test_complete_serialization_cycle() {
        let mut state = GameState::new();
        state.current_run = Some(RunState::new(42, AircraftType::Mustang));
        state.meta_progression.add_xp(500);
        
        let json = state.serialize_to_json().unwrap();
        let restored = GameState::deserialize_from_json(&json).unwrap();
        
        assert_eq!(state, restored);
    }
    
    #[wasm_bindgen_test]
    fn test_complete_serialization_cycle_wasm() {
        let mut state = GameState::new();
        state.current_run = Some(RunState::new(42, AircraftType::Mustang));
        
        let json = state.serialize_to_json().unwrap();
        let restored = GameState::deserialize_from_json(&json).unwrap();
        
        assert_eq!(state, restored);
    }
}

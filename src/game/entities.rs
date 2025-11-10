//! Entity definitions and management

use serde::{Deserialize, Serialize};

/// Entity identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Entity {
    pub id: u32,
    pub generation: u32,
}

impl Entity {
    pub fn new(id: u32) -> Self {
        Self { id, generation: 0 }
    }
}

/// Aircraft types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AircraftType {
    Spitfire,
    Mustang,
    Corsair,
    Thunderbolt,
    Lightning,
}

/// Enemy types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnemyType {
    Fighter,
    Bomber,
    Ace,
    Kamikaze,
    HeavyBomber,
}

/// Projectile owner (player or enemy)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProjectileOwner {
    Player,
    Enemy,
}

use crate::game::entities::{Entity, ProjectileOwner};
use crate::utils::Vec2;
use cgmath::InnerSpace;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WeaponId(pub u32);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponSystem {
    weapons: HashMap<WeaponId, WeaponDefinition>,
    upgrades: HashMap<WeaponId, Vec<WeaponUpgrade>>,
}

impl WeaponSystem {
    pub fn new() -> Self {
        Self {
            weapons: HashMap::new(),
            upgrades: HashMap::new(),
        }
    }

    pub fn register_weapon(&mut self, weapon: WeaponDefinition) {
        self.weapons.insert(weapon.id, weapon);
    }

    pub fn get_weapon(&self, id: WeaponId) -> Option<&WeaponDefinition> {
        self.weapons.get(&id)
    }

    pub fn apply_upgrade(&mut self, weapon_id: WeaponId, upgrade: WeaponUpgrade) {
        // Apply upgrade to weapon definition first
        if let Some(weapon) = self.weapons.get_mut(&weapon_id) {
            weapon.apply_upgrade(&upgrade);
        }

        // Store the upgrade in history
        self.upgrades
            .entry(weapon_id)
            .or_insert_with(Vec::new)
            .push(upgrade);
    }

    pub fn fire(
        &self,
        weapon_id: WeaponId,
        origin: Vec2,
        direction: Vec2,
        owner: ProjectileOwner,
    ) -> Vec<Projectile> {
        if let Some(weapon) = self.weapons.get(&weapon_id) {
            let pattern = self.calculate_spread(&weapon.spread_pattern, direction);

            pattern
                .into_iter()
                .map(|dir| Projectile {
                    position: origin,
                    velocity: dir * weapon.projectile_speed,
                    damage: weapon.base_damage,
                    projectile_type: weapon.projectile_type.clone(),
                    owner,
                    lifetime: 5.0,
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    fn calculate_spread(&self, pattern: &SpreadPattern, direction: Vec2) -> Vec<Vec2> {
        match pattern {
            SpreadPattern::Single => vec![direction.normalize()],
            SpreadPattern::Twin { spacing } => {
                let perpendicular = Vec2::new(-direction.y, direction.x).normalize();
                vec![
                    (direction + perpendicular * spacing).normalize(),
                    (direction - perpendicular * spacing).normalize(),
                ]
            }
            SpreadPattern::Spread { count, angle } => {
                let mut directions = Vec::new();
                let angle_rad = angle.to_radians();
                let step = angle_rad / (*count as f32 - 1.0);
                let start_angle = -angle_rad / 2.0;

                for i in 0..*count {
                    let offset_angle = start_angle + step * i as f32;
                    let rotated = rotate_vector(direction, offset_angle);
                    directions.push(rotated.normalize());
                }

                directions
            }
            SpreadPattern::Circle { count } => {
                let mut directions = Vec::new();
                let angle_step = 2.0 * std::f32::consts::PI / *count as f32;

                for i in 0..*count {
                    let angle = angle_step * i as f32;
                    directions.push(Vec2::new(angle.cos(), angle.sin()));
                }

                directions
            }
            SpreadPattern::Custom(func) => func(direction),
        }
    }
}

impl Default for WeaponSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponDefinition {
    pub id: WeaponId,
    pub name: String,
    pub base_damage: f32,
    pub fire_rate: f32,
    pub projectile_speed: f32,
    pub projectile_type: ProjectileType,
    pub spread_pattern: SpreadPattern,
    pub ammo_consumption: Option<u32>,
}

impl WeaponDefinition {
    pub fn apply_upgrade(&mut self, upgrade: &WeaponUpgrade) {
        self.base_damage *= upgrade.damage_multiplier;
        self.fire_rate *= upgrade.fire_rate_multiplier;
        self.projectile_speed *= upgrade.speed_multiplier;

        // Apply spread modifications
        if let Some(new_pattern) = &upgrade.new_spread_pattern {
            self.spread_pattern = new_pattern.clone();
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpreadPattern {
    Single,
    Twin { spacing: f32 },
    Spread { count: u32, angle: f32 },
    Circle { count: u32 },
    #[serde(skip)]
    Custom(fn(Vec2) -> Vec<Vec2>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponUpgrade {
    pub name: String,
    pub damage_multiplier: f32,
    pub fire_rate_multiplier: f32,
    pub speed_multiplier: f32,
    pub new_spread_pattern: Option<SpreadPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectileType {
    Bullet,
    Missile,
    Laser,
    Bomb,
    Rocket,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Projectile {
    pub position: Vec2,
    pub velocity: Vec2,
    pub damage: f32,
    pub projectile_type: ProjectileType,
    pub owner: ProjectileOwner,
    pub lifetime: f32,
}

impl Projectile {
    pub fn update(&mut self, delta: f32) {
        self.position += self.velocity * delta;
        self.lifetime -= delta;
    }

    pub fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }
}

fn rotate_vector(v: Vec2, angle: f32) -> Vec2 {
    let cos = angle.cos();
    let sin = angle.sin();
    Vec2::new(v.x * cos - v.y * sin, v.x * sin + v.y * cos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weapon_system_creation() {
        let mut system = WeaponSystem::new();
        let weapon = WeaponDefinition {
            id: WeaponId(1),
            name: "Machine Gun".to_string(),
            base_damage: 10.0,
            fire_rate: 10.0,
            projectile_speed: 500.0,
            projectile_type: ProjectileType::Bullet,
            spread_pattern: SpreadPattern::Single,
            ammo_consumption: None,
        };

        system.register_weapon(weapon);
        assert!(system.get_weapon(WeaponId(1)).is_some());
    }

    #[test]
    fn test_weapon_fire_single() {
        let mut system = WeaponSystem::new();
        let weapon = WeaponDefinition {
            id: WeaponId(1),
            name: "Single Gun".to_string(),
            base_damage: 10.0,
            fire_rate: 5.0,
            projectile_speed: 100.0,
            projectile_type: ProjectileType::Bullet,
            spread_pattern: SpreadPattern::Single,
            ammo_consumption: None,
        };

        system.register_weapon(weapon);

        let projectiles = system.fire(
            WeaponId(1),
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 1.0),
            ProjectileOwner::Player,
        );

        assert_eq!(projectiles.len(), 1);
        assert_eq!(projectiles[0].damage, 10.0);
    }

    #[test]
    fn test_weapon_fire_spread() {
        let mut system = WeaponSystem::new();
        let weapon = WeaponDefinition {
            id: WeaponId(1),
            name: "Spread Gun".to_string(),
            base_damage: 5.0,
            fire_rate: 3.0,
            projectile_speed: 100.0,
            projectile_type: ProjectileType::Bullet,
            spread_pattern: SpreadPattern::Spread {
                count: 3,
                angle: 30.0,
            },
            ammo_consumption: None,
        };

        system.register_weapon(weapon);

        let projectiles = system.fire(
            WeaponId(1),
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 1.0),
            ProjectileOwner::Player,
        );

        assert_eq!(projectiles.len(), 3);
    }

    #[test]
    fn test_weapon_upgrade() {
        let mut system = WeaponSystem::new();
        let weapon = WeaponDefinition {
            id: WeaponId(1),
            name: "Base Gun".to_string(),
            base_damage: 10.0,
            fire_rate: 5.0,
            projectile_speed: 100.0,
            projectile_type: ProjectileType::Bullet,
            spread_pattern: SpreadPattern::Single,
            ammo_consumption: None,
        };

        system.register_weapon(weapon);

        let upgrade = WeaponUpgrade {
            name: "Damage Boost".to_string(),
            damage_multiplier: 1.5,
            fire_rate_multiplier: 1.0,
            speed_multiplier: 1.0,
            new_spread_pattern: None,
        };

        system.apply_upgrade(WeaponId(1), upgrade);

        let weapon = system.get_weapon(WeaponId(1)).unwrap();
        assert_eq!(weapon.base_damage, 15.0);
    }

    #[test]
    fn test_projectile_update() {
        let mut projectile = Projectile {
            position: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(10.0, 0.0),
            damage: 10.0,
            projectile_type: ProjectileType::Bullet,
            owner: ProjectileOwner::Player,
            lifetime: 1.0,
        };

        projectile.update(0.1);

        assert_eq!(projectile.position.x, 1.0);
        assert_eq!(projectile.lifetime, 0.9);
        assert!(projectile.is_alive());

        projectile.update(1.0);
        assert!(!projectile.is_alive());
    }
}

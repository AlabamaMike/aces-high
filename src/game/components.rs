use crate::engine::webgl::TextureHandle;
use crate::game::entities::AircraftType;
use crate::utils::math::{Color, Vec2, AABB};
use serde::{Deserialize, Serialize};

/// Sprite component for rendering
#[derive(Debug, Clone)]
pub struct Sprite {
    pub texture: TextureHandle,
    pub rotation: f32,
    pub scale: Vec2,
    pub color: Color,
}

impl Sprite {
    pub fn new(texture: TextureHandle) -> Self {
        Self {
            texture,
            rotation: 0.0,
            scale: Vec2::new(1.0, 1.0),
            color: Color::white(),
        }
    }
}

/// Position component
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    pub fn from_vec2(v: Vec2) -> Self {
        Self { x: v.x, y: v.y }
    }
}

/// Velocity component
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

impl Velocity {
    pub fn new(dx: f32, dy: f32) -> Self {
        Self { dx, dy }
    }

    pub fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.dx, self.dy)
    }

    pub fn from_vec2(v: Vec2) -> Self {
        Self { dx: v.x, dy: v.y }
    }
}

/// Health component
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Health {
    pub current: i32,
    pub max: i32,
    pub armor: f32,
}

impl Health {
    pub fn new(max: i32) -> Self {
        Self {
            current: max,
            max,
            armor: 0.0,
        }
    }

    pub fn with_armor(max: i32, armor: f32) -> Self {
        Self {
            current: max,
            max,
            armor,
        }
    }

    pub fn take_damage(&mut self, damage: f32) {
        let actual_damage = (damage * (1.0 - self.armor)) as i32;
        self.current = (self.current - actual_damage).max(0);
    }

    pub fn heal(&mut self, amount: i32) {
        self.current = (self.current + amount).min(self.max);
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0
    }
}

/// Collider component
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Collider {
    Circle { radius: f32 },
    AABB { width: f32, height: f32 },
}

impl Collider {
    pub fn circle(radius: f32) -> Self {
        Self::Circle { radius }
    }

    pub fn aabb(width: f32, height: f32) -> Self {
        Self::AABB { width, height }
    }

    pub fn get_aabb(&self, position: &Position) -> AABB {
        match self {
            Collider::Circle { radius } => {
                AABB::from_center_size(
                    position.as_vec2(),
                    Vec2::new(*radius * 2.0, *radius * 2.0),
                )
            }
            Collider::AABB { width, height } => {
                AABB::from_center_size(position.as_vec2(), Vec2::new(*width, *height))
            }
        }
    }
}

/// Aircraft component
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Aircraft {
    pub aircraft_type: AircraftType,
    pub level: u8,
    pub experience: u32,
}

impl Aircraft {
    pub fn new(aircraft_type: AircraftType) -> Self {
        Self {
            aircraft_type,
            level: 1,
            experience: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_damage() {
        let mut health = Health::new(100);
        health.take_damage(30.0);
        assert_eq!(health.current, 70);
        assert!(health.is_alive());

        health.take_damage(100.0);
        assert_eq!(health.current, 0);
        assert!(!health.is_alive());
    }

    #[test]
    fn test_health_armor() {
        let mut health = Health::with_armor(100, 0.5);
        health.take_damage(40.0);
        assert_eq!(health.current, 80);
    }

    #[test]
    fn test_health_heal() {
        let mut health = Health::new(100);
        health.take_damage(50.0);
        health.heal(30);
        assert_eq!(health.current, 80);

        health.heal(50);
        assert_eq!(health.current, 100);
    }
}

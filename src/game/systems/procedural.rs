use crate::game::entities::EnemyType;
use crate::game::systems::ai::{AIBehavior, Formation, Path, WavePattern};
use crate::utils::Vec2;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use serde::{Deserialize, Serialize};

pub struct ProceduralGenerator {
    rng: StdRng,
    wave_templates: Vec<WaveTemplate>,
    terrain_generator: TerrainGenerator,
    difficulty_manager: DifficultyManager,
}

impl ProceduralGenerator {
    pub fn new(seed: u64) -> Self {
        let mut generator = Self {
            rng: StdRng::seed_from_u64(seed),
            wave_templates: Vec::new(),
            terrain_generator: TerrainGenerator::new(),
            difficulty_manager: DifficultyManager::new(),
        };

        generator.init_wave_templates();
        generator
    }

    fn init_wave_templates(&mut self) {
        // Basic fighter wave
        self.wave_templates.push(WaveTemplate {
            name: "Fighter Squadron".to_string(),
            enemy_types: vec![EnemyType::Fighter],
            formation: Formation::V { spacing: 50.0 },
            base_count: 5,
            min_difficulty: 0.0,
            max_difficulty: 1.0,
            zone_types: vec![ZoneType::Sky, ZoneType::Clouds],
        });

        // Bomber formation
        self.wave_templates.push(WaveTemplate {
            name: "Bomber Wing".to_string(),
            enemy_types: vec![EnemyType::Bomber],
            formation: Formation::Line {
                spacing: 80.0,
                angle: 0.0,
            },
            base_count: 3,
            min_difficulty: 0.3,
            max_difficulty: 1.0,
            zone_types: vec![ZoneType::Sky, ZoneType::Ocean],
        });

        // Mixed assault
        self.wave_templates.push(WaveTemplate {
            name: "Mixed Assault".to_string(),
            enemy_types: vec![EnemyType::Fighter, EnemyType::Bomber],
            formation: Formation::Diamond,
            base_count: 7,
            min_difficulty: 0.5,
            max_difficulty: 1.0,
            zone_types: vec![ZoneType::Sky, ZoneType::Clouds, ZoneType::Mountains],
        });

        // Ace encounter
        self.wave_templates.push(WaveTemplate {
            name: "Ace Patrol".to_string(),
            enemy_types: vec![EnemyType::Ace],
            formation: Formation::Circle { radius: 150.0 },
            base_count: 2,
            min_difficulty: 0.7,
            max_difficulty: 1.0,
            zone_types: vec![ZoneType::Sky, ZoneType::Clouds, ZoneType::Mountains],
        });

        // Kamikaze rush
        self.wave_templates.push(WaveTemplate {
            name: "Kamikaze Wave".to_string(),
            enemy_types: vec![EnemyType::Kamikaze],
            formation: Formation::V { spacing: 30.0 },
            base_count: 8,
            min_difficulty: 0.4,
            max_difficulty: 1.0,
            zone_types: vec![ZoneType::Ocean, ZoneType::Desert],
        });
    }

    pub fn generate_zone(&mut self, zone_type: ZoneType, zone_number: u32) -> Zone {
        let difficulty = self.difficulty_manager.calculate_difficulty(zone_number);
        let mut zone = Zone::new(zone_type, zone_number);

        // Generate terrain
        let terrain = self.terrain_generator.generate(&zone_type, &mut self.rng);
        zone.terrain = terrain;

        // Generate waves
        let wave_count = self.calculate_wave_count(difficulty);
        for i in 0..wave_count {
            let wave_difficulty = difficulty * (1.0 + i as f32 * 0.1);
            let wave = self.generate_wave(zone_type, wave_difficulty);
            zone.waves.push(wave);
        }

        // Add hazards
        let hazards = self.generate_hazards(&zone_type, difficulty);
        zone.hazards = hazards;

        // Place collectibles
        let collectibles = self.generate_collectibles(difficulty);
        zone.collectibles = collectibles;

        zone
    }

    fn calculate_wave_count(&self, difficulty: f32) -> u32 {
        (5.0 + difficulty * 5.0) as u32
    }

    pub fn generate_wave(&mut self, zone_type: ZoneType, difficulty: f32) -> Wave {
        // Filter valid templates
        let valid_indices: Vec<usize> = self
            .wave_templates
            .iter()
            .enumerate()
            .filter(|(_, t)| {
                t.min_difficulty <= difficulty
                    && t.max_difficulty >= difficulty
                    && t.zone_types.contains(&zone_type)
            })
            .map(|(i, _)| i)
            .collect();

        if valid_indices.is_empty() {
            return self.create_default_wave(difficulty);
        }

        // Select random template
        let template_idx = valid_indices[self.rng.gen_range(0..valid_indices.len())];
        let template = &self.wave_templates[template_idx];
        self.instantiate_wave(template, difficulty)
    }

    fn instantiate_wave(&mut self, template: &WaveTemplate, difficulty: f32) -> Wave {
        let enemy_count = (template.base_count as f32 * (1.0 + difficulty * 0.3)) as u32;

        // Select enemy type distribution
        let mut enemy_composition = Vec::new();
        for _ in 0..enemy_count {
            let enemy_type =
                template.enemy_types[self.rng.gen_range(0..template.enemy_types.len())];
            enemy_composition.push(enemy_type);
        }

        // Create spawn pattern
        let spawn_positions = self.generate_formation_positions(&template.formation, enemy_count);

        Wave {
            enemy_composition,
            spawn_positions,
            health_multiplier: 1.0 + difficulty * 0.2,
            damage_multiplier: 1.0 + difficulty * 0.15,
            speed_multiplier: 1.0 + difficulty * 0.1,
            spawn_delay: 0.5,
            has_elite: self.rng.gen_bool(difficulty as f64 * 0.3),
        }
    }

    fn create_default_wave(&mut self, difficulty: f32) -> Wave {
        Wave {
            enemy_composition: vec![EnemyType::Fighter; 3],
            spawn_positions: vec![
                Vec2::new(-50.0, -100.0),
                Vec2::new(0.0, -100.0),
                Vec2::new(50.0, -100.0),
            ],
            health_multiplier: 1.0 + difficulty * 0.2,
            damage_multiplier: 1.0 + difficulty * 0.15,
            speed_multiplier: 1.0 + difficulty * 0.1,
            spawn_delay: 0.5,
            has_elite: false,
        }
    }

    fn generate_formation_positions(&mut self, formation: &Formation, count: u32) -> Vec<Vec2> {
        let mut positions = Vec::new();

        match formation {
            Formation::V { spacing } => {
                for i in 0..count {
                    let row = i / 2;
                    let col = if i % 2 == 0 {
                        i as i32 / 2
                    } else {
                        -(i as i32 / 2) - 1
                    };
                    positions.push(Vec2::new(
                        col as f32 * spacing,
                        -row as f32 * spacing - 100.0,
                    ));
                }
            }

            Formation::Line { spacing, angle } => {
                let angle_rad = angle.to_radians();
                let dir = Vec2::new(angle_rad.cos(), angle_rad.sin());
                for i in 0..count {
                    let offset = (i as f32 - count as f32 / 2.0) * spacing;
                    positions.push(dir * offset + Vec2::new(0.0, -100.0));
                }
            }

            Formation::Circle { radius } => {
                let angle_step = 2.0 * std::f32::consts::PI / count as f32;
                for i in 0..count {
                    let angle = angle_step * i as f32;
                    positions.push(Vec2::new(
                        angle.cos() * radius,
                        angle.sin() * radius - 100.0,
                    ));
                }
            }

            Formation::Diamond => {
                let half = count / 2;
                for i in 0..count {
                    let x = if i < half {
                        i as f32 * 40.0
                    } else {
                        (count - i - 1) as f32 * 40.0
                    } - half as f32 * 20.0;
                    let y = -i as f32 * 40.0 - 100.0;
                    positions.push(Vec2::new(x, y));
                }
            }

            Formation::Custom(positions_template) => {
                positions.extend_from_slice(positions_template);
            }
        }

        positions
    }

    fn generate_hazards(&mut self, zone_type: &ZoneType, difficulty: f32) -> Vec<Hazard> {
        let mut hazards = Vec::new();
        let hazard_count = (difficulty * 5.0) as u32;

        for _ in 0..hazard_count {
            let hazard_type = match zone_type {
                ZoneType::Sky | ZoneType::Clouds => HazardType::Lightning,
                ZoneType::Ocean => HazardType::Waterspout,
                ZoneType::Mountains => HazardType::WindShear,
                ZoneType::Desert => HazardType::Sandstorm,
            };

            hazards.push(Hazard {
                hazard_type,
                position: Vec2::new(
                    self.rng.gen_range(-500.0..500.0),
                    self.rng.gen_range(-300.0..300.0),
                ),
                radius: 50.0,
                damage_per_second: 10.0 * (1.0 + difficulty),
            });
        }

        hazards
    }

    fn generate_collectibles(&mut self, difficulty: f32) -> Vec<Collectible> {
        let mut collectibles = Vec::new();
        let count = self.rng.gen_range(3..8);

        for _ in 0..count {
            let collectible_type = if self.rng.gen_bool(0.7) {
                CollectibleType::HealthPack
            } else if self.rng.gen_bool(0.5) {
                CollectibleType::Ammo
            } else {
                CollectibleType::PowerUp
            };

            collectibles.push(Collectible {
                collectible_type,
                position: Vec2::new(
                    self.rng.gen_range(-400.0..400.0),
                    self.rng.gen_range(-200.0..200.0),
                ),
                value: (10.0 * (1.0 + difficulty * 0.5)) as u32,
            });
        }

        collectibles
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ZoneType {
    Sky,
    Clouds,
    Ocean,
    Mountains,
    Desert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Zone {
    pub zone_type: ZoneType,
    pub zone_number: u32,
    pub terrain: Terrain,
    pub waves: Vec<Wave>,
    pub hazards: Vec<Hazard>,
    pub collectibles: Vec<Collectible>,
}

impl Zone {
    pub fn new(zone_type: ZoneType, zone_number: u32) -> Self {
        Self {
            zone_type,
            zone_number,
            terrain: Terrain::default(),
            waves: Vec::new(),
            hazards: Vec::new(),
            collectibles: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wave {
    pub enemy_composition: Vec<EnemyType>,
    pub spawn_positions: Vec<Vec2>,
    pub health_multiplier: f32,
    pub damage_multiplier: f32,
    pub speed_multiplier: f32,
    pub spawn_delay: f32,
    pub has_elite: bool,
}

#[derive(Debug, Clone)]
pub struct WaveTemplate {
    pub name: String,
    pub enemy_types: Vec<EnemyType>,
    pub formation: Formation,
    pub base_count: u32,
    pub min_difficulty: f32,
    pub max_difficulty: f32,
    pub zone_types: Vec<ZoneType>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Terrain {
    pub background_layers: Vec<TerrainLayer>,
    pub obstacles: Vec<Obstacle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainLayer {
    pub texture_name: String,
    pub scroll_speed: f32,
    pub parallax_factor: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Obstacle {
    pub position: Vec2,
    pub size: Vec2,
    pub damage_on_collision: f32,
}

pub struct TerrainGenerator {
    // Terrain generation state
}

impl TerrainGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate<R: Rng>(&self, zone_type: &ZoneType, _rng: &mut R) -> Terrain {
        let layers = match zone_type {
            ZoneType::Sky => vec![
                TerrainLayer {
                    texture_name: "sky_bg".to_string(),
                    scroll_speed: 10.0,
                    parallax_factor: 0.2,
                },
                TerrainLayer {
                    texture_name: "clouds_far".to_string(),
                    scroll_speed: 20.0,
                    parallax_factor: 0.5,
                },
            ],
            ZoneType::Clouds => vec![
                TerrainLayer {
                    texture_name: "cloud_layer_1".to_string(),
                    scroll_speed: 15.0,
                    parallax_factor: 0.3,
                },
                TerrainLayer {
                    texture_name: "cloud_layer_2".to_string(),
                    scroll_speed: 30.0,
                    parallax_factor: 0.6,
                },
            ],
            ZoneType::Ocean => vec![
                TerrainLayer {
                    texture_name: "ocean_bg".to_string(),
                    scroll_speed: 12.0,
                    parallax_factor: 0.25,
                },
                TerrainLayer {
                    texture_name: "waves".to_string(),
                    scroll_speed: 35.0,
                    parallax_factor: 0.7,
                },
            ],
            ZoneType::Mountains => vec![
                TerrainLayer {
                    texture_name: "mountain_far".to_string(),
                    scroll_speed: 8.0,
                    parallax_factor: 0.15,
                },
                TerrainLayer {
                    texture_name: "mountain_near".to_string(),
                    scroll_speed: 25.0,
                    parallax_factor: 0.5,
                },
            ],
            ZoneType::Desert => vec![
                TerrainLayer {
                    texture_name: "desert_bg".to_string(),
                    scroll_speed: 10.0,
                    parallax_factor: 0.2,
                },
                TerrainLayer {
                    texture_name: "sand_dunes".to_string(),
                    scroll_speed: 28.0,
                    parallax_factor: 0.6,
                },
            ],
        };

        Terrain {
            background_layers: layers,
            obstacles: Vec::new(),
        }
    }
}

impl Default for TerrainGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hazard {
    pub hazard_type: HazardType,
    pub position: Vec2,
    pub radius: f32,
    pub damage_per_second: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HazardType {
    Lightning,
    Waterspout,
    WindShear,
    Sandstorm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collectible {
    pub collectible_type: CollectibleType,
    pub position: Vec2,
    pub value: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CollectibleType {
    HealthPack,
    Ammo,
    PowerUp,
}

pub struct DifficultyManager {
    base_difficulty: f32,
}

impl DifficultyManager {
    pub fn new() -> Self {
        Self {
            base_difficulty: 0.1,
        }
    }

    pub fn calculate_difficulty(&self, zone_number: u32) -> f32 {
        // Exponential difficulty curve
        let zone_factor = zone_number as f32 * 0.15;
        (self.base_difficulty + zone_factor).min(1.0)
    }
}

impl Default for DifficultyManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_procedural_generator_creation() {
        let generator = ProceduralGenerator::new(12345);
        assert!(!generator.wave_templates.is_empty());
    }

    #[test]
    fn test_zone_generation() {
        let mut generator = ProceduralGenerator::new(12345);
        let zone = generator.generate_zone(ZoneType::Sky, 1);

        assert_eq!(zone.zone_type, ZoneType::Sky);
        assert_eq!(zone.zone_number, 1);
        assert!(!zone.waves.is_empty());
    }

    #[test]
    fn test_wave_generation() {
        let mut generator = ProceduralGenerator::new(12345);
        let wave = generator.generate_wave(ZoneType::Sky, 0.5);

        assert!(!wave.enemy_composition.is_empty());
        assert!(!wave.spawn_positions.is_empty());
        assert!(wave.health_multiplier > 1.0);
    }

    #[test]
    fn test_difficulty_scaling() {
        let difficulty_manager = DifficultyManager::new();

        let diff1 = difficulty_manager.calculate_difficulty(1);
        let diff5 = difficulty_manager.calculate_difficulty(5);
        let diff10 = difficulty_manager.calculate_difficulty(10);

        assert!(diff1 < diff5);
        assert!(diff5 < diff10);
        assert!(diff10 <= 1.0);
    }

    #[test]
    fn test_formation_positions() {
        let mut generator = ProceduralGenerator::new(12345);

        let v_formation = Formation::V { spacing: 50.0 };
        let positions = generator.generate_formation_positions(&v_formation, 5);
        assert_eq!(positions.len(), 5);

        let circle_formation = Formation::Circle { radius: 100.0 };
        let positions = generator.generate_formation_positions(&circle_formation, 8);
        assert_eq!(positions.len(), 8);
    }
}

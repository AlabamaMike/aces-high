# ACES HIGH: ENDLESS SKIES

## Technical Specification Document v1.0

### HD Roguelike WWII Arcade Shooter

-----

## Table of Contents

1. [Executive Summary](#executive-summary)
1. [Technology Stack](#technology-stack)
1. [Architecture Overview](#architecture-overview)
1. [Core Systems](#core-systems)
1. [Rendering Pipeline](#rendering-pipeline)
1. [Game Systems Implementation](#game-systems-implementation)
1. [Procedural Generation](#procedural-generation)
1. [Data Structures](#data-structures)
1. [Performance Specifications](#performance-specifications)
1. [Build & Deployment](#build--deployment)
1. [Testing Strategy](#testing-strategy)
1. [Development Timeline](#development-timeline)
1. [Appendices](#appendices)

-----

## Executive Summary

### Project Overview

**ACES HIGH: ENDLESS SKIES** is a WebAssembly-based roguelike arcade shooter featuring HD graphics, procedural generation, and meta-progression systems. The game targets 60 FPS performance at 1080p resolution on mid-range hardware while maintaining sub-50MB initial download size.

### Key Technical Requirements

- **Platform**: WebAssembly (WASM) running in modern browsers
- **Performance**: 60 FPS @ 1920x1080 on mid-range hardware
- **Language**: Rust with wasm-bindgen
- **Rendering**: WebGL 2.0 with fallback to WebGL 1.0
- **Audio**: Web Audio API with spatial sound
- **File Size**: <50MB initial load, progressive asset streaming
- **Load Time**: <3 seconds to playable state

-----

## Technology Stack

### Core Technologies

#### Primary Language: Rust

```toml
[dependencies]
wasm-bindgen = "0.2"
web-sys = "0.3"
js-sys = "0.3"
wasm-bindgen-futures = "0.4"
getrandom = { version = "0.2", features = ["js"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

#### Game Framework: Custom Engine with Libraries

```toml
# Graphics
glow = "0.13"  # OpenGL bindings for WebGL
cgmath = "0.18"  # Mathematics library
image = "0.24"  # Image loading and processing

# Audio
rodio = "0.17"  # Audio playback
lewton = "0.10"  # OGG Vorbis decoder

# Physics
rapier2d = "0.17"  # 2D physics engine

# Utilities
rand = "0.8"  # Random number generation
instant = "0.1"  # Time handling
```

### Web Technologies

- **WebGL 2.0**: Primary rendering API
- **Web Audio API**: Spatial audio and effects
- **WebAssembly**: Core game logic
- **IndexedDB**: Save data and asset caching
- **WebWorkers**: Background asset loading
- **WebRTC**: Future multiplayer support

### Development Tools

```yaml
Build System: 
  - wasm-pack 0.12+
  - webpack 5 / vite 5
  - npm/yarn for dependencies

Asset Pipeline:
  - Texture Packer for sprite atlases
  - Audacity for audio processing
  - Tiled for level templates
  - Custom tool for procedural generation testing

Testing:
  - wasm-bindgen-test for unit tests
  - Playwright for integration tests
  - Chrome DevTools for profiling
```

-----

## Architecture Overview

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Web Browser                          │
├─────────────────────────────────────────────────────────┤
│                    HTML/CSS/JS Shell                    │
├──────────────┬────────────────┬────────────────────────┤
│   WebGL 2.0  │  Web Audio API │     IndexedDB          │
├──────────────┴────────────────┴────────────────────────┤
│                  WASM Bridge Layer                      │
│                  (wasm-bindgen)                         │
├─────────────────────────────────────────────────────────┤
│                  Game Engine Core                       │
│  ┌──────────┬──────────┬──────────┬──────────┐        │
│  │ Renderer │  Audio   │  Input   │  Physics │        │
│  └──────────┴──────────┴──────────┴──────────┘        │
├─────────────────────────────────────────────────────────┤
│                   Game Logic                            │
│  ┌──────────┬──────────┬──────────┬──────────┐        │
│  │  Entity  │ Gameplay │ Procgen  │   Meta   │        │
│  │  System  │  Systems │  Engine  │   Game   │        │
│  └──────────┴──────────┴──────────┴──────────┘        │
└─────────────────────────────────────────────────────────┘
```

### Module Structure

```rust
// src/lib.rs - Main entry point
mod engine {
    pub mod renderer;
    pub mod audio;
    pub mod input;
    pub mod physics;
    pub mod resources;
}

mod game {
    pub mod entities;
    pub mod components;
    pub mod systems;
    pub mod procedural;
    pub mod progression;
}

mod utils {
    pub mod math;
    pub mod pool;
    pub mod random;
    pub mod performance;
}

mod web {
    pub mod bindings;
    pub mod storage;
    pub mod networking;
}
```

-----

## Core Systems

### 1. Entity Component System (ECS)

```rust
// Simplified ECS Architecture
pub struct World {
    entities: Vec<Entity>,
    components: ComponentStorage,
    systems: Vec<Box<dyn System>>,
    resources: Resources,
}

pub struct Entity {
    id: u32,
    generation: u32,
    components: BitSet,
}

pub trait Component: 'static + Send + Sync {
    fn component_id() -> ComponentId;
}

pub trait System {
    fn run(&mut self, world: &mut World, delta: f32);
}

// Component Examples
#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
    pub armor: f32,
}

#[derive(Component)]
pub struct Aircraft {
    pub aircraft_type: AircraftType,
    pub level: u8,
    pub experience: u32,
}
```

### 2. Game Loop Architecture

```rust
pub struct GameLoop {
    last_frame_time: f64,
    accumulator: f64,
    fixed_timestep: f64,  // 1/60 for 60 FPS
    max_updates: u32,      // Prevent spiral of death
}

impl GameLoop {
    pub fn tick(&mut self, current_time: f64) -> LoopResult {
        let delta = current_time - self.last_frame_time;
        self.last_frame_time = current_time;
        
        self.accumulator += delta.min(0.25); // Cap at 250ms
        
        let mut updates = 0;
        while self.accumulator >= self.fixed_timestep && updates < self.max_updates {
            self.fixed_update(self.fixed_timestep);
            self.accumulator -= self.fixed_timestep;
            updates += 1;
        }
        
        let interpolation = self.accumulator / self.fixed_timestep;
        self.render(interpolation);
        
        LoopResult { updates, interpolation }
    }
    
    fn fixed_update(&mut self, dt: f64) {
        // Physics, collision detection, game logic
        self.physics_system.update(dt);
        self.collision_system.update(dt);
        self.gameplay_system.update(dt);
    }
    
    fn render(&mut self, interpolation: f64) {
        // Interpolated rendering
        self.renderer.render_frame(interpolation);
    }
}
```

### 3. Memory Management

```rust
// Object Pooling System
pub struct ObjectPool<T> {
    available: Vec<T>,
    in_use: Vec<T>,
    factory: Box<dyn Fn() -> T>,
    reset: Box<dyn Fn(&mut T)>,
    max_size: usize,
}

impl<T> ObjectPool<T> {
    pub fn acquire(&mut self) -> Option<T> {
        if let Some(obj) = self.available.pop() {
            Some(obj)
        } else if self.in_use.len() < self.max_size {
            Some((self.factory)())
        } else {
            None
        }
    }
    
    pub fn release(&mut self, mut obj: T) {
        (self.reset)(&mut obj);
        self.available.push(obj);
    }
}

// Usage for projectiles
let bullet_pool = ObjectPool::new(
    || Bullet::new(),
    |b| b.reset(),
    1000  // Max 1000 bullets
);
```

-----

## Rendering Pipeline

### 1. WebGL 2.0 Renderer

```rust
pub struct Renderer {
    context: WebGl2RenderingContext,
    shader_cache: HashMap<String, ShaderProgram>,
    texture_cache: HashMap<String, TextureHandle>,
    sprite_batcher: SpriteBatcher,
    particle_system: ParticleSystem,
    post_processing: PostProcessingPipeline,
}

impl Renderer {
    pub fn render_frame(&mut self, world: &World, interpolation: f32) {
        // Clear framebuffer
        self.context.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
        
        // Render layers in order
        self.render_background(world);
        self.render_terrain(world);
        self.render_entities(world, interpolation);
        self.render_effects(world);
        self.render_ui(world);
        
        // Post-processing
        self.apply_post_processing();
    }
    
    fn render_entities(&mut self, world: &World, interp: f32) {
        self.sprite_batcher.begin();
        
        for entity in world.query::<(&Position, &Sprite, Option<&Velocity>)>() {
            let pos = if let Some(vel) = entity.2 {
                // Interpolate position for smooth rendering
                Position {
                    x: entity.0.x + vel.dx * interp,
                    y: entity.0.y + vel.dy * interp,
                }
            } else {
                *entity.0
            };
            
            self.sprite_batcher.draw(
                entity.1.texture,
                pos,
                entity.1.rotation,
                entity.1.scale,
                entity.1.color,
            );
        }
        
        self.sprite_batcher.flush();
    }
}
```

### 2. Sprite Batching System

```rust
pub struct SpriteBatcher {
    vertices: Vec<f32>,
    indices: Vec<u16>,
    current_texture: Option<TextureHandle>,
    draw_calls: u32,
    max_sprites_per_batch: usize,
}

impl SpriteBatcher {
    pub fn draw(&mut self, texture: TextureHandle, transform: Transform, color: Color) {
        if self.current_texture != Some(texture) {
            self.flush();
            self.current_texture = Some(texture);
        }
        
        if self.vertices.len() >= self.max_sprites_per_batch * 4 * 8 {
            self.flush();
        }
        
        // Add sprite vertices (position, uv, color)
        let base_index = (self.vertices.len() / 8) as u16;
        
        // Top-left, top-right, bottom-right, bottom-left
        self.add_vertex(transform.tl(), UV::TOP_LEFT, color);
        self.add_vertex(transform.tr(), UV::TOP_RIGHT, color);
        self.add_vertex(transform.br(), UV::BOTTOM_RIGHT, color);
        self.add_vertex(transform.bl(), UV::BOTTOM_LEFT, color);
        
        // Two triangles: 0-1-2, 0-2-3
        self.indices.extend_from_slice(&[
            base_index, base_index + 1, base_index + 2,
            base_index, base_index + 2, base_index + 3,
        ]);
    }
    
    pub fn flush(&mut self) {
        if self.vertices.is_empty() {
            return;
        }
        
        // Upload to GPU and draw
        self.upload_buffers();
        self.draw_elements();
        
        // Reset for next batch
        self.vertices.clear();
        self.indices.clear();
        self.draw_calls += 1;
    }
}
```

### 3. Particle System

```rust
pub struct ParticleSystem {
    emitters: Vec<ParticleEmitter>,
    particles: Vec<Particle>,
    particle_pool: ObjectPool<Particle>,
    vertex_buffer: WebGlBuffer,
}

pub struct ParticleEmitter {
    position: Vec2,
    emission_rate: f32,
    particle_lifetime: Range<f32>,
    initial_velocity: Range<Vec2>,
    acceleration: Vec2,
    size_curve: AnimationCurve,
    color_gradient: Gradient,
    texture: TextureHandle,
}

impl ParticleSystem {
    pub fn update(&mut self, delta: f32) {
        // Update existing particles
        self.particles.retain_mut(|particle| {
            particle.lifetime -= delta;
            if particle.lifetime <= 0.0 {
                self.particle_pool.release(*particle);
                false
            } else {
                particle.position += particle.velocity * delta;
                particle.velocity += particle.acceleration * delta;
                particle.size = particle.size_curve.evaluate(particle.lifetime);
                particle.color = particle.color_gradient.evaluate(particle.lifetime);
                true
            }
        });
        
        // Emit new particles
        for emitter in &mut self.emitters {
            let particles_to_emit = (emitter.emission_rate * delta) as u32;
            for _ in 0..particles_to_emit {
                if let Some(mut particle) = self.particle_pool.acquire() {
                    particle.reset_from_emitter(emitter);
                    self.particles.push(particle);
                }
            }
        }
    }
}
```

-----

## Game Systems Implementation

### 1. Weapon System

```rust
pub struct WeaponSystem {
    weapons: HashMap<WeaponId, WeaponDefinition>,
    upgrades: HashMap<WeaponId, Vec<WeaponUpgrade>>,
}

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

pub enum SpreadPattern {
    Single,
    Twin { spacing: f32 },
    Spread { count: u32, angle: f32 },
    Circle { count: u32 },
    Custom(Box<dyn Fn(f32) -> Vec<Vec2>>),
}

impl WeaponSystem {
    pub fn fire(&mut self, weapon_id: WeaponId, origin: Vec2, direction: Vec2) -> Vec<Projectile> {
        let weapon = &self.weapons[&weapon_id];
        let pattern = self.calculate_spread(&weapon.spread_pattern, direction);
        
        pattern.into_iter().map(|dir| {
            Projectile {
                position: origin,
                velocity: dir * weapon.projectile_speed,
                damage: weapon.base_damage,
                projectile_type: weapon.projectile_type.clone(),
                owner: ProjectileOwner::Player,
                lifetime: 5.0,
            }
        }).collect()
    }
}
```

### 2. Collision Detection

```rust
pub struct CollisionSystem {
    spatial_grid: SpatialHashGrid,
    collision_pairs: Vec<(Entity, Entity)>,
}

pub struct SpatialHashGrid {
    cell_size: f32,
    cells: HashMap<(i32, i32), Vec<Entity>>,
}

impl SpatialHashGrid {
    pub fn insert(&mut self, entity: Entity, aabb: AABB) {
        let min_cell = self.world_to_cell(aabb.min);
        let max_cell = self.world_to_cell(aabb.max);
        
        for x in min_cell.0..=max_cell.0 {
            for y in min_cell.1..=max_cell.1 {
                self.cells.entry((x, y))
                    .or_insert_with(Vec::new)
                    .push(entity);
            }
        }
    }
    
    pub fn query(&self, aabb: AABB) -> HashSet<Entity> {
        let mut entities = HashSet::new();
        let min_cell = self.world_to_cell(aabb.min);
        let max_cell = self.world_to_cell(aabb.max);
        
        for x in min_cell.0..=max_cell.0 {
            for y in min_cell.1..=max_cell.1 {
                if let Some(cell_entities) = self.cells.get(&(x, y)) {
                    entities.extend(cell_entities);
                }
            }
        }
        
        entities
    }
    
    fn world_to_cell(&self, pos: Vec2) -> (i32, i32) {
        (
            (pos.x / self.cell_size).floor() as i32,
            (pos.y / self.cell_size).floor() as i32,
        )
    }
}

impl CollisionSystem {
    pub fn detect_collisions(&mut self, world: &World) {
        self.spatial_grid.clear();
        self.collision_pairs.clear();
        
        // Insert all collidable entities into spatial grid
        for (entity, position, collider) in world.query::<(&Entity, &Position, &Collider)>() {
            let aabb = collider.get_aabb(position);
            self.spatial_grid.insert(*entity, aabb);
        }
        
        // Broad phase: spatial hash grid
        // Narrow phase: precise collision detection
        for (entity, position, collider) in world.query::<(&Entity, &Position, &Collider)>() {
            let aabb = collider.get_aabb(position);
            let nearby = self.spatial_grid.query(aabb.expanded(1.0));
            
            for other in nearby {
                if other <= *entity { continue; } // Avoid duplicate checks
                
                if let Some((other_pos, other_collider)) = world.get::<(&Position, &Collider)>(other) {
                    if self.test_collision(position, collider, other_pos, other_collider) {
                        self.collision_pairs.push((*entity, other));
                    }
                }
            }
        }
    }
    
    fn test_collision(&self, pos1: &Position, col1: &Collider, 
                     pos2: &Position, col2: &Collider) -> bool {
        match (col1, col2) {
            (Collider::Circle(r1), Collider::Circle(r2)) => {
                let dist_sq = (pos1.x - pos2.x).powi(2) + (pos1.y - pos2.y).powi(2);
                dist_sq < (r1 + r2).powi(2)
            },
            (Collider::AABB(aabb1), Collider::AABB(aabb2)) => {
                let aabb1 = aabb1.translated(*pos1);
                let aabb2 = aabb2.translated(*pos2);
                aabb1.intersects(&aabb2)
            },
            _ => {
                // Circle-AABB collision
                self.test_circle_aabb_collision(pos1, col1, pos2, col2)
            }
        }
    }
}
```

### 3. AI System

```rust
pub struct AISystem {
    behavior_trees: HashMap<EnemyType, BehaviorTree>,
    pathfinding: PathfindingSystem,
}

pub enum AIBehavior {
    Sequence(Vec<AIBehavior>),
    Selector(Vec<AIBehavior>),
    Parallel(Vec<AIBehavior>),
    
    // Leaf nodes
    MoveToPlayer { speed: f32 },
    CircleStrafe { radius: f32, speed: f32 },
    FireAtPlayer { accuracy: f32 },
    Evade { duration: f32 },
    FormationFly { pattern: FormationPattern },
    KamikazeDive,
}

impl AISystem {
    pub fn update(&mut self, world: &mut World, delta: f32) {
        let player_pos = world.get_player_position();
        
        for (entity, ai_component, position) in world.query::<(&Entity, &mut AI, &Position)>() {
            let behavior = &self.behavior_trees[&ai_component.enemy_type];
            let context = AIContext {
                entity: *entity,
                position: *position,
                player_position: player_pos,
                world,
                delta,
            };
            
            behavior.execute(context);
        }
    }
}

// Enemy wave patterns
pub struct WavePattern {
    pub formation: Formation,
    pub entry_path: Path,
    pub behavior: AIBehavior,
    pub spawn_timing: Vec<f32>,
}

pub enum Formation {
    V { spacing: f32 },
    Line { spacing: f32, angle: f32 },
    Circle { radius: f32 },
    Diamond,
    Custom(Vec<Vec2>),
}
```

-----

## Procedural Generation

### 1. Level Generation Pipeline

```rust
pub struct ProceduralGenerator {
    rng: XorShiftRng,
    wave_templates: Vec<WaveTemplate>,
    terrain_generator: TerrainGenerator,
    difficulty_curve: DifficultyManager,
}

impl ProceduralGenerator {
    pub fn generate_zone(&mut self, zone_type: ZoneType, difficulty: f32) -> Zone {
        let mut zone = Zone::new(zone_type);
        
        // Generate terrain features
        let terrain = self.terrain_generator.generate(&zone_type, &mut self.rng);
        zone.set_terrain(terrain);
        
        // Generate enemy waves
        let wave_count = self.calculate_wave_count(difficulty);
        for i in 0..wave_count {
            let wave_difficulty = difficulty * (1.0 + i as f32 * 0.1);
            let wave = self.generate_wave(zone_type, wave_difficulty);
            zone.add_wave(wave);
        }
        
        // Add environmental hazards
        let hazards = self.generate_hazards(zone_type, difficulty);
        zone.set_hazards(hazards);
        
        // Place power-ups and upgrades
        let upgrades = self.generate_upgrades(difficulty);
        zone.set_upgrades(upgrades);
        
        zone
    }
    
    fn generate_wave(&mut self, zone_type: ZoneType, difficulty: f32) -> Wave {
        // Select appropriate templates based on zone and difficulty
        let valid_templates: Vec<_> = self.wave_templates.iter()
            .filter(|t| t.min_difficulty <= difficulty && t.zone_types.contains(&zone_type))
            .collect();
        
        let template = self.rng.choose(&valid_templates).unwrap();
        let mut wave = template.instantiate();
        
        // Apply difficulty modifiers
        wave.enemy_count = (wave.enemy_count as f32 * (1.0 + difficulty * 0.2)) as u32;
        wave.enemy_health_multiplier = 1.0 + difficulty * 0.15;
        wave.enemy_speed_multiplier = 1.0 + difficulty * 0.1;
        
        // Add variety
        if self.rng.gen_bool(0.3) {
            wave.add_elite_variant();
        }
        
        wave
    }
}
```

### 2. Upgrade System

```rust
pub struct UpgradeSystem {
    upgrade_pool: Vec<Upgrade>,
    synergy_map: HashMap<(UpgradeId, UpgradeId), SynergyBonus>,
    player_build: PlayerBuild,
}

pub struct Upgrade {
    pub id: UpgradeId,
    pub name: String,
    pub description: String,
    pub rarity: Rarity,
    pub category: UpgradeCategory,
    pub effects: Vec<Effect>,
    pub prerequisites: Vec<UpgradeId>,
}

pub enum Effect {
    StatModifier { stat: Stat, modifier: Modifier },
    AddWeapon { weapon: WeaponId },
    UnlockAbility { ability: AbilityId },
    PassiveEffect { effect: PassiveEffectType },
}

impl UpgradeSystem {
    pub fn generate_upgrade_choices(&mut self, count: u32, zone: u32) -> Vec<Upgrade> {
        let mut weights = self.calculate_upgrade_weights(zone);
        let mut choices = Vec::new();
        
        for _ in 0..count {
            let upgrade = self.weighted_random_selection(&weights);
            choices.push(upgrade.clone());
            
            // Remove selected upgrade from pool for this selection
            weights.retain(|u, _| u.id != upgrade.id);
            
            // Adjust weights based on synergies
            self.adjust_weights_for_synergies(&mut weights, &upgrade);
        }
        
        choices
    }
    
    fn calculate_upgrade_weights(&self, zone: u32) -> Vec<(Upgrade, f32)> {
        self.upgrade_pool.iter().map(|upgrade| {
            let mut weight = match upgrade.rarity {
                Rarity::Common => 100.0,
                Rarity::Rare => 25.0,
                Rarity::Epic => 5.0,
                Rarity::Legendary => 1.0,
            };
            
            // Increase weight for upgrades that synergize with current build
            for owned_upgrade in &self.player_build.upgrades {
                if let Some(synergy) = self.synergy_map.get(&(owned_upgrade.id, upgrade.id)) {
                    weight *= synergy.weight_multiplier;
                }
            }
            
            // Zone-based availability
            if zone < upgrade.min_zone {
                weight = 0.0;
            }
            
            (upgrade.clone(), weight)
        }).filter(|(_, w)| *w > 0.0).collect()
    }
}
```

-----

## Data Structures

### 1. Core Game State

```rust
#[derive(Serialize, Deserialize)]
pub struct GameState {
    // Current run state
    pub current_run: Option<RunState>,
    
    // Meta progression
    pub meta_progression: MetaProgression,
    
    // Settings
    pub settings: GameSettings,
    
    // Statistics
    pub statistics: GameStatistics,
}

#[derive(Serialize, Deserialize)]
pub struct RunState {
    pub seed: u64,
    pub aircraft: AircraftType,
    pub zone: u32,
    pub score: u64,
    pub time_elapsed: f32,
    pub upgrades: Vec<UpgradeId>,
    pub resources: Resources,
    pub current_health: i32,
    pub max_health: i32,
}

#[derive(Serialize, Deserialize)]
pub struct MetaProgression {
    pub squadron_xp: u32,
    pub squadron_level: u32,
    pub unlocked_aircraft: HashSet<AircraftType>,
    pub unlocked_upgrades: HashSet<UpgradeId>,
    pub permanent_upgrades: Vec<PermanentUpgrade>,
    pub mastery_levels: HashMap<AircraftType, MasteryLevel>,
}

#[derive(Serialize, Deserialize)]
pub struct Resources {
    pub primary_ammo: Option<u32>,
    pub special_ammo: u32,
    pub bombs: u32,
    pub boost_fuel: f32,
}
```

### 2. Asset Management

```rust
pub struct AssetManager {
    textures: HashMap<String, TextureAsset>,
    sounds: HashMap<String, AudioAsset>,
    sprite_sheets: HashMap<String, SpriteSheet>,
    loading_queue: VecDeque<AssetRequest>,
}

pub struct TextureAsset {
    pub handle: WebGlTexture,
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
    pub last_used: Instant,
}

pub struct SpriteSheet {
    pub texture: TextureAsset,
    pub sprites: HashMap<String, SpriteDefinition>,
}

pub struct SpriteDefinition {
    pub uv_min: Vec2,
    pub uv_max: Vec2,
    pub pivot: Vec2,
    pub border: Option<Border>,
}

impl AssetManager {
    pub async fn load_progressive(&mut self) {
        // Load critical assets first
        self.load_priority_assets().await;
        
        // Load remaining assets in background
        while let Some(request) = self.loading_queue.pop_front() {
            match request {
                AssetRequest::Texture(path) => {
                    let texture = self.load_texture(&path).await;
                    self.textures.insert(path, texture);
                },
                AssetRequest::Audio(path) => {
                    let audio = self.load_audio(&path).await;
                    self.sounds.insert(path, audio);
                },
                AssetRequest::SpriteSheet(path) => {
                    let sheet = self.load_sprite_sheet(&path).await;
                    self.sprite_sheets.insert(path, sheet);
                },
            }
        }
    }
}
```

-----

## Performance Specifications

### 1. Performance Targets

```yaml
Frame Rate:
  Desktop:
    Minimum: 60 FPS @ 1920x1080
    Target: 144 FPS @ 1920x1080
    Maximum Entities: 500 simultaneous
    
  Mobile:
    Minimum: 30 FPS @ 1280x720
    Target: 60 FPS @ 1280x720
    Maximum Entities: 200 simultaneous

Memory Usage:
  Initial Load: < 100MB
  Runtime Peak: < 500MB
  Texture Memory: < 256MB
  Audio Memory: < 64MB

Load Times:
  Initial Load: < 3 seconds
  Zone Transition: < 500ms
  Asset Streaming: Progressive

Battery Life (Mobile):
  Target: > 2 hours continuous play
  Optimization: Dynamic quality adjustment
```

### 2. Optimization Strategies

```rust
// Culling System
pub struct CullingSystem {
    view_bounds: AABB,
    frustum: Frustum,
}

impl CullingSystem {
    pub fn cull_entities(&self, entities: &[Entity]) -> Vec<Entity> {
        entities.iter()
            .filter(|e| self.is_visible(e))
            .cloned()
            .collect()
    }
    
    fn is_visible(&self, entity: &Entity) -> bool {
        // Early out for always-visible entities
        if entity.has_component::<AlwaysVisible>() {
            return true;
        }
        
        // Frustum culling for 3D objects
        if let Some(bounds) = entity.get_component::<Bounds>() {
            return self.frustum.intersects(bounds);
        }
        
        // Simple bounds check for 2D sprites
        if let Some(position) = entity.get_component::<Position>() {
            return self.view_bounds.contains(position.as_vec2());
        }
        
        false
    }
}

// LOD System
pub struct LODSystem {
    camera_position: Vec2,
    lod_distances: [f32; 4],
}

impl LODSystem {
    pub fn get_lod_level(&self, entity_position: Vec2) -> u8 {
        let distance = (entity_position - self.camera_position).length();
        
        for (i, &threshold) in self.lod_distances.iter().enumerate() {
            if distance < threshold {
                return i as u8;
            }
        }
        
        // Beyond all LOD levels, don't render
        return 255;
    }
}
```

### 3. Profiling and Metrics

```rust
pub struct PerformanceMonitor {
    frame_times: RingBuffer<f32>,
    update_times: RingBuffer<f32>,
    render_times: RingBuffer<f32>,
    memory_usage: MemoryMetrics,
    draw_calls: u32,
    triangles_drawn: u32,
}

impl PerformanceMonitor {
    pub fn begin_frame(&mut self) {
        self.frame_start = performance::now();
        self.draw_calls = 0;
        self.triangles_drawn = 0;
    }
    
    pub fn end_frame(&mut self) {
        let frame_time = performance::now() - self.frame_start;
        self.frame_times.push(frame_time);
        
        // Detect performance issues
        if frame_time > 16.67 {  // Below 60 FPS
            self.log_performance_warning(frame_time);
        }
        
        // Auto-adjust quality settings
        if self.get_average_fps() < 55.0 {
            self.suggest_quality_reduction();
        }
    }
    
    pub fn get_metrics(&self) -> PerformanceMetrics {
        PerformanceMetrics {
            fps: self.get_average_fps(),
            frame_time_ms: self.get_average_frame_time(),
            memory_used_mb: self.memory_usage.used_mb(),
            draw_calls: self.draw_calls,
            triangles: self.triangles_drawn,
        }
    }
}
```

-----

## Build & Deployment

### 1. Build Pipeline

```bash
#!/bin/bash
# build.sh - Production build script

# Build WASM module
echo "Building WASM module..."
wasm-pack build --target web --release \
  --no-typescript \
  --out-dir ./pkg \
  --out-name aces_high

# Optimize WASM binary
echo "Optimizing WASM..."
wasm-opt -Os -o ./pkg/aces_high_opt.wasm ./pkg/aces_high_bg.wasm

# Build web assets
echo "Building web assets..."
npm run build

# Compress assets
echo "Compressing assets..."
find ./dist -type f \( -name "*.js" -o -name "*.wasm" -o -name "*.json" \) \
  -exec gzip -9 -k {} \;

# Generate manifest
echo "Generating manifest..."
node ./scripts/generate-manifest.js > ./dist/manifest.json

echo "Build complete!"
```

### 2. Webpack Configuration

```javascript
// webpack.config.js
const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const WorkboxPlugin = require('workbox-webpack-plugin');
const CompressionPlugin = require('compression-webpack-plugin');

module.exports = (env, argv) => {
  const isProduction = argv.mode === 'production';
  
  return {
    entry: './src/index.js',
    output: {
      path: path.resolve(__dirname, 'dist'),
      filename: isProduction ? '[name].[contenthash].js' : '[name].js',
      clean: true,
    },
    module: {
      rules: [
        {
          test: /\.wasm$/,
          type: 'webassembly/async',
        },
        {
          test: /\.(png|jpg|gif|ogg|mp3)$/,
          type: 'asset/resource',
        },
      ],
    },
    plugins: [
      new HtmlWebpackPlugin({
        template: './src/index.html',
        minify: isProduction,
      }),
      new WasmPackPlugin({
        crateDirectory: path.resolve(__dirname, '.'),
        outDir: path.resolve(__dirname, 'pkg'),
      }),
      isProduction && new CompressionPlugin({
        test: /\.(js|wasm|json)$/,
        algorithm: 'gzip',
      }),
      isProduction && new WorkboxPlugin.GenerateSW({
        clientsClaim: true,
        skipWaiting: true,
        runtimeCaching: [{
          urlPattern: /\.(?:png|jpg|jpeg|svg|gif|ogg|mp3)$/,
          handler: 'CacheFirst',
          options: {
            cacheName: 'assets',
            expiration: {
              maxEntries: 100,
              maxAgeSeconds: 30 * 24 * 60 * 60, // 30 Days
            },
          },
        }],
      }),
    ].filter(Boolean),
    experiments: {
      asyncWebAssembly: true,
    },
    optimization: {
      splitChunks: {
        chunks: 'all',
        cacheGroups: {
          vendor: {
            test: /[\\/]node_modules[\\/]/,
            name: 'vendors',
            priority: -10,
          },
        },
      },
    },
    devServer: {
      static: './dist',
      hot: true,
      headers: {
        'Cross-Origin-Embedder-Policy': 'require-corp',
        'Cross-Origin-Opener-Policy': 'same-origin',
      },
    },
  };
};
```

### 3. CI/CD Pipeline

```yaml
# .github/workflows/deploy.yml
name: Build and Deploy

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  build:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        target: wasm32-unknown-unknown
        
    - name: Install wasm-pack
      run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
      
    - name: Install Node.js
      uses: actions/setup-node@v3
      with:
        node-version: '18'
        
    - name: Install dependencies
      run: npm ci
      
    - name: Run tests
      run: |
        cargo test
        wasm-pack test --headless --chrome
        
    - name: Build production
      run: npm run build:prod
      
    - name: Deploy to CDN
      if: github.ref == 'refs/heads/main'
      run: |
        aws s3 sync dist/ s3://${{ secrets.S3_BUCKET }} --delete
        aws cloudfront create-invalidation \
          --distribution-id ${{ secrets.CLOUDFRONT_ID }} \
          --paths "/*"
```

-----

## Testing Strategy

### 1. Unit Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn test_collision_detection() {
        let mut collision_system = CollisionSystem::new();
        
        let entity1 = Entity::new(1);
        let pos1 = Position { x: 0.0, y: 0.0 };
        let collider1 = Collider::Circle(10.0);
        
        let entity2 = Entity::new(2);
        let pos2 = Position { x: 15.0, y: 0.0 };
        let collider2 = Collider::Circle(10.0);
        
        assert!(collision_system.test_collision(&pos1, &collider1, &pos2, &collider2));
    }
    
    #[wasm_bindgen_test]
    fn test_weapon_upgrade() {
        let mut weapon_system = WeaponSystem::new();
        let base_weapon = weapon_system.get_weapon(WeaponId::MachineGun);
        
        weapon_system.apply_upgrade(WeaponId::MachineGun, UpgradeId::RapidFire);
        let upgraded = weapon_system.get_weapon(WeaponId::MachineGun);
        
        assert!(upgraded.fire_rate > base_weapon.fire_rate);
        assert!(upgraded.damage < base_weapon.damage);
    }
}
```

### 2. Integration Testing

```javascript
// tests/integration.test.js
const puppeteer = require('puppeteer');

describe('Game Integration Tests', () => {
  let browser;
  let page;
  
  beforeAll(async () => {
    browser = await puppeteer.launch();
    page = await browser.newPage();
    await page.goto('http://localhost:8080');
  });
  
  afterAll(async () => {
    await browser.close();
  });
  
  test('Game loads within 3 seconds', async () => {
    const startTime = Date.now();
    await page.waitForSelector('.game-canvas', { timeout: 3000 });
    const loadTime = Date.now() - startTime;
    expect(loadTime).toBeLessThan(3000);
  });
  
  test('Maintains 60 FPS during gameplay', async () => {
    const fps = await page.evaluate(() => {
      return new Promise(resolve => {
        let frames = 0;
        const startTime = performance.now();
        
        function countFrame() {
          frames++;
          if (performance.now() - startTime < 1000) {
            requestAnimationFrame(countFrame);
          } else {
            resolve(frames);
          }
        }
        
        requestAnimationFrame(countFrame);
      });
    });
    
    expect(fps).toBeGreaterThan(58);
  });
});
```

### 3. Performance Testing

```rust
pub struct PerformanceTest {
    scenarios: Vec<TestScenario>,
    results: Vec<TestResult>,
}

impl PerformanceTest {
    pub fn run_stress_test(&mut self) {
        // Test maximum entity count
        self.test_entity_limit();
        
        // Test particle system limits
        self.test_particle_limits();
        
        // Test collision detection performance
        self.test_collision_performance();
        
        // Test memory usage
        self.test_memory_usage();
    }
    
    fn test_entity_limit(&mut self) {
        let mut world = World::new();
        let mut fps_samples = Vec::new();
        
        for count in (100..=1000).step_by(50) {
            // Spawn entities
            for _ in 0..50 {
                world.spawn_enemy(random_position(), EnemyType::Fighter);
            }
            
            // Measure frame time
            let frame_time = self.measure_frame_time(&mut world);
            fps_samples.push((count, 1000.0 / frame_time));
            
            // Find breaking point
            if frame_time > 16.67 {  // Below 60 FPS
                println!("Performance degrades at {} entities", count);
                break;
            }
        }
        
        self.results.push(TestResult {
            name: "Entity Limit Test".to_string(),
            data: fps_samples,
            passed: true,
        });
    }
}
```

-----

## Development Timeline

### Phase 1: Foundation (Weeks 1-4)

- [x] Setup Rust/WASM build pipeline
- [ ] Implement core ECS architecture
- [ ] Basic rendering pipeline
- [ ] Input handling system
- [ ] Basic physics and collision
- [ ] Single aircraft prototype

### Phase 2: Core Gameplay (Weeks 5-8)

- [ ] Weapon system implementation
- [ ] Enemy AI basics
- [ ] Procedural wave generation
- [ ] Power-up system
- [ ] Score and combo system
- [ ] First 3 zones

### Phase 3: Roguelike Systems (Weeks 9-12)

- [ ] Meta-progression system
- [ ] Upgrade tree implementation
- [ ] Run persistence
- [ ] Aircraft unlocking
- [ ] Difficulty scaling
- [ ] All 5 zones complete

### Phase 4: Polish & Content (Weeks 13-16)

- [ ] All 5 aircraft implemented
- [ ] Boss battles
- [ ] Particle effects
- [ ] Sound system
- [ ] Music integration
- [ ] UI/UX polish

### Phase 5: Optimization (Weeks 17-18)

- [ ] Performance profiling
- [ ] Mobile optimization
- [ ] Asset optimization
- [ ] Loading optimization
- [ ] Battery usage optimization

### Phase 6: Launch Prep (Weeks 19-20)

- [ ] Bug fixes
- [ ] Balancing
- [ ] Achievements system
- [ ] Leaderboards
- [ ] Marketing materials
- [ ] Launch!

-----

## Appendices

### A. Asset Specifications

```yaml
Sprites:
  Aircraft:
    Resolution: 256x256
    Format: PNG with alpha
    Angles: 16 rotations
    States: [normal, damaged, destroyed]
    
  Projectiles:
    Resolution: 32x32
    Format: PNG with alpha
    
  Effects:
    Resolution: 128x128
    Format: PNG sprite sheets
    Frames: 8-16 per effect
    
  Terrain:
    Resolution: 512x512 tiles
    Format: PNG
    
Audio:
  Music:
    Format: OGG Vorbis
    Quality: 128kbps
    Loops: Seamless
    
  SFX:
    Format: OGG Vorbis
    Quality: 96kbps
    Length: < 2 seconds typically
```

### B. Browser Compatibility

```javascript
// Compatibility checks
const checkCompatibility = () => {
  const required = {
    webgl2: !!document.createElement('canvas').getContext('webgl2'),
    wasm: typeof WebAssembly !== 'undefined',
    audioContext: 'AudioContext' in window || 'webkitAudioContext' in window,
    indexedDB: 'indexedDB' in window,
    gamepad: 'getGamepads' in navigator,
  };
  
  const optional = {
    webgpu: 'gpu' in navigator,
    sharedArrayBuffer: 'SharedArrayBuffer' in window,
    offscreenCanvas: 'OffscreenCanvas' in window,
  };
  
  return { required, optional };
};
```

### C. Security Considerations

```rust
// Input sanitization
pub fn sanitize_player_name(input: &str) -> String {
    input.chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .take(20)
        .collect()
}

// Save data integrity
pub fn verify_save_data(data: &[u8]) -> Result<SaveData, Error> {
    let hash = calculate_hash(data);
    let stored_hash = get_stored_hash()?;
    
    if hash != stored_hash {
        return Err(Error::CorruptedSaveData);
    }
    
    Ok(deserialize_save_data(data)?)
}
```

### D. Accessibility Features

```yaml
Visual:
  - Colorblind modes (Protanopia, Deuteranopia, Tritanopia)
  - Contrast adjustment
  - UI scaling (75% - 150%)
  - Reduced motion option
  - Screen reader support for menus

Audio:
  - Subtitle support for audio cues
  - Visual indicators for off-screen threats
  - Adjustable audio mix
  - Mono audio option

Controls:
  - Remappable controls
  - One-handed mode
  - Hold-to-fire toggle
  - Difficulty accessibility options
```

-----

## Version History

|Version|Date      |Changes                        |
|-------|----------|-------------------------------|
|1.0    |2024-01-20|Initial technical specification|

-----

## Contributors

- Technical Lead: [Your Name]
- Game Design: [Designer Name]
- Art Direction: [Artist Name]

-----

*This document is a living specification and will be updated throughout development.*

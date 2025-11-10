use crate::game::components::Position;
use crate::game::entities::{Entity, EnemyType};
use crate::utils::Vec2;
use cgmath::InnerSpace;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct AISystem {
    behavior_trees: HashMap<EnemyType, BehaviorTree>,
    enemy_states: HashMap<Entity, AIState>,
}

impl AISystem {
    pub fn new() -> Self {
        let mut system = Self {
            behavior_trees: HashMap::new(),
            enemy_states: HashMap::new(),
        };

        // Initialize default behavior trees for each enemy type
        system.init_default_behaviors();
        system
    }

    fn init_default_behaviors(&mut self) {
        // Fighter: aggressive pursuit and fire
        self.behavior_trees.insert(
            EnemyType::Fighter,
            BehaviorTree {
                root: AIBehavior::Sequence(vec![
                    AIBehavior::MoveToPlayer { speed: 150.0 },
                    AIBehavior::FireAtPlayer { accuracy: 0.8 },
                ]),
            },
        );

        // Bomber: maintain distance and fire
        self.behavior_trees.insert(
            EnemyType::Bomber,
            BehaviorTree {
                root: AIBehavior::Selector(vec![
                    AIBehavior::Sequence(vec![
                        AIBehavior::CircleStrafe {
                            radius: 200.0,
                            speed: 100.0,
                        },
                        AIBehavior::FireAtPlayer { accuracy: 0.6 },
                    ]),
                    AIBehavior::MoveToPlayer { speed: 80.0 },
                ]),
            },
        );

        // Ace: advanced tactics
        self.behavior_trees.insert(
            EnemyType::Ace,
            BehaviorTree {
                root: AIBehavior::Parallel(vec![
                    AIBehavior::Selector(vec![
                        AIBehavior::Evade { duration: 2.0 },
                        AIBehavior::CircleStrafe {
                            radius: 150.0,
                            speed: 200.0,
                        },
                    ]),
                    AIBehavior::FireAtPlayer { accuracy: 0.95 },
                ]),
            },
        );

        // Kamikaze: dive straight at player
        self.behavior_trees.insert(
            EnemyType::Kamikaze,
            BehaviorTree {
                root: AIBehavior::KamikazeDive,
            },
        );

        // Heavy Bomber: formation flying
        self.behavior_trees.insert(
            EnemyType::HeavyBomber,
            BehaviorTree {
                root: AIBehavior::Sequence(vec![
                    AIBehavior::FormationFly {
                        pattern: FormationPattern::VFormation,
                    },
                    AIBehavior::FireAtPlayer { accuracy: 0.5 },
                ]),
            },
        );
    }

    pub fn register_enemy(&mut self, entity: Entity, enemy_type: EnemyType) {
        self.enemy_states.insert(
            entity,
            AIState {
                enemy_type,
                state_timer: 0.0,
                target_position: None,
                formation_offset: Vec2::new(0.0, 0.0),
            },
        );
    }

    pub fn unregister_enemy(&mut self, entity: Entity) {
        self.enemy_states.remove(&entity);
    }

    pub fn update(
        &mut self,
        entity: Entity,
        position: &Position,
        player_position: &Position,
        delta: f32,
    ) -> AICommand {
        if let Some(state) = self.enemy_states.get_mut(&entity) {
            state.state_timer += delta;

            if let Some(behavior_tree) = self.behavior_trees.get(&state.enemy_type) {
                let context = AIContext {
                    entity,
                    position: *position,
                    player_position: *player_position,
                    state,
                    delta,
                };

                return self.execute_behavior(&behavior_tree.root, context);
            }
        }

        AICommand::None
    }

    fn execute_behavior(&self, behavior: &AIBehavior, context: AIContext) -> AICommand {
        match behavior {
            AIBehavior::Sequence(behaviors) => {
                // Execute all behaviors in sequence, return first non-None
                for behavior in behaviors {
                    let result = self.execute_behavior(behavior, context);
                    if !matches!(result, AICommand::None) {
                        return result;
                    }
                }
                AICommand::None
            }

            AIBehavior::Selector(behaviors) => {
                // Execute first successful behavior
                for behavior in behaviors {
                    let result = self.execute_behavior(behavior, context);
                    if !matches!(result, AICommand::None) {
                        return result;
                    }
                }
                AICommand::None
            }

            AIBehavior::Parallel(behaviors) => {
                // Execute all behaviors and combine results
                let mut commands = Vec::new();
                for behavior in behaviors {
                    let result = self.execute_behavior(behavior, context);
                    if !matches!(result, AICommand::None) {
                        commands.push(result);
                    }
                }

                if commands.is_empty() {
                    AICommand::None
                } else {
                    AICommand::Multiple(commands)
                }
            }

            AIBehavior::MoveToPlayer { speed } => {
                let direction = (context.player_position.as_vec2() - context.position.as_vec2())
                    .normalize();
                AICommand::Move {
                    direction,
                    speed: *speed,
                }
            }

            AIBehavior::CircleStrafe { radius, speed } => {
                let to_player = context.player_position.as_vec2() - context.position.as_vec2();
                let distance = to_player.magnitude();

                if distance < *radius * 0.8 {
                    // Too close, move away
                    AICommand::Move {
                        direction: -to_player.normalize(),
                        speed: *speed,
                    }
                } else if distance > *radius * 1.2 {
                    // Too far, move closer
                    AICommand::Move {
                        direction: to_player.normalize(),
                        speed: *speed,
                    }
                } else {
                    // Maintain distance and strafe
                    let perpendicular = Vec2::new(-to_player.y, to_player.x).normalize();
                    AICommand::Move {
                        direction: perpendicular,
                        speed: *speed,
                    }
                }
            }

            AIBehavior::FireAtPlayer { accuracy } => {
                let direction = (context.player_position.as_vec2() - context.position.as_vec2())
                    .normalize();

                // Add inaccuracy
                let inaccuracy = (1.0 - accuracy) * 0.5;
                let random_offset = Vec2::new(
                    (rand::random::<f32>() - 0.5) * inaccuracy,
                    (rand::random::<f32>() - 0.5) * inaccuracy,
                );

                AICommand::Fire {
                    direction: (direction + random_offset).normalize(),
                }
            }

            AIBehavior::Evade { duration } => {
                if context.state.state_timer % (duration + 2.0) < *duration {
                    // Evade by moving perpendicular to player
                    let to_player =
                        context.player_position.as_vec2() - context.position.as_vec2();
                    let perpendicular = Vec2::new(-to_player.y, to_player.x).normalize();
                    let sign = if (context.state.state_timer as i32) % 2 == 0 {
                        1.0
                    } else {
                        -1.0
                    };

                    AICommand::Move {
                        direction: perpendicular * sign,
                        speed: 250.0,
                    }
                } else {
                    AICommand::None
                }
            }

            AIBehavior::FormationFly { pattern } => {
                let target = self.calculate_formation_position(
                    context.player_position.as_vec2(),
                    context.state.formation_offset,
                    pattern,
                );

                let direction = (target - context.position.as_vec2()).normalize();
                AICommand::Move {
                    direction,
                    speed: 120.0,
                }
            }

            AIBehavior::KamikazeDive => {
                let direction = (context.player_position.as_vec2() - context.position.as_vec2())
                    .normalize();
                AICommand::Move {
                    direction,
                    speed: 300.0,
                }
            }
        }
    }

    fn calculate_formation_position(
        &self,
        base: Vec2,
        offset: Vec2,
        pattern: &FormationPattern,
    ) -> Vec2 {
        match pattern {
            FormationPattern::VFormation => base + Vec2::new(offset.x * 50.0, offset.y * -30.0),
            FormationPattern::Line => base + Vec2::new(offset.x * 80.0, 0.0),
            FormationPattern::Circle => {
                let angle = offset.x * std::f32::consts::PI * 2.0;
                base + Vec2::new(angle.cos() * 100.0, angle.sin() * 100.0)
            }
            FormationPattern::Diamond => {
                base + Vec2::new(offset.x * 50.0, (offset.y - 0.5).abs() * 100.0)
            }
        }
    }
}

impl Default for AISystem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorTree {
    pub root: AIBehavior,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AIBehavior {
    // Composite nodes
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FormationPattern {
    VFormation,
    Line,
    Circle,
    Diamond,
}

#[derive(Debug, Clone)]
pub struct AIState {
    pub enemy_type: EnemyType,
    pub state_timer: f32,
    pub target_position: Option<Vec2>,
    pub formation_offset: Vec2,
}

#[derive(Debug, Clone, Copy)]
pub struct AIContext<'a> {
    pub entity: Entity,
    pub position: Position,
    pub player_position: Position,
    pub state: &'a AIState,
    pub delta: f32,
}

#[derive(Debug, Clone)]
pub enum AICommand {
    None,
    Move { direction: Vec2, speed: f32 },
    Fire { direction: Vec2 },
    Multiple(Vec<AICommand>),
}

// Wave patterns for enemy spawning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WavePattern {
    pub formation: Formation,
    pub entry_path: Path,
    pub behavior: AIBehavior,
    pub spawn_timing: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Formation {
    V { spacing: f32 },
    Line { spacing: f32, angle: f32 },
    Circle { radius: f32 },
    Diamond,
    Custom(Vec<Vec2>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Path {
    pub waypoints: Vec<Vec2>,
    pub loop_path: bool,
}

impl Path {
    pub fn new(waypoints: Vec<Vec2>) -> Self {
        Self {
            waypoints,
            loop_path: false,
        }
    }

    pub fn looping(waypoints: Vec<Vec2>) -> Self {
        Self {
            waypoints,
            loop_path: true,
        }
    }

    pub fn get_position_at(&self, t: f32) -> Option<Vec2> {
        if self.waypoints.is_empty() {
            return None;
        }

        if self.waypoints.len() == 1 {
            return Some(self.waypoints[0]);
        }

        let total_segments = (self.waypoints.len() - 1) as f32;
        let mut t = t.clamp(0.0, 1.0);

        if self.loop_path {
            t = t % 1.0;
        }

        let segment_float = t * total_segments;
        let segment_index = segment_float.floor() as usize;
        let local_t = segment_float - segment_index as f32;

        if segment_index >= self.waypoints.len() - 1 {
            return Some(*self.waypoints.last().unwrap());
        }

        let start = self.waypoints[segment_index];
        let end = self.waypoints[segment_index + 1];

        Some(start + (end - start) * local_t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_system_creation() {
        let ai_system = AISystem::new();
        assert_eq!(ai_system.behavior_trees.len(), 5); // 5 enemy types
    }

    #[test]
    fn test_enemy_registration() {
        let mut ai_system = AISystem::new();
        let entity = Entity::new(1);

        ai_system.register_enemy(entity, EnemyType::Fighter);
        assert!(ai_system.enemy_states.contains_key(&entity));

        ai_system.unregister_enemy(entity);
        assert!(!ai_system.enemy_states.contains_key(&entity));
    }

    #[test]
    fn test_path_linear() {
        let path = Path::new(vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(10.0, 0.0),
            Vec2::new(10.0, 10.0),
        ]);

        let pos = path.get_position_at(0.0).unwrap();
        assert!((pos.x - 0.0).abs() < 0.001);
        assert!((pos.y - 0.0).abs() < 0.001);

        let pos = path.get_position_at(0.5).unwrap();
        assert!((pos.x - 10.0).abs() < 0.001);
        assert!((pos.y - 0.0).abs() < 0.001);

        let pos = path.get_position_at(1.0).unwrap();
        assert!((pos.x - 10.0).abs() < 0.001);
        assert!((pos.y - 10.0).abs() < 0.001);
    }

    #[test]
    fn test_path_looping() {
        let path = Path::looping(vec![Vec2::new(0.0, 0.0), Vec2::new(10.0, 0.0)]);

        assert!(path.loop_path);

        let pos = path.get_position_at(0.0).unwrap();
        assert!((pos.x - 0.0).abs() < 0.001);
    }
}

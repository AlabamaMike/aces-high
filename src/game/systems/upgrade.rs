use crate::game::state::UpgradeId;
use crate::game::systems::weapon::WeaponId;
use crate::utils::WeightedRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct UpgradeSystem {
    upgrade_pool: Vec<Upgrade>,
    synergy_map: HashMap<(UpgradeId, UpgradeId), SynergyBonus>,
    player_build: PlayerBuild,
}

impl UpgradeSystem {
    pub fn new() -> Self {
        let mut system = Self {
            upgrade_pool: Vec::new(),
            synergy_map: HashMap::new(),
            player_build: PlayerBuild::new(),
        };

        system.init_upgrades();
        system.init_synergies();
        system
    }

    fn init_upgrades(&mut self) {
        // Weapon upgrades
        self.upgrade_pool.push(Upgrade {
            id: UpgradeId(1),
            name: "Rapid Fire".to_string(),
            description: "Increases fire rate by 30%".to_string(),
            rarity: Rarity::Common,
            category: UpgradeCategory::Weapon,
            effects: vec![Effect::StatModifier {
                stat: Stat::FireRate,
                modifier: Modifier::Multiply(1.3),
            }],
            prerequisites: Vec::new(),
            min_zone: 1,
        });

        self.upgrade_pool.push(Upgrade {
            id: UpgradeId(2),
            name: "Armor Piercing Rounds".to_string(),
            description: "Increases damage by 50%".to_string(),
            rarity: Rarity::Rare,
            category: UpgradeCategory::Weapon,
            effects: vec![Effect::StatModifier {
                stat: Stat::Damage,
                modifier: Modifier::Multiply(1.5),
            }],
            prerequisites: Vec::new(),
            min_zone: 2,
        });

        self.upgrade_pool.push(Upgrade {
            id: UpgradeId(3),
            name: "Twin Guns".to_string(),
            description: "Fire two bullets at once".to_string(),
            rarity: Rarity::Rare,
            category: UpgradeCategory::Weapon,
            effects: vec![Effect::AddWeapon {
                weapon: WeaponId(2),
            }],
            prerequisites: Vec::new(),
            min_zone: 2,
        });

        // Defense upgrades
        self.upgrade_pool.push(Upgrade {
            id: UpgradeId(4),
            name: "Reinforced Hull".to_string(),
            description: "Increases max health by 25%".to_string(),
            rarity: Rarity::Common,
            category: UpgradeCategory::Defense,
            effects: vec![Effect::StatModifier {
                stat: Stat::MaxHealth,
                modifier: Modifier::Multiply(1.25),
            }],
            prerequisites: Vec::new(),
            min_zone: 1,
        });

        self.upgrade_pool.push(Upgrade {
            id: UpgradeId(5),
            name: "Armor Plating".to_string(),
            description: "Reduces damage taken by 20%".to_string(),
            rarity: Rarity::Rare,
            category: UpgradeCategory::Defense,
            effects: vec![Effect::StatModifier {
                stat: Stat::Armor,
                modifier: Modifier::Add(0.2),
            }],
            prerequisites: Vec::new(),
            min_zone: 2,
        });

        self.upgrade_pool.push(Upgrade {
            id: UpgradeId(6),
            name: "Shield Generator".to_string(),
            description: "Grants a rechargeable shield".to_string(),
            rarity: Rarity::Epic,
            category: UpgradeCategory::Defense,
            effects: vec![Effect::UnlockAbility {
                ability: AbilityId(1),
            }],
            prerequisites: Vec::new(),
            min_zone: 3,
        });

        // Mobility upgrades
        self.upgrade_pool.push(Upgrade {
            id: UpgradeId(7),
            name: "Afterburner".to_string(),
            description: "Increases movement speed by 20%".to_string(),
            rarity: Rarity::Common,
            category: UpgradeCategory::Mobility,
            effects: vec![Effect::StatModifier {
                stat: Stat::MoveSpeed,
                modifier: Modifier::Multiply(1.2),
            }],
            prerequisites: Vec::new(),
            min_zone: 1,
        });

        self.upgrade_pool.push(Upgrade {
            id: UpgradeId(8),
            name: "Evasive Maneuvers".to_string(),
            description: "Grants a dash ability".to_string(),
            rarity: Rarity::Rare,
            category: UpgradeCategory::Mobility,
            effects: vec![Effect::UnlockAbility {
                ability: AbilityId(2),
            }],
            prerequisites: Vec::new(),
            min_zone: 2,
        });

        // Utility upgrades
        self.upgrade_pool.push(Upgrade {
            id: UpgradeId(9),
            name: "Auto-Repair".to_string(),
            description: "Slowly regenerate health over time".to_string(),
            rarity: Rarity::Epic,
            category: UpgradeCategory::Utility,
            effects: vec![Effect::PassiveEffect {
                effect: PassiveEffectType::HealthRegen(2.0),
            }],
            prerequisites: Vec::new(),
            min_zone: 3,
        });

        self.upgrade_pool.push(Upgrade {
            id: UpgradeId(10),
            name: "Treasure Hunter".to_string(),
            description: "Increases pickup range and value".to_string(),
            rarity: Rarity::Common,
            category: UpgradeCategory::Utility,
            effects: vec![
                Effect::StatModifier {
                    stat: Stat::PickupRadius,
                    modifier: Modifier::Multiply(1.5),
                },
                Effect::PassiveEffect {
                    effect: PassiveEffectType::PickupBonus(1.25),
                },
            ],
            prerequisites: Vec::new(),
            min_zone: 1,
        });

        // Legendary upgrades
        self.upgrade_pool.push(Upgrade {
            id: UpgradeId(11),
            name: "Ace Pilot".to_string(),
            description: "Significantly improves all stats".to_string(),
            rarity: Rarity::Legendary,
            category: UpgradeCategory::Special,
            effects: vec![
                Effect::StatModifier {
                    stat: Stat::Damage,
                    modifier: Modifier::Multiply(1.5),
                },
                Effect::StatModifier {
                    stat: Stat::FireRate,
                    modifier: Modifier::Multiply(1.3),
                },
                Effect::StatModifier {
                    stat: Stat::MoveSpeed,
                    modifier: Modifier::Multiply(1.25),
                },
            ],
            prerequisites: Vec::new(),
            min_zone: 5,
        });
    }

    fn init_synergies(&mut self) {
        // Rapid Fire + Armor Piercing = Devastating Assault
        self.synergy_map.insert(
            (UpgradeId(1), UpgradeId(2)),
            SynergyBonus {
                name: "Devastating Assault".to_string(),
                description: "Rapid fire + High damage = Extra critical chance".to_string(),
                weight_multiplier: 2.0,
                bonus_effects: vec![Effect::StatModifier {
                    stat: Stat::CritChance,
                    modifier: Modifier::Add(0.15),
                }],
            },
        );

        // Armor Plating + Reinforced Hull = Fortress
        self.synergy_map.insert(
            (UpgradeId(4), UpgradeId(5)),
            SynergyBonus {
                name: "Fortress".to_string(),
                description: "High health + High armor = Massive survivability".to_string(),
                weight_multiplier: 1.8,
                bonus_effects: vec![Effect::StatModifier {
                    stat: Stat::MaxHealth,
                    modifier: Modifier::Multiply(1.2),
                }],
            },
        );

        // Afterburner + Evasive Maneuvers = Speed Demon
        self.synergy_map.insert(
            (UpgradeId(7), UpgradeId(8)),
            SynergyBonus {
                name: "Speed Demon".to_string(),
                description: "Fast movement + Dash = Reduced dash cooldown".to_string(),
                weight_multiplier: 1.5,
                bonus_effects: vec![Effect::StatModifier {
                    stat: Stat::AbilityCooldown,
                    modifier: Modifier::Multiply(0.7),
                }],
            },
        );
    }

    pub fn generate_upgrade_choices(&mut self, count: u32, zone: u32) -> Vec<Upgrade> {
        let weights = self.calculate_upgrade_weights(zone);
        let mut weighted_random = WeightedRandom::new();

        for (upgrade, weight) in weights {
            weighted_random.add(upgrade, weight);
        }

        let mut choices = Vec::new();
        let mut selected_ids = Vec::new();

        for _ in 0..count {
            if let Some(upgrade) = weighted_random.select(&mut rand::thread_rng()) {
                if !selected_ids.contains(&upgrade.id) {
                    choices.push(upgrade.clone());
                    selected_ids.push(upgrade.id);
                }
            }
        }

        choices
    }

    fn calculate_upgrade_weights(&self, zone: u32) -> Vec<(Upgrade, f32)> {
        self.upgrade_pool
            .iter()
            .filter_map(|upgrade| {
                if zone < upgrade.min_zone {
                    return None;
                }

                // Check prerequisites
                if !upgrade
                    .prerequisites
                    .iter()
                    .all(|prereq| self.player_build.has_upgrade(*prereq))
                {
                    return None;
                }

                let mut weight = match upgrade.rarity {
                    Rarity::Common => 100.0,
                    Rarity::Rare => 25.0,
                    Rarity::Epic => 5.0,
                    Rarity::Legendary => 1.0,
                };

                // Increase weight for upgrades with synergies
                for owned_upgrade in &self.player_build.upgrades {
                    if let Some(synergy) = self
                        .synergy_map
                        .get(&(*owned_upgrade, upgrade.id))
                        .or_else(|| self.synergy_map.get(&(upgrade.id, *owned_upgrade)))
                    {
                        weight *= synergy.weight_multiplier;
                    }
                }

                // Reduce weight for duplicate category upgrades
                let category_count = self
                    .player_build
                    .upgrades
                    .iter()
                    .filter(|id| {
                        self.upgrade_pool
                            .iter()
                            .find(|u| &u.id == *id)
                            .map_or(false, |u| u.category == upgrade.category)
                    })
                    .count();

                if category_count > 2 {
                    weight *= 0.5;
                }

                Some((upgrade.clone(), weight))
            })
            .collect()
    }

    pub fn apply_upgrade(&mut self, upgrade_id: UpgradeId) {
        if let Some(upgrade) = self.upgrade_pool.iter().find(|u| u.id == upgrade_id) {
            self.player_build.add_upgrade(upgrade_id);

            // Check for synergies
            for owned_upgrade in &self.player_build.upgrades {
                if let Some(synergy) = self
                    .synergy_map
                    .get(&(*owned_upgrade, upgrade_id))
                    .or_else(|| self.synergy_map.get(&(upgrade_id, *owned_upgrade)))
                {
                    self.player_build.add_synergy(synergy.clone());
                }
            }
        }
    }

    pub fn get_player_build(&self) -> &PlayerBuild {
        &self.player_build
    }

    pub fn get_active_synergies(&self) -> &[SynergyBonus] {
        &self.player_build.active_synergies
    }
}

impl Default for UpgradeSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Upgrade {
    pub id: UpgradeId,
    pub name: String,
    pub description: String,
    pub rarity: Rarity,
    pub category: UpgradeCategory,
    pub effects: Vec<Effect>,
    pub prerequisites: Vec<UpgradeId>,
    pub min_zone: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UpgradeCategory {
    Weapon,
    Defense,
    Mobility,
    Utility,
    Special,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Effect {
    StatModifier { stat: Stat, modifier: Modifier },
    AddWeapon { weapon: WeaponId },
    UnlockAbility { ability: AbilityId },
    PassiveEffect { effect: PassiveEffectType },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Stat {
    MaxHealth,
    Armor,
    MoveSpeed,
    FireRate,
    Damage,
    PickupRadius,
    CritChance,
    AbilityCooldown,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Modifier {
    Add(f32),
    Multiply(f32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AbilityId(pub u32);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PassiveEffectType {
    HealthRegen(f32),
    PickupBonus(f32),
    DamageReflection(f32),
    LifeSteal(f32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynergyBonus {
    pub name: String,
    pub description: String,
    pub weight_multiplier: f32,
    pub bonus_effects: Vec<Effect>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlayerBuild {
    pub upgrades: Vec<UpgradeId>,
    pub active_synergies: Vec<SynergyBonus>,
    pub stat_modifiers: HashMap<Stat, f32>,
}

impl PlayerBuild {
    pub fn new() -> Self {
        Self {
            upgrades: Vec::new(),
            active_synergies: Vec::new(),
            stat_modifiers: HashMap::new(),
        }
    }

    pub fn add_upgrade(&mut self, upgrade_id: UpgradeId) {
        if !self.upgrades.contains(&upgrade_id) {
            self.upgrades.push(upgrade_id);
        }
    }

    pub fn has_upgrade(&self, upgrade_id: UpgradeId) -> bool {
        self.upgrades.contains(&upgrade_id)
    }

    pub fn add_synergy(&mut self, synergy: SynergyBonus) {
        self.active_synergies.push(synergy);
    }

    pub fn get_stat_modifier(&self, stat: Stat) -> f32 {
        *self.stat_modifiers.get(&stat).unwrap_or(&1.0)
    }

    pub fn apply_stat_modifier(&mut self, stat: Stat, modifier: Modifier) {
        let current = self.get_stat_modifier(stat);
        let new_value = match modifier {
            Modifier::Add(value) => current + value,
            Modifier::Multiply(value) => current * value,
        };
        self.stat_modifiers.insert(stat, new_value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upgrade_system_creation() {
        let system = UpgradeSystem::new();
        assert!(!system.upgrade_pool.is_empty());
        assert!(!system.synergy_map.is_empty());
    }

    #[test]
    fn test_upgrade_generation() {
        let mut system = UpgradeSystem::new();
        let choices = system.generate_upgrade_choices(3, 1);

        assert!(choices.len() <= 3);
        for choice in &choices {
            assert!(choice.min_zone <= 1);
        }
    }

    #[test]
    fn test_upgrade_application() {
        let mut system = UpgradeSystem::new();
        let upgrade_id = UpgradeId(1);

        system.apply_upgrade(upgrade_id);
        assert!(system.player_build.has_upgrade(upgrade_id));
    }

    #[test]
    fn test_synergy_detection() {
        let mut system = UpgradeSystem::new();

        system.apply_upgrade(UpgradeId(1)); // Rapid Fire
        assert_eq!(system.get_active_synergies().len(), 0);

        system.apply_upgrade(UpgradeId(2)); // Armor Piercing
        assert_eq!(system.get_active_synergies().len(), 1);
        assert_eq!(
            system.get_active_synergies()[0].name,
            "Devastating Assault"
        );
    }

    #[test]
    fn test_rarity_weights() {
        let system = UpgradeSystem::new();
        let weights = system.calculate_upgrade_weights(1);

        let common_weight = weights
            .iter()
            .find(|(u, _)| u.rarity == Rarity::Common)
            .map(|(_, w)| *w)
            .unwrap_or(0.0);

        let rare_weight = weights
            .iter()
            .find(|(u, _)| u.rarity == Rarity::Rare)
            .map(|(_, w)| *w)
            .unwrap_or(0.0);

        assert!(common_weight > rare_weight);
    }

    #[test]
    fn test_player_build() {
        let mut build = PlayerBuild::new();

        build.add_upgrade(UpgradeId(1));
        assert!(build.has_upgrade(UpgradeId(1)));
        assert!(!build.has_upgrade(UpgradeId(2)));

        build.apply_stat_modifier(Stat::Damage, Modifier::Multiply(1.5));
        assert_eq!(build.get_stat_modifier(Stat::Damage), 1.5);

        build.apply_stat_modifier(Stat::Damage, Modifier::Add(0.5));
        assert_eq!(build.get_stat_modifier(Stat::Damage), 2.0);
    }

    #[test]
    fn test_prerequisite_filtering() {
        let mut system = UpgradeSystem::new();

        // Add an upgrade with prerequisites
        system.upgrade_pool.push(Upgrade {
            id: UpgradeId(99),
            name: "Advanced Upgrade".to_string(),
            description: "Requires another upgrade".to_string(),
            rarity: Rarity::Rare,
            category: UpgradeCategory::Weapon,
            effects: vec![],
            prerequisites: vec![UpgradeId(1)],
            min_zone: 1,
        });

        // Should not be available without prerequisite
        let weights_before = system.calculate_upgrade_weights(1);
        assert!(!weights_before.iter().any(|(u, _)| u.id == UpgradeId(99)));

        // Apply prerequisite
        system.apply_upgrade(UpgradeId(1));

        // Should now be available
        let weights_after = system.calculate_upgrade_weights(1);
        assert!(weights_after.iter().any(|(u, _)| u.id == UpgradeId(99)));
    }
}

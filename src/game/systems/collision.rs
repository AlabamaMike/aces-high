use crate::game::components::{Collider, Position};
use crate::game::entities::Entity;
use crate::utils::{Vec2, AABB};
use cgmath::InnerSpace;
use std::collections::{HashMap, HashSet};

pub struct CollisionSystem {
    spatial_grid: SpatialHashGrid,
    collision_pairs: Vec<(Entity, Entity)>,
}

impl CollisionSystem {
    pub fn new(cell_size: f32) -> Self {
        Self {
            spatial_grid: SpatialHashGrid::new(cell_size),
            collision_pairs: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.spatial_grid.clear();
        self.collision_pairs.clear();
    }

    pub fn insert(&mut self, entity: Entity, position: &Position, collider: &Collider) {
        let aabb = collider.get_aabb(position);
        self.spatial_grid.insert(entity, aabb);
    }

    pub fn query_region(&self, region: AABB) -> HashSet<Entity> {
        self.spatial_grid.query(region)
    }

    pub fn get_collisions(&self) -> &[(Entity, Entity)] {
        &self.collision_pairs
    }

    pub fn test_collision(
        pos1: &Position,
        col1: &Collider,
        pos2: &Position,
        col2: &Collider,
    ) -> bool {
        match (col1, col2) {
            (Collider::Circle { radius: r1 }, Collider::Circle { radius: r2 }) => {
                Self::test_circle_circle(pos1.as_vec2(), *r1, pos2.as_vec2(), *r2)
            }
            (Collider::AABB { width: w1, height: h1 }, Collider::AABB { width: w2, height: h2 }) => {
                let aabb1 = AABB::from_center_size(pos1.as_vec2(), Vec2::new(*w1, *h1));
                let aabb2 = AABB::from_center_size(pos2.as_vec2(), Vec2::new(*w2, *h2));
                aabb1.intersects(&aabb2)
            }
            (Collider::Circle { radius: r }, Collider::AABB { width, height }) => {
                let aabb = AABB::from_center_size(pos2.as_vec2(), Vec2::new(*width, *height));
                Self::test_circle_aabb(pos1.as_vec2(), *r, &aabb)
            }
            (Collider::AABB { width, height }, Collider::Circle { radius: r }) => {
                let aabb = AABB::from_center_size(pos1.as_vec2(), Vec2::new(*width, *height));
                Self::test_circle_aabb(pos2.as_vec2(), *r, &aabb)
            }
        }
    }

    fn test_circle_circle(pos1: Vec2, r1: f32, pos2: Vec2, r2: f32) -> bool {
        let dist_sq = (pos1 - pos2).magnitude2();
        let radius_sum = r1 + r2;
        dist_sq < radius_sum * radius_sum
    }

    fn test_circle_aabb(circle_pos: Vec2, radius: f32, aabb: &AABB) -> bool {
        // Find the closest point on the AABB to the circle center
        let closest_x = circle_pos.x.max(aabb.min.x).min(aabb.max.x);
        let closest_y = circle_pos.y.max(aabb.min.y).min(aabb.max.y);
        let closest = Vec2::new(closest_x, closest_y);

        // Calculate distance from circle center to closest point
        let dist_sq = (circle_pos - closest).magnitude2();
        dist_sq < radius * radius
    }
}

impl Default for CollisionSystem {
    fn default() -> Self {
        Self::new(100.0) // Default cell size
    }
}

pub struct SpatialHashGrid {
    cell_size: f32,
    cells: HashMap<(i32, i32), Vec<Entity>>,
}

impl SpatialHashGrid {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            cells: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.cells.clear();
    }

    pub fn insert(&mut self, entity: Entity, aabb: AABB) {
        let min_cell = self.world_to_cell(aabb.min);
        let max_cell = self.world_to_cell(aabb.max);

        for x in min_cell.0..=max_cell.0 {
            for y in min_cell.1..=max_cell.1 {
                self.cells
                    .entry((x, y))
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

    pub fn query_point(&self, point: Vec2) -> Vec<Entity> {
        let cell = self.world_to_cell(point);
        self.cells.get(&cell).cloned().unwrap_or_default()
    }

    fn world_to_cell(&self, pos: Vec2) -> (i32, i32) {
        (
            (pos.x / self.cell_size).floor() as i32,
            (pos.y / self.cell_size).floor() as i32,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_circle_collision() {
        let pos1 = Position::new(0.0, 0.0);
        let col1 = Collider::circle(10.0);

        let pos2 = Position::new(15.0, 0.0);
        let col2 = Collider::circle(10.0);

        // Circles should be touching
        assert!(CollisionSystem::test_collision(&pos1, &col1, &pos2, &col2));

        let pos3 = Position::new(25.0, 0.0);
        let col3 = Collider::circle(10.0);

        // Circles should not be touching
        assert!(!CollisionSystem::test_collision(&pos1, &col1, &pos3, &col3));
    }

    #[test]
    fn test_aabb_aabb_collision() {
        let pos1 = Position::new(0.0, 0.0);
        let col1 = Collider::aabb(10.0, 10.0);

        let pos2 = Position::new(8.0, 0.0);
        let col2 = Collider::aabb(10.0, 10.0);

        // AABBs should be overlapping
        assert!(CollisionSystem::test_collision(&pos1, &col1, &pos2, &col2));

        let pos3 = Position::new(15.0, 0.0);
        let col3 = Collider::aabb(10.0, 10.0);

        // AABBs should not be overlapping
        assert!(!CollisionSystem::test_collision(&pos1, &col1, &pos3, &col3));
    }

    #[test]
    fn test_circle_aabb_collision() {
        let pos1 = Position::new(0.0, 0.0);
        let col1 = Collider::circle(5.0);

        let pos2 = Position::new(8.0, 0.0);
        let col2 = Collider::aabb(6.0, 6.0);

        // Circle should be touching AABB
        assert!(CollisionSystem::test_collision(&pos1, &col1, &pos2, &col2));

        let pos3 = Position::new(15.0, 0.0);
        let col3 = Collider::aabb(6.0, 6.0);

        // Circle should not be touching AABB
        assert!(!CollisionSystem::test_collision(&pos1, &col1, &pos3, &col3));
    }

    #[test]
    fn test_spatial_hash_grid() {
        let mut grid = SpatialHashGrid::new(100.0);
        let entity1 = Entity::new(1);
        let entity2 = Entity::new(2);

        let aabb1 = AABB::new(Vec2::new(0.0, 0.0), Vec2::new(50.0, 50.0));
        let aabb2 = AABB::new(Vec2::new(200.0, 200.0), Vec2::new(250.0, 250.0));

        grid.insert(entity1, aabb1);
        grid.insert(entity2, aabb2);

        // Query near entity1
        let query_result = grid.query(AABB::new(Vec2::new(-10.0, -10.0), Vec2::new(60.0, 60.0)));
        assert!(query_result.contains(&entity1));
        assert!(!query_result.contains(&entity2));

        // Query near entity2
        let query_result = grid.query(AABB::new(Vec2::new(190.0, 190.0), Vec2::new(260.0, 260.0)));
        assert!(!query_result.contains(&entity1));
        assert!(query_result.contains(&entity2));
    }

    #[test]
    fn test_spatial_hash_grid_clear() {
        let mut grid = SpatialHashGrid::new(100.0);
        let entity = Entity::new(1);
        let aabb = AABB::new(Vec2::new(0.0, 0.0), Vec2::new(50.0, 50.0));

        grid.insert(entity, aabb);
        assert!(!grid.query(aabb).is_empty());

        grid.clear();
        assert!(grid.query(aabb).is_empty());
    }
}

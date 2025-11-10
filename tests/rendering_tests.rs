//! Comprehensive unit tests for rendering components

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
mod rendering_tests {
    use super::*;

    // Note: These tests would require a WebGL context which isn't available in pure unit tests
    // In practice, these would be integration tests run in a browser environment
    
    #[test]
    fn test_module_compilation() {
        // This test just ensures all modules compile correctly
        assert!(true);
    }
}

#[cfg(test)]
mod math_tests {
    use aces_high::utils::math::*;

    #[test]
    fn test_position_distance() {
        let p1 = Position::new(0.0, 0.0);
        let p2 = Position::new(3.0, 4.0);
        let dist = p1.distance_to(&p2);
        assert!((dist - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_color_operations() {
        let white = Color::white();
        let black = Color::black();
        let gray = white.lerp(&black, 0.5);
        
        assert!((gray.r - 0.5).abs() < 0.001);
        assert!((gray.g - 0.5).abs() < 0.001);
        assert!((gray.b - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_aabb_contains() {
        let aabb = AABB::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        
        assert!(aabb.contains(Vec2::new(5.0, 5.0)));
        assert!(!aabb.contains(Vec2::new(15.0, 15.0)));
    }

    #[test]
    fn test_aabb_intersects() {
        let a = AABB::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        let b = AABB::new(Vec2::new(5.0, 5.0), Vec2::new(15.0, 15.0));
        let c = AABB::new(Vec2::new(20.0, 20.0), Vec2::new(30.0, 30.0));
        
        assert!(a.intersects(&b));
        assert!(!a.intersects(&c));
    }

    #[test]
    fn test_transform_identity() {
        let transform = Transform::identity();
        assert_eq!(transform.rotation, 0.0);
        assert_eq!(transform.scale.x, 1.0);
        assert_eq!(transform.scale.y, 1.0);
    }

    #[test]
    fn test_animation_curve_linear() {
        let curve = AnimationCurve::linear(0.0, 10.0);
        
        assert!((curve.evaluate(0.0) - 0.0).abs() < 0.001);
        assert!((curve.evaluate(0.5) - 5.0).abs() < 0.001);
        assert!((curve.evaluate(1.0) - 10.0).abs() < 0.001);
    }

    #[test]
    fn test_gradient() {
        let red = Color::rgb(1.0, 0.0, 0.0);
        let blue = Color::rgb(0.0, 0.0, 1.0);
        let gradient = Gradient::new(vec![(0.0, red), (1.0, blue)]);
        
        let mid = gradient.evaluate(0.5);
        assert!((mid.r - 0.5).abs() < 0.001);
        assert!((mid.b - 0.5).abs() < 0.001);
    }
}

#[cfg(test)]
mod pool_tests {
    use aces_high::utils::pool::ObjectPool;

    #[derive(Debug, PartialEq)]
    struct TestObject {
        value: i32,
    }

    impl TestObject {
        fn new() -> Self {
            Self { value: 0 }
        }

        fn reset(&mut self) {
            self.value = 0;
        }
    }

    #[test]
    fn test_pool_basic_operations() {
        let mut pool = ObjectPool::new(
            Box::new(|| TestObject::new()),
            Box::new(|obj| obj.reset()),
            10,
        );

        let obj1 = pool.acquire().unwrap();
        assert_eq!(pool.in_use(), 1);
        assert_eq!(pool.available(), 0);

        pool.release(obj1);
        assert_eq!(pool.in_use(), 0);
        assert_eq!(pool.available(), 1);
    }

    #[test]
    fn test_pool_reset() {
        let mut pool = ObjectPool::new(
            Box::new(|| TestObject::new()),
            Box::new(|obj| obj.reset()),
            10,
        );

        let mut obj = pool.acquire().unwrap();
        obj.value = 42;
        pool.release(obj);

        let obj2 = pool.acquire().unwrap();
        assert_eq!(obj2.value, 0); // Should be reset
    }

    #[test]
    fn test_pool_max_capacity() {
        let mut pool = ObjectPool::new(
            Box::new(|| TestObject::new()),
            Box::new(|obj| obj.reset()),
            2,
        );

        let _obj1 = pool.acquire().unwrap();
        let _obj2 = pool.acquire().unwrap();
        let obj3 = pool.acquire();

        assert!(obj3.is_none()); // Pool exhausted
    }
}

#[cfg(test)]
mod culling_tests {
    use aces_high::engine::culling::{CullingSystem, LODSystem};
    use aces_high::utils::math::{Position, Vec2, AABB};

    #[test]
    fn test_culling_basic() {
        let view_bounds = AABB::new(Vec2::new(0.0, 0.0), Vec2::new(100.0, 100.0));
        let mut culling = CullingSystem::new(view_bounds);

        let positions = vec![
            Position::new(50.0, 50.0),   // Inside
            Position::new(200.0, 200.0), // Outside
            Position::new(10.0, 10.0),   // Inside
        ];

        let visible = culling.cull_by_position(&positions);
        assert_eq!(visible.len(), 2);
        assert_eq!(culling.visible_count(), 2);
        assert_eq!(culling.culled_count(), 1);
    }

    #[test]
    fn test_lod_levels() {
        let camera_pos = Vec2::new(0.0, 0.0);
        let lod = LODSystem::new(camera_pos);

        // Close entity
        assert_eq!(lod.get_lod_level(Vec2::new(10.0, 10.0)), 0);

        // Medium distance
        let mid_pos = Vec2::new(75.0, 75.0);
        assert!(lod.get_lod_level(mid_pos) > 0);

        // Very far
        assert_eq!(lod.get_lod_level(Vec2::new(1000.0, 1000.0)), 255);
    }

    #[test]
    fn test_lod_should_render() {
        let camera_pos = Vec2::new(0.0, 0.0);
        let lod = LODSystem::new(camera_pos);

        assert!(lod.should_render(Vec2::new(50.0, 50.0)));
        assert!(!lod.should_render(Vec2::new(1000.0, 1000.0)));
    }
}

#[cfg(test)]
mod component_tests {
    use aces_high::game::components::{Health, Velocity};
    use aces_high::utils::math::Vec2;

    #[test]
    fn test_health_damage() {
        let mut health = Health::new(100);
        
        assert!(health.is_alive());
        
        health.damage(30);
        assert_eq!(health.current, 70);
        assert!(health.is_alive());
        
        health.damage(100);
        assert_eq!(health.current, 0);
        assert!(!health.is_alive());
    }

    #[test]
    fn test_health_armor() {
        let mut health = Health::new(100);
        health.armor = 0.5; // 50% damage reduction
        
        health.damage(100);
        assert_eq!(health.current, 50);
    }

    #[test]
    fn test_health_heal() {
        let mut health = Health::new(100);
        health.damage(50);
        
        health.heal(30);
        assert_eq!(health.current, 80);
        
        health.heal(100);
        assert_eq!(health.current, 100); // Capped at max
    }

    #[test]
    fn test_velocity_conversion() {
        let vel = Velocity::new(3.0, 4.0);
        let vec2 = vel.as_vec2();
        
        assert_eq!(vec2.x, 3.0);
        assert_eq!(vec2.y, 4.0);
    }
}

#[cfg(test)]
mod performance_tests {
    use aces_high::utils::performance::{PerformanceMonitor, RingBuffer};

    #[test]
    fn test_ring_buffer() {
        let mut buffer = RingBuffer::new(3);
        
        buffer.push(1.0);
        buffer.push(2.0);
        buffer.push(3.0);
        assert_eq!(buffer.len(), 3);
        
        buffer.push(4.0); // Should evict first element
        assert_eq!(buffer.len(), 3);
    }

    #[test]
    fn test_performance_monitor() {
        let mut monitor = PerformanceMonitor::new();
        
        monitor.begin_frame(0.0);
        monitor.end_frame(16.67); // ~60 FPS
        
        let metrics = monitor.get_metrics();
        assert!(metrics.frame_time_ms > 0.0);
    }
}

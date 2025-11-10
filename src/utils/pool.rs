//! Object pooling system for efficient memory management

use std::marker::PhantomData;

/// Object pool for reusing objects
pub struct ObjectPool<T> {
    available: Vec<T>,
    in_use_count: usize,
    factory: Box<dyn Fn() -> T>,
    reset: Box<dyn Fn(&mut T)>,
    max_size: usize,
}

impl<T> ObjectPool<T> {
    pub fn new<F, R>(factory: F, reset: R, max_size: usize) -> Self
    where
        F: Fn() -> T + 'static,
        R: Fn(&mut T) + 'static,
    {
        ObjectPool {
            available: Vec::with_capacity(max_size / 2),
            in_use_count: 0,
            factory: Box::new(factory),
            reset: Box::new(reset),
            max_size,
        }
    }
    
    pub fn acquire(&mut self) -> Option<T> {
        if let Some(obj) = self.available.pop() {
            self.in_use_count += 1;
            Some(obj)
        } else if self.in_use_count < self.max_size {
            self.in_use_count += 1;
            Some((self.factory)())
        } else {
            None
        }
    }
    
    pub fn release(&mut self, mut obj: T) {
        (self.reset)(&mut obj);
        self.available.push(obj);
        self.in_use_count = self.in_use_count.saturating_sub(1);
    }
    
    pub fn available_count(&self) -> usize {
        self.available.len()
    }
    
    pub fn in_use_count(&self) -> usize {
        self.in_use_count
    }
    
    pub fn capacity(&self) -> usize {
        self.max_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[derive(Debug, Clone, PartialEq)]
    struct TestObject {
        value: i32,
    }
    
    impl TestObject {
        fn new() -> Self {
            TestObject { value: 0 }
        }
        
        fn reset(&mut self) {
            self.value = 0;
        }
    }
    
    #[test]
    fn test_object_pool_creation() {
        let pool: ObjectPool<TestObject> = ObjectPool::new(
            || TestObject::new(),
            |obj| obj.reset(),
            10
        );
        
        assert_eq!(pool.available_count(), 0);
        assert_eq!(pool.in_use_count(), 0);
        assert_eq!(pool.capacity(), 10);
    }
    
    #[wasm_bindgen_test]
    fn test_object_pool_creation_wasm() {
        let pool: ObjectPool<TestObject> = ObjectPool::new(
            || TestObject::new(),
            |obj| obj.reset(),
            10
        );
        
        assert_eq!(pool.capacity(), 10);
    }
    
    #[test]
    fn test_object_pool_acquire() {
        let mut pool = ObjectPool::new(
            || TestObject::new(),
            |obj| obj.reset(),
            10
        );
        
        let obj = pool.acquire();
        assert!(obj.is_some());
        assert_eq!(pool.in_use_count(), 1);
    }
    
    #[wasm_bindgen_test]
    fn test_object_pool_acquire_wasm() {
        let mut pool = ObjectPool::new(
            || TestObject::new(),
            |obj| obj.reset(),
            10
        );
        
        let obj = pool.acquire();
        assert!(obj.is_some());
    }
    
    #[test]
    fn test_object_pool_release() {
        let mut pool = ObjectPool::new(
            || TestObject::new(),
            |obj| obj.reset(),
            10
        );
        
        let mut obj = pool.acquire().unwrap();
        obj.value = 42;
        pool.release(obj);
        
        assert_eq!(pool.available_count(), 1);
        assert_eq!(pool.in_use_count(), 0);
        
        let recycled = pool.acquire().unwrap();
        assert_eq!(recycled.value, 0); // Should be reset
    }
    
    #[wasm_bindgen_test]
    fn test_object_pool_release_wasm() {
        let mut pool = ObjectPool::new(
            || TestObject::new(),
            |obj| obj.reset(),
            10
        );
        
        let mut obj = pool.acquire().unwrap();
        obj.value = 42;
        pool.release(obj);
        
        assert_eq!(pool.available_count(), 1);
    }
    
    #[test]
    fn test_object_pool_max_capacity() {
        let mut pool = ObjectPool::new(
            || TestObject::new(),
            |obj| obj.reset(),
            3
        );
        
        let obj1 = pool.acquire();
        let obj2 = pool.acquire();
        let obj3 = pool.acquire();
        let obj4 = pool.acquire();
        
        assert!(obj1.is_some());
        assert!(obj2.is_some());
        assert!(obj3.is_some());
        assert!(obj4.is_none()); // Exceeds capacity
    }
    
    #[wasm_bindgen_test]
    fn test_object_pool_max_capacity_wasm() {
        let mut pool = ObjectPool::new(
            || TestObject::new(),
            |obj| obj.reset(),
            3
        );
        
        let obj1 = pool.acquire();
        let obj2 = pool.acquire();
        let obj3 = pool.acquire();
        let obj4 = pool.acquire();
        
        assert!(obj1.is_some());
        assert!(obj2.is_some());
        assert!(obj3.is_some());
        assert!(obj4.is_none());
    }
    
    #[test]
    fn test_object_pool_reuse() {
        let mut pool = ObjectPool::new(
            || TestObject::new(),
            |obj| obj.reset(),
            10
        );
        
        let obj1 = pool.acquire().unwrap();
        pool.release(obj1);
        
        let obj2 = pool.acquire().unwrap();
        assert_eq!(pool.in_use_count(), 1);
        assert_eq!(pool.available_count(), 0);
    }
    
    #[wasm_bindgen_test]
    fn test_object_pool_reuse_wasm() {
        let mut pool = ObjectPool::new(
            || TestObject::new(),
            |obj| obj.reset(),
            10
        );
        
        let obj1 = pool.acquire().unwrap();
        pool.release(obj1);
        
        let obj2 = pool.acquire().unwrap();
        assert_eq!(pool.in_use_count(), 1);
    }
}

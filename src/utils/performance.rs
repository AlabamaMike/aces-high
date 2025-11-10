//! Performance monitoring and profiling utilities
//!
//! This module provides tools for tracking frame times, FPS, memory usage,
//! and other performance metrics.

use std::collections::VecDeque;

/// Ring buffer for storing a fixed number of recent values
pub struct RingBuffer<T> {
    data: VecDeque<T>,
    capacity: usize,
}

impl<T> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.data.len() >= self.capacity {
            self.data.pop_front();
        }
        self.data.push_back(value);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl RingBuffer<f32> {
    pub fn average(&self) -> f32 {
        if self.data.is_empty() {
            return 0.0;
        }
        let sum: f32 = self.data.iter().sum();
        sum / self.data.len() as f32
    }

    pub fn min(&self) -> f32 {
        self.data
            .iter()
            .copied()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0)
    }

    pub fn max(&self) -> f32 {
        self.data
            .iter()
            .copied()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0)
    }
}

/// Memory usage metrics
#[derive(Debug, Clone, Copy, Default)]
pub struct MemoryMetrics {
    pub heap_used: usize,
    pub heap_total: usize,
    pub wasm_memory: usize,
}

impl MemoryMetrics {
    pub fn used_mb(&self) -> f32 {
        self.heap_used as f32 / (1024.0 * 1024.0)
    }

    pub fn total_mb(&self) -> f32 {
        self.heap_total as f32 / (1024.0 * 1024.0)
    }

    pub fn wasm_mb(&self) -> f32 {
        self.wasm_memory as f32 / (1024.0 * 1024.0)
    }
}

/// Performance metrics for a frame
#[derive(Debug, Clone, Copy)]
pub struct PerformanceMetrics {
    pub fps: f32,
    pub frame_time_ms: f32,
    pub update_time_ms: f32,
    pub render_time_ms: f32,
    pub memory_used_mb: f32,
    pub draw_calls: u32,
    pub triangles: u32,
    pub entities: u32,
}

/// Performance monitor that tracks various metrics over time
pub struct PerformanceMonitor {
    frame_times: RingBuffer<f32>,
    update_times: RingBuffer<f32>,
    render_times: RingBuffer<f32>,
    
    frame_start: f64,
    update_start: f64,
    render_start: f64,
    
    pub memory_usage: MemoryMetrics,
    pub draw_calls: u32,
    pub triangles_drawn: u32,
    pub entity_count: u32,
    
    sample_count: usize,
}

impl PerformanceMonitor {
    pub fn new(sample_count: usize) -> Self {
        Self {
            frame_times: RingBuffer::new(sample_count),
            update_times: RingBuffer::new(sample_count),
            render_times: RingBuffer::new(sample_count),
            frame_start: 0.0,
            update_start: 0.0,
            render_start: 0.0,
            memory_usage: MemoryMetrics::default(),
            draw_calls: 0,
            triangles_drawn: 0,
            entity_count: 0,
            sample_count,
        }
    }

    pub fn begin_frame(&mut self, current_time: f64) {
        self.frame_start = current_time;
        self.draw_calls = 0;
        self.triangles_drawn = 0;
    }

    pub fn begin_update(&mut self, current_time: f64) {
        self.update_start = current_time;
    }

    pub fn end_update(&mut self, current_time: f64) {
        let update_time = (current_time - self.update_start) as f32 * 1000.0;
        self.update_times.push(update_time);
    }

    pub fn begin_render(&mut self, current_time: f64) {
        self.render_start = current_time;
    }

    pub fn end_render(&mut self, current_time: f64) {
        let render_time = (current_time - self.render_start) as f32 * 1000.0;
        self.render_times.push(render_time);
    }

    pub fn end_frame(&mut self, current_time: f64) {
        let frame_time = (current_time - self.frame_start) as f32 * 1000.0;
        self.frame_times.push(frame_time);

        // Check for performance issues
        if frame_time > 16.67 {
            // Below 60 FPS
            self.log_performance_warning(frame_time);
        }
    }

    pub fn get_average_fps(&self) -> f32 {
        let avg_frame_time = self.frame_times.average();
        if avg_frame_time > 0.0 {
            1000.0 / avg_frame_time
        } else {
            0.0
        }
    }

    pub fn get_average_frame_time(&self) -> f32 {
        self.frame_times.average()
    }

    pub fn get_min_fps(&self) -> f32 {
        let max_frame_time = self.frame_times.max();
        if max_frame_time > 0.0 {
            1000.0 / max_frame_time
        } else {
            0.0
        }
    }

    pub fn get_max_fps(&self) -> f32 {
        let min_frame_time = self.frame_times.min();
        if min_frame_time > 0.0 {
            1000.0 / min_frame_time
        } else {
            0.0
        }
    }

    pub fn get_metrics(&self) -> PerformanceMetrics {
        PerformanceMetrics {
            fps: self.get_average_fps(),
            frame_time_ms: self.frame_times.average(),
            update_time_ms: self.update_times.average(),
            render_time_ms: self.render_times.average(),
            memory_used_mb: self.memory_usage.used_mb(),
            draw_calls: self.draw_calls,
            triangles: self.triangles_drawn,
            entities: self.entity_count,
        }
    }

    pub fn should_reduce_quality(&self) -> bool {
        self.get_average_fps() < 55.0
    }

    pub fn should_increase_quality(&self) -> bool {
        self.get_average_fps() > 65.0 && self.get_min_fps() > 60.0
    }

    fn log_performance_warning(&self, frame_time: f32) {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::prelude::*;
            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen(js_namespace = console, js_name = warn)]
                fn console_warn(s: &str);
            }
            console_warn(&format!(
                "Performance warning: Frame time {}ms (target: 16.67ms)",
                frame_time
            ));
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            eprintln!(
                "Performance warning: Frame time {}ms (target: 16.67ms)",
                frame_time
            );
        }
    }

    pub fn reset(&mut self) {
        self.frame_times.clear();
        self.update_times.clear();
        self.render_times.clear();
        self.draw_calls = 0;
        self.triangles_drawn = 0;
        self.entity_count = 0;
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new(120) // 2 seconds at 60 FPS
    }
}

/// Simple timer for profiling code sections
pub struct Timer {
    start: instant::Instant,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start: instant::Instant::now(),
        }
    }

    pub fn elapsed_ms(&self) -> f32 {
        self.start.elapsed().as_secs_f32() * 1000.0
    }

    pub fn elapsed_us(&self) -> f32 {
        self.start.elapsed().as_secs_f32() * 1_000_000.0
    }

    pub fn reset(&mut self) {
        self.start = instant::Instant::now();
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ring_buffer() {
        let mut buffer = RingBuffer::new(3);
        buffer.push(1.0);
        buffer.push(2.0);
        buffer.push(3.0);
        
        assert_eq!(buffer.len(), 3);
        assert_eq!(buffer.average(), 2.0);
        
        buffer.push(4.0); // Should push out 1.0
        
        assert_eq!(buffer.len(), 3);
        assert_eq!(buffer.average(), 3.0); // (2 + 3 + 4) / 3
    }

    #[test]
    fn test_ring_buffer_min_max() {
        let mut buffer = RingBuffer::new(5);
        buffer.push(10.0);
        buffer.push(5.0);
        buffer.push(20.0);
        buffer.push(3.0);
        buffer.push(15.0);
        
        assert_eq!(buffer.min(), 3.0);
        assert_eq!(buffer.max(), 20.0);
    }

    #[test]
    fn test_performance_monitor() {
        let mut monitor = PerformanceMonitor::new(60);
        
        monitor.begin_frame(0.0);
        monitor.begin_update(0.0);
        monitor.end_update(0.005); // 5ms update
        monitor.begin_render(0.005);
        monitor.end_render(0.015); // 10ms render
        monitor.end_frame(0.016); // 16ms total
        
        assert!(monitor.get_average_frame_time() > 0.0);
        assert!(monitor.get_average_fps() > 0.0);
    }

    #[test]
    fn test_timer() {
        let timer = Timer::new();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let elapsed = timer.elapsed_ms();
        assert!(elapsed >= 10.0);
    }
}

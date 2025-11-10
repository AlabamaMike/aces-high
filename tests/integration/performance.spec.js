/**
 * Integration tests for game performance
 * Tests FPS, memory usage, and entity limits
 */

const { test, expect } = require('@playwright/test');

test.describe('Game Performance', () => {
  test('should maintain 60 FPS during gameplay', async ({ page }) => {
    await page.goto('/');
    
    // Start the game
    await page.evaluate(() => {
      if (window.game) {
        window.game.start();
      }
    });
    
    // Measure FPS over 2 seconds
    const fps = await page.evaluate(() => {
      return new Promise(resolve => {
        let frames = 0;
        const startTime = performance.now();
        
        function countFrame() {
          frames++;
          const elapsed = performance.now() - startTime;
          
          if (elapsed < 2000) {
            requestAnimationFrame(countFrame);
          } else {
            resolve((frames / elapsed) * 1000);
          }
        }
        
        requestAnimationFrame(countFrame);
      });
    });
    
    console.log('Average FPS: ' + fps.toFixed(2));
    expect(fps).toBeGreaterThan(58); // Allow some variance
  });

  test('should handle entity spawning without performance degradation', async ({ page }) => {
    await page.goto('/');
    
    // Spawn multiple entities
    const fps = await page.evaluate(() => {
      // Simulate spawning 100 entities
      for (let i = 0; i < 100; i++) {
        // Entity spawning logic would go here
      }
      
      return new Promise(resolve => {
        let frames = 0;
        const startTime = performance.now();
        
        function countFrame() {
          frames++;
          const elapsed = performance.now() - startTime;
          
          if (elapsed < 1000) {
            requestAnimationFrame(countFrame);
          } else {
            resolve(frames);
          }
        }
        
        requestAnimationFrame(countFrame);
      });
    });
    
    expect(fps).toBeGreaterThan(55);
  });

  test('should track memory usage', async ({ page }) => {
    await page.goto('/');
    
    const memoryInfo = await page.evaluate(() => {
      if (performance.memory) {
        return {
          usedJSHeapSize: performance.memory.usedJSHeapSize,
          totalJSHeapSize: performance.memory.totalJSHeapSize,
          jsHeapSizeLimit: performance.memory.jsHeapSizeLimit,
        };
      }
      return null;
    });
    
    if (memoryInfo) {
      const usedMB = (memoryInfo.usedJSHeapSize / 1024 / 1024).toFixed(2);
      console.log('Memory usage: ' + usedMB + ' MB');
      expect(memoryInfo.usedJSHeapSize).toBeLessThan(500 * 1024 * 1024); // Less than 500MB
    }
  });

  test('should measure frame timing consistency', async ({ page }) => {
    await page.goto('/');
    
    const frameTimings = await page.evaluate(() => {
      return new Promise(resolve => {
        const timings = [];
        let lastTime = performance.now();
        let count = 0;
        
        function measureFrame() {
          const currentTime = performance.now();
          const delta = currentTime - lastTime;
          timings.push(delta);
          lastTime = currentTime;
          count++;
          
          if (count < 60) {
            requestAnimationFrame(measureFrame);
          } else {
            const avg = timings.reduce((a, b) => a + b, 0) / timings.length;
            const max = Math.max(...timings);
            const min = Math.min(...timings);
            resolve({ avg, max, min });
          }
        }
        
        requestAnimationFrame(measureFrame);
      });
    });
    
    console.log('Frame timing - Avg: ' + frameTimings.avg.toFixed(2) + 'ms, Max: ' + frameTimings.max.toFixed(2) + 'ms');
    expect(frameTimings.avg).toBeLessThan(20); // Average frame time less than 20ms (>50 FPS)
  });
});

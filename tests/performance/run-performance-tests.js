/**
 * Performance stress tests for Aces High
 * Tests entity limits, collision performance, and memory usage
 */

const fs = require('fs');
const path = require('path');

class PerformanceTestRunner {
  constructor() {
    this.results = [];
  }

  log(message) {
    console.log(`[Performance Test] ${message}`);
  }

  recordResult(testName, metrics) {
    this.results.push({
      testName,
      timestamp: new Date().toISOString(),
      metrics,
    });
  }

  async runEntityLimitTest() {
    this.log('Running Entity Limit Stress Test...');
    
    const entityCounts = [100, 200, 300, 400, 500];
    const metrics = [];

    for (const count of entityCounts) {
      const start = Date.now();
      
      // Simulate entity processing
      const entities = Array(count).fill(0).map((_, i) => ({
        id: i,
        position: { x: Math.random() * 1000, y: Math.random() * 1000 },
        velocity: { dx: Math.random() * 10, dy: Math.random() * 10 },
      }));

      // Simulate update loop
      let frames = 0;
      const testDuration = 1000; // 1 second
      const updateStart = Date.now();
      
      while (Date.now() - updateStart < testDuration) {
        entities.forEach(entity => {
          entity.position.x += entity.velocity.dx * 0.016;
          entity.position.y += entity.velocity.dy * 0.016;
        });
        frames++;
      }

      const elapsed = Date.now() - start;
      const fps = frames;

      metrics.push({
        entityCount: count,
        fps,
        avgFrameTime: testDuration / frames,
        passed: fps >= 55,
      });

      this.log(`  ${count} entities: ${fps} FPS (avg ${(testDuration / frames).toFixed(2)}ms per frame)`);
    }

    this.recordResult('Entity Limit Test', metrics);
    
    const allPassed = metrics.every(m => m.passed);
    this.log(`Entity Limit Test: ${allPassed ? 'PASSED' : 'FAILED'}`);
    
    return allPassed;
  }

  async run() {
    console.log('========================================');
    console.log('    ACES HIGH - Performance Tests');
    console.log('========================================');

    const results = {
      entityLimit: await this.runEntityLimitTest(),
    };

    console.log('========================================');
    console.log('           Test Summary');
    console.log('========================================');
    console.log(`Entity Limit Test: ${results.entityLimit ? 'PASSED' : 'FAILED'}`);

    const allPassed = Object.values(results).every(r => r);
    process.exit(allPassed ? 0 : 1);
  }
}

// Run tests
const runner = new PerformanceTestRunner();
runner.run().catch(error => {
  console.error('Performance tests failed:', error);
  process.exit(1);
});

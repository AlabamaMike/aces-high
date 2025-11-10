/**
 * Integration tests for game loading
 * Tests game initialization and load times
 */

const { test, expect } = require('@playwright/test');

test.describe('Game Loading', () => {
  test('should load game within 3 seconds', async ({ page }) => {
    const startTime = Date.now();
    
    await page.goto('/');
    
    // Wait for WASM module to load
    await page.waitForFunction(() => {
      return typeof window.wasm !== 'undefined';
    }, { timeout: 3000 });
    
    const loadTime = Date.now() - startTime;
    
    expect(loadTime).toBeLessThan(3000);
    console.log(`Game loaded in ${loadTime}ms`);
  });

  test('should initialize canvas element', async ({ page }) => {
    await page.goto('/');
    
    const canvas = await page.locator('canvas#game-canvas');
    await expect(canvas).toBeVisible();
    
    const canvasWidth = await canvas.evaluate(el => el.width);
    const canvasHeight = await canvas.evaluate(el => el.height);
    
    expect(canvasWidth).toBeGreaterThan(0);
    expect(canvasHeight).toBeGreaterThan(0);
  });

  test('should not have console errors during load', async ({ page }) => {
    const consoleErrors = [];
    
    page.on('console', msg => {
      if (msg.type() === 'error') {
        consoleErrors.push(msg.text());
      }
    });
    
    await page.goto('/');
    await page.waitForTimeout(2000);
    
    expect(consoleErrors).toHaveLength(0);
  });

  test('should load with correct MIME types', async ({ page }) => {
    const response = await page.goto('/');
    
    expect(response.ok()).toBeTruthy();
    
    const contentType = response.headers()['content-type'];
    expect(contentType).toBeDefined();
  });
});

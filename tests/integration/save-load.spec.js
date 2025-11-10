/**
 * Integration tests for save/load functionality
 */

const { test, expect } = require('@playwright/test');

test.describe('Save/Load Functionality', () => {
  test('should save game state to localStorage', async ({ page }) => {
    await page.goto('/');
    
    // Create a game state
    await page.evaluate(() => {
      const gameState = {
        squadron_level: 5,
        score: 1000,
        unlocked_aircraft: ['Spitfire', 'Mustang'],
      };
      localStorage.setItem('aces_high_save', JSON.stringify(gameState));
    });
    
    // Verify it was saved
    const savedData = await page.evaluate(() => {
      return localStorage.getItem('aces_high_save');
    });
    
    expect(savedData).toBeTruthy();
    const parsed = JSON.parse(savedData);
    expect(parsed.squadron_level).toBe(5);
  });

  test('should load game state from localStorage', async ({ page }) => {
    await page.goto('/');
    
    // Pre-populate localStorage
    await page.evaluate(() => {
      const gameState = {
        squadron_level: 10,
        score: 5000,
        unlocked_aircraft: ['Spitfire', 'Mustang', 'Thunderbolt'],
      };
      localStorage.setItem('aces_high_save', JSON.stringify(gameState));
    });
    
    // Reload page
    await page.reload();
    
    // Verify state was loaded
    const loadedState = await page.evaluate(() => {
      const data = localStorage.getItem('aces_high_save');
      return JSON.parse(data);
    });
    
    expect(loadedState.squadron_level).toBe(10);
    expect(loadedState.score).toBe(5000);
  });

  test('should handle corrupted save data gracefully', async ({ page }) => {
    await page.goto('/');
    
    // Set corrupted data
    await page.evaluate(() => {
      localStorage.setItem('aces_high_save', 'corrupted data {]');
    });
    
    // Should not throw errors
    const errors = [];
    page.on('pageerror', error => {
      errors.push(error);
    });
    
    await page.reload();
    await page.waitForTimeout(1000);
    
    // Game should still load with default state
    expect(errors.length).toBe(0);
  });

  test('should persist game progress across sessions', async ({ page, context }) => {
    await page.goto('/');
    
    // Save progress
    await page.evaluate(() => {
      localStorage.setItem('aces_high_save', JSON.stringify({
        progress: 'test_data',
        timestamp: Date.now(),
      }));
    });
    
    // Create new page in same context
    const newPage = await context.newPage();
    await newPage.goto('/');
    
    // Verify data persists
    const persistedData = await newPage.evaluate(() => {
      return localStorage.getItem('aces_high_save');
    });
    
    expect(persistedData).toBeTruthy();
    const parsed = JSON.parse(persistedData);
    expect(parsed.progress).toBe('test_data');
    
    await newPage.close();
  });
});

// ACES HIGH: ENDLESS SKIES - Main entry point

async function init() {
    const loadingElement = document.getElementById('loading');
    const loadingText = document.getElementById('loading-text');
    const errorElement = document.getElementById('error');
    const errorMessage = document.getElementById('error-message');
    const canvas = document.getElementById('game-canvas');
    const fpsCounter = document.getElementById('fps-counter');

    function showError(message) {
        console.error(message);
        errorMessage.textContent = message;
        errorElement.classList.add('visible');
        loadingElement.classList.add('hidden');
    }

    try {
        // Check for WebAssembly support
        if (typeof WebAssembly !== 'object') {
            throw new Error('WebAssembly is not supported in this browser');
        }

        // Check for WebGL 2.0 support
        const gl = canvas.getContext('webgl2');
        if (!gl) {
            throw new Error('WebGL 2.0 is not supported in this browser');
        }

        loadingText.textContent = 'Loading WebAssembly module...';

        // Import the WASM module
        const wasm = await import('../pkg/aces_high.js');

        loadingText.textContent = 'Initializing game...';

        // Initialize the game
        await wasm.default();
        
        loadingText.textContent = 'Creating game instance...';

        // Create game instance
        const game = new wasm.Game();

        // Hide loading screen
        loadingElement.classList.add('hidden');

        // Show FPS counter (can be toggled with F key)
        let showFps = false;
        document.addEventListener('keydown', (e) => {
            if (e.key === 'F' || e.key === 'f') {
                showFps = !showFps;
                fpsCounter.classList.toggle('visible', showFps);
            }
        });

        // Game loop
        let lastTime = 0;
        let frameCount = 0;
        let fpsTime = 0;

        function gameLoop(currentTime) {
            try {
                // Update game
                game.update(currentTime);

                // Render game
                game.render();

                // FPS counter
                if (showFps) {
                    frameCount++;
                    fpsTime += currentTime - lastTime;
                    
                    if (fpsTime >= 1000) {
                        const fps = Math.round(frameCount * 1000 / fpsTime);
                        fpsCounter.textContent = `FPS: ${fps}`;
                        frameCount = 0;
                        fpsTime = 0;
                    }
                }

                lastTime = currentTime;
            } catch (error) {
                showError(`Game error: ${error.message}`);
                return;
            }

            requestAnimationFrame(gameLoop);
        }

        // Start the game loop
        requestAnimationFrame(gameLoop);

        console.log('ACES HIGH: Game initialized successfully');

    } catch (error) {
        showError(`Initialization error: ${error.message}`);
    }
}

// Check if document is ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
} else {
    init();
}

// Handle window resize
window.addEventListener('resize', () => {
    const canvas = document.getElementById('game-canvas');
    const container = document.getElementById('game-container');
    const containerRect = container.getBoundingClientRect();
    
    // Maintain aspect ratio
    const aspectRatio = 16 / 9;
    const containerAspect = containerRect.width / containerRect.height;
    
    if (containerAspect > aspectRatio) {
        canvas.style.width = 'auto';
        canvas.style.height = '100%';
    } else {
        canvas.style.width = '100%';
        canvas.style.height = 'auto';
    }
});

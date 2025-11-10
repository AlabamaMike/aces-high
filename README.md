# ACES HIGH: ENDLESS SKIES

A WebAssembly-based roguelike arcade shooter featuring HD graphics, procedural generation, and meta-progression systems.

![Build Status](https://img.shields.io/badge/build-in%20development-yellow)
![License](https://img.shields.io/badge/license-MIT-blue)
![WASM](https://img.shields.io/badge/WebAssembly-supported-brightgreen)

## Overview

**ACES HIGH: ENDLESS SKIES** is a high-performance WWII-themed vertical scrolling shoot-em-up that runs entirely in the browser using WebAssembly. Battle through procedurally generated zones, unlock powerful aircraft, and build unique loadouts with synergistic upgrades.

### Key Features

- **WebAssembly Performance**: Smooth 60 FPS gameplay at 1080p
- **Roguelike Progression**: Procedurally generated runs with permanent unlocks
- **Five Unique Aircraft**: Spitfire, Mustang, Corsair, Thunderbolt, and Lightning
- **Deep Upgrade System**: Build-defining upgrades with powerful synergies
- **Meta-Progression**: Squadron levels and aircraft mastery
- **HD Graphics**: WebGL 2.0 rendering with particle effects
- **Spatial Audio**: Immersive Web Audio API soundscapes

## Quick Start

### Prerequisites

- **Rust** 1.70+ with `wasm32-unknown-unknown` target
- **Node.js** 18+ and npm 9+
- **wasm-pack** 0.12+

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/aces-high.git
cd aces-high

# Install Rust WASM target
rustup target add wasm32-unknown-unknown

# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install Node dependencies
npm install
```

### Development

```bash
# Start development server with hot reload
npm run dev

# Open browser to http://localhost:8080
```

### Building for Production

```bash
# Build optimized WASM and bundle assets
npm run build:prod

# Output will be in ./dist directory
```

## Project Structure

```
aces-high/
├── src/
│   ├── lib.rs                 # Main entry point
│   ├── engine/                # Core engine systems
│   │   ├── renderer.rs        # WebGL rendering
│   │   ├── audio.rs           # Audio system
│   │   └── input.rs           # Input handling
│   ├── game/                  # Game logic
│   │   ├── components.rs      # ECS components
│   │   ├── entities.rs        # Entity definitions
│   │   ├── state.rs           # Game state management
│   │   ├── systems/           # Game systems
│   │   │   ├── collision.rs   # Collision detection
│   │   │   ├── weapon.rs      # Weapon system
│   │   │   ├── ai.rs          # Enemy AI
│   │   │   ├── procedural.rs  # Level generation
│   │   │   └── upgrade.rs     # Upgrade system
│   │   └── mod.rs
│   ├── utils/                 # Utilities
│   │   ├── math.rs            # Math helpers
│   │   ├── random.rs          # RNG utilities
│   │   ├── pool.rs            # Object pooling
│   │   └── performance.rs     # Performance monitoring
│   └── web/                   # Web bindings
│       ├── bindings.rs        # JS interop
│       └── storage.rs         # IndexedDB persistence
├── assets/                    # Game assets
│   ├── sprites/               # Sprite sheets
│   ├── audio/                 # Sound effects and music
│   └── data/                  # Game data files
├── tests/                     # Test suites
│   ├── unit/                  # Rust unit tests
│   └── integration/           # Playwright tests
├── Cargo.toml                 # Rust dependencies
├── package.json               # Node dependencies
├── webpack.config.js          # Build configuration
└── README.md                  # This file
```

## Architecture

### Technology Stack

- **Language**: Rust 2021 Edition
- **Target**: WebAssembly (wasm32-unknown-unknown)
- **Rendering**: WebGL 2.0 (with WebGL 1.0 fallback)
- **Audio**: Web Audio API
- **Storage**: IndexedDB for save data
- **Build**: wasm-pack + webpack 5

### Performance Targets

| Platform | Resolution | Target FPS | Max Entities |
|----------|-----------|------------|--------------|
| Desktop  | 1920x1080 | 60 FPS     | 500          |
| Mobile   | 1280x720  | 30-60 FPS  | 200          |

### Memory Budget

- Initial Load: < 100MB
- Runtime Peak: < 500MB
- Texture Memory: < 256MB
- Audio Memory: < 64MB

## Gameplay

### Controls

#### Keyboard
- **WASD / Arrow Keys**: Movement
- **Space**: Primary weapon
- **Shift**: Special weapon
- **B**: Drop bomb
- **Tab**: Boost
- **ESC**: Pause menu

#### Mouse
- **Move**: Aim direction
- **Left Click**: Primary weapon
- **Right Click**: Special weapon

#### Gamepad
- **Left Stick**: Movement
- **A Button**: Primary weapon
- **B Button**: Special weapon
- **X Button**: Bomb
- **Right Trigger**: Boost

### Aircraft Types

#### Spitfire (Starter)
- Balanced stats, good for learning
- Medium speed and firepower
- Starting aircraft, always unlocked

#### Mustang (Unlocked: Squadron Level 5)
- High speed, lower health
- Excels at hit-and-run tactics
- Bonus dodge chance

#### Corsair (Unlocked: Squadron Level 10)
- Heavy firepower, slower movement
- High health and armor
- Devastating special weapons

#### Thunderbolt (Unlocked: Squadron Level 15)
- Tank role with highest health
- Regenerating armor
- AoE damage bonuses

#### Lightning (Unlocked: Squadron Level 20)
- Twin-engine speed demon
- Dual weapons simultaneously
- Chain lightning special

## Development

### Running Tests

```bash
# Run Rust unit tests
cargo test

# Run WASM tests in browser
wasm-pack test --headless --chrome

# Run integration tests
npm run test:integration

# Run performance tests
npm run test:performance
```

### Code Style

```bash
# Format Rust code
cargo fmt

# Lint Rust code
cargo clippy

# Format JavaScript
npm run format

# Lint JavaScript
npm run lint
```

### Performance Profiling

```bash
# Build with profiling enabled
cargo build --target wasm32-unknown-unknown --profile profiling

# Open Chrome DevTools > Performance
# Record gameplay session
# Analyze frame times and bottlenecks
```

## Build Optimization

### WASM Binary Size

The release build applies aggressive optimizations:

- **Link-Time Optimization (LTO)**: Enabled
- **Optimization Level**: `z` (optimize for size)
- **Codegen Units**: 1 (maximum optimization)
- **Panic Strategy**: Abort (smaller binary)
- **Strip Symbols**: Yes

Typical binary sizes:
- Development: ~5MB
- Release (uncompressed): ~800KB
- Release (gzipped): ~300KB

### Asset Optimization

- Sprites: PNG with pngcrush optimization
- Audio: OGG Vorbis at 96-128kbps
- Sprite sheets: Packed with TexturePacker
- Progressive loading: Critical assets first

## Deployment

### Static Hosting (Recommended)

```bash
# Build for production
npm run build:prod

# Deploy dist/ to any static host:
# - GitHub Pages
# - Netlify
# - Vercel
# - AWS S3 + CloudFront
# - Firebase Hosting
```

### Required HTTP Headers

```
Cross-Origin-Embedder-Policy: require-corp
Cross-Origin-Opener-Policy: same-origin
```

These headers are required for SharedArrayBuffer support (future multiplayer features).

### CDN Configuration

```nginx
# Enable compression
gzip on;
gzip_types application/wasm application/javascript text/css;

# Cache static assets
location ~* \.(wasm|js|css|png|jpg|ogg)$ {
    expires 1y;
    add_header Cache-Control "public, immutable";
}
```

## Accessibility

### Supported Features

- **Visual**
  - Colorblind modes (Protanopia, Deuteranopia, Tritanopia)
  - Contrast adjustment
  - UI scaling (75% - 150%)
  - Reduced motion option

- **Audio**
  - Subtitle support for audio cues
  - Visual indicators for off-screen threats
  - Adjustable audio mix

- **Controls**
  - Fully remappable controls
  - One-handed mode
  - Hold-to-fire toggle
  - Difficulty accessibility options

## Browser Compatibility

### Minimum Requirements

| Browser | Version | WebGL | WASM | Notes |
|---------|---------|-------|------|-------|
| Chrome  | 89+     | 2.0   | ✅   | Recommended |
| Firefox | 87+     | 2.0   | ✅   | Full support |
| Safari  | 15+     | 2.0   | ✅   | Some audio limitations |
| Edge    | 89+     | 2.0   | ✅   | Chromium-based |

### Feature Detection

The game automatically detects and adapts to:
- WebGL 2.0 vs 1.0
- Available input devices (keyboard, mouse, gamepad)
- Audio context capabilities
- Available storage (IndexedDB)

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Workflow

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass (`npm test`)
6. Format code (`cargo fmt && npm run format`)
7. Commit your changes (`git commit -m 'Add amazing feature'`)
8. Push to the branch (`git push origin feature/amazing-feature`)
9. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by classic arcade shooters like 1942, Raiden, and DoDonPachi
- Built with Rust and WebAssembly
- WebGL rendering powered by glow
- Mathematics powered by cgmath

## Support

- **Documentation**: [Wiki](https://github.com/yourusername/aces-high/wiki)
- **Issues**: [GitHub Issues](https://github.com/yourusername/aces-high/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/aces-high/discussions)

## Roadmap

- [x] Core engine architecture
- [x] Basic gameplay systems
- [ ] All 5 aircraft implemented
- [ ] Complete upgrade system
- [ ] Boss battles
- [ ] Achievements system
- [ ] Leaderboards
- [ ] Multiplayer co-op (future)
- [ ] Mobile touch controls

---

**Made with ❤️ and Rust**

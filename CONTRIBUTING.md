# Contributing to ACES HIGH: ENDLESS SKIES

First off, thank you for considering contributing to ACES HIGH! It's people like you that make this project great.

## Code of Conduct

This project and everyone participating in it is governed by our Code of Conduct. By participating, you are expected to uphold this code. Please report unacceptable behavior to the project maintainers.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the existing issues as you might find that you don't need to create one. When you are creating a bug report, please include as many details as possible:

* **Use a clear and descriptive title**
* **Describe the exact steps to reproduce the problem**
* **Provide specific examples** to demonstrate the steps
* **Describe the behavior you observed** and what behavior you expected to see
* **Include screenshots or animated GIFs** if possible
* **Include your browser and OS version**
* **Include the game version** if applicable

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

* **Use a clear and descriptive title**
* **Provide a detailed description** of the suggested enhancement
* **Provide specific examples** to demonstrate the steps or show similar features
* **Explain why this enhancement would be useful**
* **List some other games or applications** where this enhancement exists

### Pull Requests

Please follow these steps to have your contribution considered by the maintainers:

1. Follow all instructions in the pull request template
2. Follow the style guides
3. After you submit your pull request, verify that all status checks are passing

## Development Process

### Setting Up Your Development Environment

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/aces-high.git`
3. Install dependencies:
   ```bash
   # Install Rust toolchain
   rustup target add wasm32-unknown-unknown
   
   # Install wasm-pack
   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
   
   # Install Node dependencies
   npm install
   ```

4. Create a branch: `git checkout -b feature/my-feature`

### Development Workflow

```bash
# Start development server with hot reload
npm run dev

# Run tests
cargo test                    # Rust unit tests
wasm-pack test --chrome       # WASM tests
npm run test:integration      # Integration tests

# Format code
cargo fmt                     # Rust
npm run format                # JavaScript

# Lint code
cargo clippy                  # Rust
npm run lint                  # JavaScript

# Build for production
npm run build:prod
```

### Making Changes

1. **Write clear, descriptive commit messages**
   ```
   feat: Add twin-engine spread pattern for Lightning aircraft
   
   - Implements dual weapon system
   - Adds synchronized fire timing
   - Updates aircraft stats balance
   ```

2. **Follow Rust style guidelines**
   - Run `cargo fmt` before committing
   - Address all `cargo clippy` warnings
   - Add documentation comments for public APIs
   - Write tests for new functionality

3. **Follow JavaScript style guidelines**
   - Use ESLint configuration
   - Use Prettier for formatting
   - Write JSDoc comments for functions
   - Prefer `const` over `let`, avoid `var`

4. **Write or update tests**
   - Unit tests for pure logic
   - Integration tests for systems interaction
   - WASM tests for browser-specific features
   - Performance tests for hot paths

5. **Update documentation**
   - Update README.md if adding user-facing features
   - Update CHANGELOG.md following Keep a Changelog format
   - Add JSDoc/Rustdoc comments for new APIs
   - Update technical specification if changing architecture

### Rust Style Guide

```rust
// Use descriptive names
pub struct WeaponSystem {
    weapons: HashMap<WeaponId, WeaponDefinition>,
}

// Document public APIs
/// Fires a weapon in the specified direction
///
/// # Arguments
/// * `weapon_id` - The ID of the weapon to fire
/// * `origin` - Starting position of projectiles
/// * `direction` - Direction vector (will be normalized)
///
/// # Returns
/// Vector of spawned projectiles
pub fn fire(&self, weapon_id: WeaponId, origin: Vec2, direction: Vec2) -> Vec<Projectile> {
    // Implementation
}

// Use Result for operations that can fail
pub fn load_texture(&mut self, path: &str) -> Result<TextureHandle, AssetError> {
    // Implementation
}

// Prefer iterators over loops
let total_damage: f32 = projectiles
    .iter()
    .filter(|p| p.is_alive())
    .map(|p| p.damage)
    .sum();
```

### JavaScript Style Guide

```javascript
// Use clear, descriptive names
class AssetLoader {
    /**
     * Load an image asset
     * @param {string} url - URL of the image to load
     * @returns {Promise<Image>} Loaded image element
     */
    async loadImage(url) {
        // Implementation
    }
}

// Use async/await over promises
async function initGame() {
    const wasm = await initWasm();
    const assets = await loadAssets();
    return new GameInstance(wasm, assets);
}

// Use const by default
const MAX_ENTITIES = 500;
const gameState = {
    score: 0,
    level: 1,
};
```

### Testing Guidelines

#### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collision_detection() {
        let pos1 = Position::new(0.0, 0.0);
        let col1 = Collider::Circle { radius: 10.0 };
        
        let pos2 = Position::new(15.0, 0.0);
        let col2 = Collider::Circle { radius: 10.0 };
        
        assert!(CollisionSystem::test_collision(&pos1, &col1, &pos2, &col2));
    }
}
```

#### WASM Tests

```rust
#[cfg(test)]
mod wasm_tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_web_storage() {
        let storage = WebStorage::new();
        storage.save("test_key", "test_value");
        assert_eq!(storage.load("test_key"), Some("test_value"));
    }
}
```

#### Integration Tests

```javascript
// tests/integration/gameplay.test.js
describe('Gameplay Integration', () => {
    test('Player can fire weapons', async () => {
        const game = await initTestGame();
        game.fire_weapon();
        
        const projectiles = game.get_projectiles();
        expect(projectiles.length).toBeGreaterThan(0);
    });
});
```

### Performance Guidelines

1. **Profile Before Optimizing**
   - Use Chrome DevTools Performance tab
   - Use `console.time()` for critical sections
   - Measure frame times with performance monitor

2. **Avoid Allocations in Hot Paths**
   ```rust
   // Bad: Allocates on every call
   fn get_nearby_entities(&self, pos: Vec2) -> Vec<Entity> {
       self.entities.iter()
           .filter(|e| e.distance_to(pos) < 100.0)
           .collect()
   }
   
   // Good: Reuse buffer
   fn get_nearby_entities(&self, pos: Vec2, buffer: &mut Vec<Entity>) {
       buffer.clear();
       buffer.extend(
           self.entities.iter()
               .filter(|e| e.distance_to(pos) < 100.0)
       );
   }
   ```

3. **Use Object Pooling**
   - Pool frequently created/destroyed objects
   - Bullets, particles, effects, etc.
   - See `src/utils/pool.rs` for implementation

4. **Batch Rendering**
   - Group draw calls by texture
   - Use sprite batching
   - Minimize state changes

## Git Workflow

### Branches

- `main` - Production-ready code
- `develop` - Integration branch for features
- `feature/` - Feature branches
- `bugfix/` - Bug fix branches
- `hotfix/` - Urgent production fixes

### Commit Messages

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Formatting, missing semicolons, etc.
- `refactor`: Code change that neither fixes a bug nor adds a feature
- `perf`: Performance improvement
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

Examples:
```
feat(weapons): Add chain lightning special weapon

Implements chain lightning weapon that arcs between nearby enemies.
- Chains up to 5 targets
- Damage decreases by 20% per chain
- Visual arc effect

Closes #123
```

### Pull Request Process

1. **Update your branch** with the latest `develop`:
   ```bash
   git checkout develop
   git pull upstream develop
   git checkout feature/my-feature
   git rebase develop
   ```

2. **Ensure all tests pass**:
   ```bash
   npm test
   cargo test
   wasm-pack test --headless --chrome
   ```

3. **Lint and format code**:
   ```bash
   cargo fmt
   cargo clippy -- -D warnings
   npm run lint
   npm run format
   ```

4. **Update documentation** if needed

5. **Create pull request** with clear description:
   - What changes were made
   - Why the changes were necessary
   - How to test the changes
   - Screenshots/GIFs for UI changes

6. **Respond to code review** feedback

7. **Squash commits** if requested before merging

## Community

- **GitHub Discussions**: For questions and feature discussions
- **GitHub Issues**: For bug reports and feature requests
- **Pull Requests**: For code contributions

## Recognition

Contributors will be recognized in:
- README.md contributors section
- CHANGELOG.md for significant contributions
- In-game credits (for major contributors)

## License

By contributing to ACES HIGH, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to ACES HIGH: ENDLESS SKIES! 🚀

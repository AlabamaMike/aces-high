#!/bin/bash
# build.sh - Production build script for ACES HIGH

set -e

echo "================================================"
echo " ACES HIGH: ENDLESS SKIES - Production Build"
echo "================================================"
echo ""

# Check dependencies
command -v cargo >/dev/null 2>&1 || { echo "Error: cargo is not installed" >&2; exit 1; }
command -v wasm-pack >/dev/null 2>&1 || { echo "Error: wasm-pack is not installed" >&2; exit 1; }
command -v npm >/dev/null 2>&1 || { echo "Error: npm is not installed" >&2; exit 1; }

# Clean previous builds
echo "Cleaning previous builds..."
rm -rf ./dist ./pkg

# Build WASM module
echo ""
echo "Building WASM module (optimized)..."
wasm-pack build \
    --target web \
    --release \
    --no-typescript \
    --out-dir ./pkg \
    --out-name aces_high

# Optimize WASM binary (if wasm-opt is available)
if command -v wasm-opt >/dev/null 2>&1; then
    echo ""
    echo "Optimizing WASM binary with wasm-opt..."
    wasm-opt -Oz -o ./pkg/aces_high_bg_opt.wasm ./pkg/aces_high_bg.wasm
    mv ./pkg/aces_high_bg_opt.wasm ./pkg/aces_high_bg.wasm
    
    # Show size comparison
    echo ""
    echo "WASM binary size:"
    ls -lh ./pkg/aces_high_bg.wasm | awk '{print $5}'
else
    echo ""
    echo "Warning: wasm-opt not found, skipping WASM optimization"
    echo "Install binaryen for better optimization: https://github.com/WebAssembly/binaryen"
fi

# Build web assets
echo ""
echo "Building web assets with webpack..."
npm run build:prod

# Compress assets
echo ""
echo "Compressing assets..."
find ./dist -type f \( -name "*.js" -o -name "*.wasm" -o -name "*.json" -o -name "*.css" \) \
    -exec sh -c 'gzip -9 -k "$1" 2>/dev/null || true' _ {} \;

# Generate asset manifest (if node is available)
if [ -f "./scripts/generate-manifest.js" ]; then
    echo ""
    echo "Generating asset manifest..."
    node ./scripts/generate-manifest.js > ./dist/manifest.json
fi

# Show build statistics
echo ""
echo "================================================"
echo " Build Complete!"
echo "================================================"
echo ""
echo "Build statistics:"
echo "  WASM size: $(ls -lh ./pkg/aces_high_bg.wasm | awk '{print $5}')"
echo "  Dist size: $(du -sh ./dist | awk '{print $1}')"
echo ""
echo "Output directory: ./dist"
echo ""
echo "To test the build locally:"
echo "  npx serve dist"
echo ""
echo "To deploy:"
echo "  Upload ./dist contents to your static hosting provider"
echo ""

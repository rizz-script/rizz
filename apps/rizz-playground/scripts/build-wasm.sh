#!/bin/bash
# Build script for RizzScript WASM module
# Builds the WASM module and copies it to the playground's public folder

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PLAYGROUND_DIR="$(dirname "$SCRIPT_DIR")"
WASM_DIR="$PLAYGROUND_DIR/../rizz-wasm"
PUBLIC_DIR="$PLAYGROUND_DIR/public"
WASM_PUBLIC_DIR="$PUBLIC_DIR/wasm"

echo "🔨 Building RizzScript WASM module..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack is not installed. Installing..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Build WASM module
cd "$WASM_DIR"
echo "📦 Building WASM package..."
wasm-pack build --target web --release

# Create wasm directory in public if it doesn't exist
mkdir -p "$WASM_PUBLIC_DIR"

# Copy WASM files to public folder
echo "📋 Copying WASM files to public folder..."
cp -r pkg/* "$WASM_PUBLIC_DIR/"

# Clean up pkg directory (optional, comment out if you want to keep it)
# rm -rf pkg

echo "✅ WASM build complete!"
echo "📁 Files copied to: $WASM_PUBLIC_DIR"
echo ""
echo "Files:"
ls -lh "$WASM_PUBLIC_DIR" | grep -E "\.(wasm|js|ts|d\.ts)$" || ls -lh "$WASM_PUBLIC_DIR"

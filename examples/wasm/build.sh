#!/bin/bash

# Build WASM package for BrepRs

set -e

echo "Building WASM package..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Error: wasm-pack is not installed."
    echo "Please install it with: cargo install wasm-pack"
    exit 1
fi

# Build the WASM package
wasm-pack build --target web --out-dir examples/wasm/pkg --scope breprs

echo "WASM package built successfully!"
echo "Output directory: examples/wasm/pkg"

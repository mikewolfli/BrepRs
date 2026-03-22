#!/bin/bash

# Build script for BrepRs documentation

set -e

echo "BrepRs Documentation Build Script"
echo "================================="

echo "\n1. Checking if mdbook is installed..."
if ! command -v mdbook &> /dev/null; then
    echo "Error: mdbook is not installed. Please install it first."
    echo "You can install it with: cargo install mdbook"
    exit 1
fi

echo "\n2. Checking if mdbook-i18n is installed..."
if ! command -v mdbook-i18n &> /dev/null; then
    echo "Error: mdbook-i18n is not installed. Please install it first."
    echo "You can install it with: cargo install mdbook-i18n"
    exit 1
fi

echo "\n3. Building documentation..."
cd docs
mdbook build

echo "\n4. Documentation built successfully!"
echo "\nTo serve the documentation locally, run:"
echo "   cd docs && mdbook serve"
echo "\nThen open your browser to http://localhost:3000"

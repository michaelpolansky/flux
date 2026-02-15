#!/bin/bash
echo "🛠️  Fixing builds environment..."

# 1. Ensure absolute latest stable toolchain
echo "📦 Updating Rust toolchain..."
rustup update stable
rustup default stable
rustup target add wasm32-unknown-unknown --toolchain stable

# 2. Check if the target is actually installed for the current directory override
echo "📍 Checking local directory override..."
rustup show

# 3. Nuke everything
echo "💥 Cleaning all compilation artifacts..."
rm -rf target
rm -rf dist
rm -rf node_modules
rm -rf ~/.cache/trunk

# 4. Reinstall deps
echo "📥 Reinstalling NPM dependencies..."
npm install

# 5. Run it
echo "🚀 Starting development server..."
npm run dev

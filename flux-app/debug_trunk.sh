#!/bin/bash
echo "🔍 Debugging Trunk Isolation..."

# 1. Force stable toolchain env var
export RUSTUP_TOOLCHAIN=stable

# 2. Check rustc version in this context
echo "Checking rustc version..."
rustc --version

# 3. Check target location
echo "Checking wasm32-unknown-unknown target..."
rustup show

# 4. Run trunk serve directly (bypassing Tauri)
echo "🚀 Running trunk serve directly..."
trunk serve

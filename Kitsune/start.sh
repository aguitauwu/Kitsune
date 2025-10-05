#!/bin/bash

echo "🦊 Starting Kitsune Discord Security Bot..."

# Start Redis if not running
if ! pgrep redis-server > /dev/null; then
    echo "Starting Redis..."
    redis-server --daemonize yes --port 6379 --dir /tmp
    sleep 2
fi

# Build the project
echo "Building Rust project (this may take a few minutes on first run)..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "🦊 Starting Kitsune..."
    ./target/release/kitsune
else
    echo "❌ Build failed. Trying debug build..."
    cargo build
    if [ $? -eq 0 ]; then
        echo "✅ Debug build successful!"
        echo "🦊 Starting Kitsune (debug mode)..."
        ./target/debug/kitsune
    else
        echo "❌ Build failed. Please check the errors above."
        exit 1
    fi
fi

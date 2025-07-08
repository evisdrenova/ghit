#!/bin/bash
set -e

# Clean previous builds
rm -rf releases/
mkdir -p releases/

# Build for different targets
targets=("x86_64-unknown-linux-gnu" "x86_64-pc-windows-gnu" "x86_64-apple-darwin" "aarch64-apple-darwin")

for target in "${targets[@]}"; do
    echo "Building for $target..."
    
    # Install target if not present
    rustup target add $target || true
    
    # Build
    cargo build --release --target $target
    
    # Package
    if [[ $target == *"windows"* ]]; then
        cp target/$target/release/ghit.exe releases/ghit-$target.exe
        (cd releases && zip ghit-$target.zip ghit-$target.exe)
        rm releases/ghit-$target.exe
    else
        cp target/$target/release/ghit releases/ghit-$target
        (cd releases && tar -czf ghit-$target.tar.gz ghit-$target)
        rm releases/ghit-$target
    fi
    
    echo "✓ Built $target"
done

echo "All binaries built in releases/ directory"
ls -la releases/
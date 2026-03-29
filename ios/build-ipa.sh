#!/bin/bash
# Unlicense — cochranblock.org
# Build oakilydokily iOS static library + Xcode archive
#
# Prerequisites:
#   rustup target add aarch64-apple-ios
#   Xcode 15+ with iOS SDK
#
# Usage: ./ios/build-ipa.sh

set -euo pipefail
cd "$(dirname "$0")/.."

echo "=== oakilydokily iOS build ==="

TARGET=aarch64-apple-ios
LIB_NAME=liboakilydokily.a

echo "--- Step 1: Build Rust static library for $TARGET ---"
cargo build --release --lib --target $TARGET 2>&1

RUST_LIB="target/$TARGET/release/$LIB_NAME"
if [ ! -f "$RUST_LIB" ]; then
  echo "ERROR: $RUST_LIB not found"
  exit 1
fi
echo "Static lib: $(ls -lh "$RUST_LIB" | awk '{print $5}') ($RUST_LIB)"

echo "--- Step 2: Xcode build (requires .xcodeproj — generate interactively) ---"
echo ""
echo "To complete the iOS build:"
echo "1. Open Xcode → New Project → App → OakilyDokily"
echo "2. Set bundle ID: org.cochranblock.oakilydokily"
echo "3. Replace AppDelegate.swift with ios/OakilyDokily/AppDelegate.swift"
echo "4. Add $RUST_LIB as a linked library in Build Phases"
echo "5. Add -lresolv -lsqlite3 to Other Linker Flags"
echo "6. Build → Archive → Export IPA"
echo ""
echo "Static lib ready at: $RUST_LIB"

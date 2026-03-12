#!/bin/bash
# Build mural-wasm and copy WASM + assets for standalone serving.
set -e
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
OAKILY="$(cd "$SCRIPT_DIR/.." && pwd)"
# Build from workspace root so target/ resolves correctly
WS_ROOT="$(cd "$OAKILY/.." && pwd)"
cd "$WS_ROOT"
cargo build --target wasm32-unknown-unknown -p mural-wasm --release
cp "$WS_ROOT/target/wasm32-unknown-unknown/release/mural-wasm.wasm" "$OAKILY/mural-wasm/"
cp "$OAKILY/assets/mural.png" "$OAKILY/assets/1000003453.png" "$OAKILY/mural-wasm/assets/"
echo "Built. Run ./serve.sh from mural-wasm/ to serve."

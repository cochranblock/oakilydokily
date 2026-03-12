#!/bin/bash
# Build mural-wasm and copy WASM + claymation assets for standalone serving.
set -e
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
OAKILY="$(cd "$SCRIPT_DIR/.." && pwd)"
# Build from workspace root so target/ resolves correctly
WS_ROOT="$(cd "$OAKILY/.." && pwd)"
cd "$WS_ROOT"

# Generate claymation sprites from mural animals
MURAL="$OAKILY/assets/mural.png"
CLAY_OUT="$OAKILY/mural-wasm/assets/claymation_out"
if [ -f "$MURAL" ]; then
  cargo run -p mural-claymation --release -- "$MURAL" -o "$CLAY_OUT" 2>/dev/null || true
  if [ -f "$CLAY_OUT/claymation_spritesheet.png" ]; then
    cp "$CLAY_OUT/claymation_spritesheet.png" "$CLAY_OUT/claymation_meta.json" "$OAKILY/mural-wasm/assets/"
  fi
fi

cargo build --target wasm32-unknown-unknown -p mural-wasm --release
cp "$WS_ROOT/target/wasm32-unknown-unknown/release/mural-wasm.wasm" "$OAKILY/mural-wasm/"
cp "$OAKILY/assets/mural.png" "$OAKILY/mural-wasm/assets/" 2>/dev/null || true
echo "Built. Run ./serve.sh from mural-wasm/ to serve."

#!/bin/bash
# Build mural-wasm and copy WASM + assets. Prefer claymation (Python crop-then-rembg) when available.
set -e
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
OAKILY="$(cd "$SCRIPT_DIR/.." && pwd)"
WS_ROOT="$(cd "$OAKILY/.." && pwd)"
cd "$WS_ROOT"

mkdir -p "$OAKILY/mural-wasm/assets"
MURAL="$OAKILY/assets/mural.png"
CLAY_OUT="$OAKILY/mural-wasm/assets/claymation_out"

# Fetch CC0 pixel art (CatnDog + guinea pig) as fallback when claymation unavailable
if [ ! -f "$OAKILY/mural-wasm/assets/pets_spritesheet.png" ]; then
  (cd "$SCRIPT_DIR/scripts" && python3 fetch_pixel_art.py 2>/dev/null) || true
fi

# Background with animals inpainted (no original animals under pet sprites)
if [ -f "$MURAL" ]; then
  (cd "$SCRIPT_DIR/scripts" && python3 background_only.py 2>/dev/null) || true
fi

if [ -f "$MURAL" ] && python3 -c "import rembg" 2>/dev/null; then
  echo "Running claymation pipeline (crop-then-rembg)..."
  (cd "$SCRIPT_DIR/scripts" && python3 claymation_pipeline.py ../../assets/mural.png -o ../assets/claymation_out --pixel-scale 1.0 2>/dev/null) || true
  if [ -f "$CLAY_OUT/claymation_spritesheet.png" ]; then
    cp "$CLAY_OUT/claymation_spritesheet.png" "$CLAY_OUT/claymation_meta.json" "$OAKILY/mural-wasm/assets/"
    [ -f "$CLAY_OUT/background_filled.png" ] && cp "$CLAY_OUT/background_filled.png" "$OAKILY/mural-wasm/assets/background.png"
    echo "Using claymation animals."
  fi
fi

cargo build --target wasm32-unknown-unknown -p mural-wasm --release
cp "$WS_ROOT/target/wasm32-unknown-unknown/release/mural-wasm.wasm" "$OAKILY/mural-wasm/"
cp "$OAKILY/assets/mural.png" "$OAKILY/mural-wasm/assets/" 2>/dev/null || true
echo "Built. Run ./serve.sh from mural-wasm/ to serve."

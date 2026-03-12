<p align="center">
  <img src="https://raw.githubusercontent.com/cochranblock/oakilydokily/main/assets/favicon.svg" alt="mural-wasm" width="48">
</p>

# OakilyDokily Interactive Mural (Rust + Macroquad, WASM)

High-performance 2D interactive mural targeting `wasm32-unknown-unknown` for web.

## Proof of Artifacts

*Wire diagrams for quick review.*

### Wire / Architecture

```mermaid
flowchart LR
    JS[JS / HTML] --> Bridge[mural-bridge.js]
    Bridge --> WASM[mural-wasm.wasm]
    WASM --> Macroquad[Macroquad render]
    Macroquad --> Pets[Pets: Cat Dog GuineaPig]
    Macroquad --> Scenes[Scenes: Cozy Nook Tubing Doggy Door]
```

### Demo

*Add `docs/artifacts/demo-mural.gif` for pets wandering, interaction, guinea pig kiss.*

---

## Build

```bash
cargo build --target wasm32-unknown-unknown -p mural-wasm --release
```

Output: `target/wasm32-unknown-unknown/release/mural-wasm.wasm`

## Standalone demo

From `oakilydokily/`:

```bash
./mural-wasm/build-standalone.sh   # build + copy WASM and assets
cd mural-wasm && ./serve.sh       # serve at http://127.0.0.1:8765
```

Open http://127.0.0.1:8765/index.html. Assets at `/assets/*` resolve from `mural-wasm/assets/`.

## Features

- **SpriteSheet**: Algorithmic grid slicing (4×3 default)
- **TextureAtlas**: Cats (row 0), Dogs (row 1), Guinea Pigs (row 2); Walk, Interaction, Sleeping, Kiss
- **Pet entities**: Wandering, Sleeping, Interacting states
- **Proximity detection**: Same species within 30px → Interaction
- **Guinea Pig kiss**: Heart particles via Macroquad
- **Scroll-triggered scenes**: Cozy Nook, Winter Tubing, Doggy Door (footer)
- **JS bridge**: `mural_set_scroll_y(y)`, `mural_set_mouse(x, y)` — call from JS to pass scroll/mouse
- **Occlusion culling**: Only pets in viewport are updated
- **FilterMode::Nearest**: Crisp 8-bit pixels (no linear filtering)

## Asset

Place sprite sheet at `assets/1000003453.png`. Grid: 4 cols × 3 rows. Falls back to white placeholder if missing.

## Claymation pipeline (original mural animals)

Extract animals from the mural, inpaint background, pixelate, rotate, composite. **Pure Rust:**

```bash
cargo run -p mural-claymation -- -o out_rust
```

Or Python: `cd mural-wasm/scripts && pip install -r requirements.txt && python claymation_pipeline.py -o out_claymation`

Outputs: `background_filled.png`, `animal_XX_rotYY.png`, `frame_sample.png`, `claymation_spritesheet.png`.

## Integration

Embed in oakilydokily hero:

```html
<canvas id="glcanvas"></canvas>
<script src="/assets/gl.js"></script>
<script src="/assets/mural-bridge.js"></script>
<script>load("/assets/mural-wasm.wasm");</script>
```

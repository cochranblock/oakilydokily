# Mural Scripts

## fetch_pixel_art.py

Fetches CC0 pixel art (CatnDog pzUH) and builds `pets_spritesheet.png` (4×3: Cat walk, Dog walk, Guinea pig). Guinea pig is an embedded minimal sprite. Run from `mural-wasm/scripts/` or via `build-standalone.sh`.

## background_only.py

Creates `background.png` (mural with animals inpainted out). Targeted mask, INPAINT_NS. **Not used by default** — inpaint often leaves visible artifacts. Run manually if needed.

## claymation_pipeline.py

Extracts animals from the mural using **Pic2Pix-style edge sampling**: color-based candidate regions → crop each → sample edge strips → mask out background colors → clean cutout. No ML deps for primary path; rembg optional fallback.

## Python

```bash
pip install -r requirements.txt
python claymation_pipeline.py ../../assets/mural.png -o ../assets/claymation_out --pixel-scale 1.0
```

Mural path defaults to `../../assets/mural.png`. `rembg` optional for fallback when edge sampling fails.

## Outputs

- `claymation_spritesheet.png` — Grid for mural-wasm (cols=rotations, rows=animals)
- `claymation_meta.json` — Sheet layout
- `animal_XX_raw.png` — Cutouts (edge-sampling or rembg-refined)
- `animal_XX_rotYY.png` — Rotated poses
- `frame_sample.png` — Sample composite

## Pipeline

1. **Color mask** — Exclude grass (green), water (blue), soil (red-brown)
2. **Connected components** — Filter by area (200–12000), aspect (<2.2), exclude mostly-green (palm fronds, thresh 0.22)
3. **Edge sampling** — For each component, crop with padding. Sample top/bottom/left/right strips; mask pixels matching edge colors (tolerance 40). Pic2Pix-style: object centered, edges = background.
4. **Post-filter** — Reject cutouts with >20% green (foliage)
5. **Pixelate, rotate, composite** — Same as before

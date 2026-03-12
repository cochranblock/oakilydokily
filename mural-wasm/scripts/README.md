# Mural Claymation Pipeline

Extracts animals from the mural using **crop-then-rembg**: color-based candidate regions → crop each → rembg on crop (object is main subject) → clean cutout.

## Python (recommended, requires rembg)

```bash
pip install -r requirements.txt
python claymation_pipeline.py ../../assets/mural.png -o ../assets/claymation_out --pixel-scale 1.0
```

Mural path defaults to `../../assets/mural.png`.

## Outputs

- `claymation_spritesheet.png` — Grid for mural-wasm (cols=rotations, rows=animals)
- `claymation_meta.json` — Sheet layout
- `animal_XX_raw.png` — Cutouts (rembg-refined when available)
- `animal_XX_rotYY.png` — Rotated poses
- `frame_sample.png` — Sample composite

## Pipeline

1. **Color mask** — Exclude grass (green), water (blue), soil (red-brown)
2. **Connected components** — Filter by area (200–12000), aspect (<2.2), exclude mostly-green (palm fronds)
3. **Crop-then-rembg** — For each component, crop with padding, run rembg on crop. rembg segments cleanly when object is isolated.
4. **Pixelate, rotate, composite** — Same as before

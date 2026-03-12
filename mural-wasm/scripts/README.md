# Mural Claymation Pipeline

Extracts animals from the mural, inpaints the background, pixelates cutouts, generates rotated poses, and composites claymation-style frames.

## Setup

```bash
pip install -r requirements.txt
```

## Run

```bash
cd mural-wasm/scripts
python claymation_pipeline.py -o out_claymation
```

Mural path defaults to `../../assets/mural.png` (oakilydokily assets).

## Outputs

- `mask_full.png` — Foreground mask
- `background_filled.png` — Inpainted background (animals removed)
- `animal_XX_raw.png` — Raw cutouts
- `animal_XX_rotYY.png` — 8-bit pixelated, rotated poses (4 angles by default)
- `frame_sample.png` — Sample composite frame

## Options

- `--rotations N` — Number of rotation angles (default 4)
- `--pixel-scale F` — Pixelation scale 0–1 (default 0.25)
- `--min-area N` — Minimum animal blob area (default 150)

## Pipeline

1. **Segment** — Color-based extraction (non-green, non-blue) to find animals
2. **Inpaint** — Fill background where animals were removed
3. **Pixelate** — Scale down/up with NEAREST + palette reduction
4. **Rotate** — Generate N static poses per animal
5. **Composite** — Layer cutouts onto filled background

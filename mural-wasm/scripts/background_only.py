#!/usr/bin/env python3
# Unlicense — cochranblock.org
# Create mural background with animals inpainted out. No rembg. Uses color mask + inpaint.

import sys
from pathlib import Path

import numpy as np
from PIL import Image

try:
    import cv2
except ImportError:
    print("Install opencv: pip install opencv-python", file=sys.stderr)
    sys.exit(1)


def _segment_animals_by_color(rgba: np.ndarray) -> np.ndarray:
    """Animal-like regions: not grass, water, soil."""
    rgb = rgba[:, :, :3]
    hsv = cv2.cvtColor(rgb, cv2.COLOR_RGB2HSV)
    h, s, v = hsv[:, :, 0], hsv[:, :, 1], hsv[:, :, 2]
    green = (h >= 15) & (h <= 45) & (s > 35)
    blue = (h >= 42) & (h <= 70)
    soil = ((h <= 15) | (h >= 168)) & (s > 40) & (v < 150)
    background = green | blue | soil | (s < 22)
    animal_candidates = ~background & (v > 25)
    mask = animal_candidates.astype(np.uint8) * 255
    kernel = np.ones((3, 3), np.uint8)
    mask = cv2.morphologyEx(mask, cv2.MORPH_OPEN, kernel)
    mask = cv2.morphologyEx(mask, cv2.MORPH_CLOSE, kernel)
    return mask


def run(mural_path: Path, out_path: Path) -> None:
    pil = Image.open(mural_path).convert("RGBA")
    rgba = np.array(pil)
    mask = _segment_animals_by_color(rgba)
    rgb = rgba[:, :, :3].copy()
    inpainted = cv2.inpaint(rgb, mask, inpaintRadius=5, flags=cv2.INPAINT_TELEA)
    result = rgba.copy()
    result[:, :, :3] = inpainted
    Image.fromarray(result).save(out_path)
    print(f"Wrote {out_path}")


def main() -> None:
    script_dir = Path(__file__).resolve().parent
    mural = script_dir.parent.parent / "assets" / "mural.png"
    out = script_dir.parent / "assets" / "background.png"
    if not mural.exists():
        mural = Path("mural.png").resolve()
    if not mural.exists():
        print(f"Mural not found: {mural}", file=sys.stderr)
        sys.exit(1)
    out.parent.mkdir(parents=True, exist_ok=True)
    run(mural, out)


if __name__ == "__main__":
    main()

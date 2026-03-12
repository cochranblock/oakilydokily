#!/usr/bin/env python3
# Unlicense — cochranblock.org
# Create mural background with animals inpainted out. Targeted mask (animal blobs only).

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


def _is_mostly_green(rgba: np.ndarray, comp_mask: np.ndarray, thresh: float = 0.15) -> bool:
    hsv = cv2.cvtColor(rgba[:, :, :3], cv2.COLOR_RGB2HSV)
    green = (hsv[:, :, 0] >= 17) & (hsv[:, :, 0] <= 42) & (hsv[:, :, 1] > 40)
    masked = comp_mask > 0
    n = np.count_nonzero(masked)
    return n > 0 and np.count_nonzero(masked & green) / n >= thresh


def _has_fur(rgba: np.ndarray, comp_mask: np.ndarray, min_frac: float = 0.08) -> bool:
    hsv = cv2.cvtColor(rgba[:, :, :3], cv2.COLOR_RGB2HSV)
    h, s, v = hsv[:, :, 0], hsv[:, :, 1], hsv[:, :, 2]
    fur = (
        ((h >= 8) & (h <= 38) & (s >= 25) & (v >= 70))
        | ((s <= 45) & (v >= 170))
        | ((h <= 15) | (h >= 160)) & (s >= 12) & (v >= 100)
        | ((s <= 60) & (v >= 35) & (v <= 190))
        | ((s <= 90) & (v <= 55))
    )
    masked = comp_mask > 0
    n = np.count_nonzero(masked)
    return n > 0 and np.count_nonzero(masked & fur) / n >= min_frac


def _animal_mask_only(rgba: np.ndarray) -> np.ndarray:
    """Mask only identified animal blobs (not palm trees, rocks, etc)."""
    raw = _segment_animals_by_color(rgba)
    num_labels, labels, stats, _ = cv2.connectedComponentsWithStats(raw, connectivity=8)
    out = np.zeros_like(raw)
    for i in range(1, num_labels):
        area = stats[i, cv2.CC_STAT_AREA]
        if area < 150 or area > 10000:
            continue
        comp = (labels == i).astype(np.uint8) * 255
        if _is_mostly_green(rgba, comp):
            continue
        if not _has_fur(rgba, comp):
            continue
        out = np.maximum(out, comp)
    kernel = np.ones((5, 5), np.uint8)
    out = cv2.dilate(out, kernel)
    return out


def run(mural_path: Path, out_path: Path) -> None:
    pil = Image.open(mural_path).convert("RGBA")
    rgba = np.array(pil)
    mask = _animal_mask_only(rgba)
    if np.count_nonzero(mask) < 100:
        Image.fromarray(rgba).save(out_path)
        print("No animal regions found, wrote original mural")
        return
    rgb = rgba[:, :, :3].copy()
    inpainted = cv2.inpaint(rgb, mask, inpaintRadius=8, flags=cv2.INPAINT_NS)
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

#!/usr/bin/env python3
# Unlicense — cochranblock.org
# Mural claymation pipeline: segment animals from mural, inpaint background,
# pixelate cutouts, generate rotated poses, composite frames.

import argparse
import sys
from pathlib import Path

import numpy as np
from PIL import Image

# Optional deps
try:
    from rembg import remove
    HAS_REMBG = True
except ImportError:
    HAS_REMBG = False

try:
    import cv2
    HAS_CV2 = True
except ImportError:
    HAS_CV2 = False


def ensure_deps():
    if not HAS_CV2:
        print("Install opencv: pip install opencv-python", file=sys.stderr)
        sys.exit(1)
    if not HAS_REMBG:
        print("Note: rembg not installed; using GrabCut (pip install rembg for better segmentation)")


def load_image(path: Path) -> tuple[np.ndarray, Image.Image]:
    """Load image as numpy (BGR for cv2) and PIL."""
    pil = Image.open(path).convert("RGBA")
    arr = np.array(pil)
    bgr = cv2.cvtColor(arr[:, :, :3], cv2.COLOR_RGBA2BGRA)
    return arr, pil


def segment_foreground_rembg(pil_img: Image.Image) -> np.ndarray:
    """Use rembg to get foreground mask (alpha). Returns binary mask 0-255."""
    out = remove(pil_img)
    alpha = np.array(out.split()[-1])
    mask = (alpha > 128).astype(np.uint8) * 255
    return mask


def segment_foreground_grabcut(rgba: np.ndarray) -> np.ndarray:
    """Fallback: OpenCV GrabCut for foreground segmentation."""
    rgb = cv2.cvtColor(rgba[:, :, :3], cv2.COLOR_RGB2BGR)
    h, w = rgb.shape[:2]
    mask = np.zeros((h, w), np.uint8)
    bgd = np.zeros((1, 65), np.float64)
    fgd = np.zeros((1, 65), np.float64)
    rect = (10, 10, w - 10, h - 10)
    cv2.grabCut(rgb, mask, rect, bgd, fgd, 5, cv2.GC_INIT_WITH_RECT)
    return np.where((mask == 2) | (mask == 0), 0, 255).astype(np.uint8)


def segment_foreground(pil_img: Image.Image, rgba: np.ndarray) -> np.ndarray:
    """Segment foreground. Try rembg first; if one huge blob, refine with color-based animal mask."""
    if HAS_REMBG:
        try:
            mask = segment_foreground_rembg(pil_img)
            # If rembg returns whole scene, try color-based animal extraction
            if np.count_nonzero(mask) > mask.size * 0.5:
                animal_mask = _segment_animals_by_color(rgba)
                if np.count_nonzero(animal_mask) > 500:
                    return animal_mask
            return mask
        except Exception as e:
            print(f"rembg failed ({e}), using GrabCut fallback")
    return segment_foreground_grabcut(rgba)


def _segment_animals_by_color(rgba: np.ndarray) -> np.ndarray:
    """Extract animal-like regions: not grass (green), not water (blue). OpenCV H in 0-180."""
    rgb = rgba[:, :, :3]
    hsv = cv2.cvtColor(rgb, cv2.COLOR_RGB2HSV)
    h, s, v = hsv[:, :, 0], hsv[:, :, 1], hsv[:, :, 2]
    # Grass: H 35-85 (OpenCV half) -> 17-42
    green = (h >= 17) & (h <= 42) & (s > 40)
    # Water: H 90-130 -> 45-65
    blue = (h >= 45) & (h <= 65)
    # Very low sat = grey/white
    background = green | blue | (s < 25)
    animal_candidates = ~background & (v > 30)  # Not dark
    mask = animal_candidates.astype(np.uint8) * 255
    kernel = np.ones((3, 3), np.uint8)
    mask = cv2.morphologyEx(mask, cv2.MORPH_OPEN, kernel)
    mask = cv2.morphologyEx(mask, cv2.MORPH_CLOSE, kernel)
    return mask


def connected_components(mask: np.ndarray, min_area: int = 200) -> list[tuple[int, np.ndarray]]:
    """Find connected components. If one huge blob, try watershed to split."""
    num_labels, labels, stats, _ = cv2.connectedComponentsWithStats(mask, connectivity=8)
    components = []
    for i in range(1, num_labels):
        area = stats[i, cv2.CC_STAT_AREA]
        if area < min_area:
            continue
        comp_mask = (labels == i).astype(np.uint8) * 255
        # If single huge component (whole scene), try watershed to get individual animals
        if area > mask.size * 0.2 and num_labels == 2:
            split = _watershed_split(comp_mask, min_area)
            return split
        components.append((i, comp_mask))
    return components if components else [(1, mask)]


def _watershed_split(mask: np.ndarray, min_area: int) -> list[tuple[int, np.ndarray]]:
    """Use distance transform + watershed to split one blob into many."""
    dist = cv2.distanceTransform(mask, cv2.DIST_L2, 5)
    dist_uint8 = np.uint8(np.clip(dist / (dist.max() or 1) * 255, 0, 255))
    # Lower threshold = more markers = more splits
    _, sure_fg = cv2.threshold(dist_uint8, 0.15 * (dist_uint8.max() or 1), 255, 0)
    sure_fg = np.uint8(sure_fg)
    num_labels, labels, stats, _ = cv2.connectedComponentsWithStats(sure_fg, connectivity=8)
    if num_labels < 3:
        return [(1, mask)]
    out = []
    for i in range(1, num_labels):
        comp = (labels == i).astype(np.uint8) * 255
        kernel = np.ones((7, 7), np.uint8)
        comp = cv2.dilate(comp, kernel)
        comp = cv2.bitwise_and(comp, mask)
        area = np.count_nonzero(comp)
        if min_area <= area <= mask.size * 0.15:  # Skip tiny and huge
            out.append((i, comp))
    return out if out else [(1, mask)]


def extract_cutout(rgba: np.ndarray, mask: np.ndarray, padding: int = 2) -> Image.Image:
    """Extract RGBA cutout using mask, with padding."""
    ys, xs = np.where(mask > 0)
    if len(xs) == 0:
        return None
    x1, x2 = xs.min() - padding, xs.max() + padding + 1
    y1, y2 = ys.min() - padding, ys.max() + padding + 1
    x1, y1 = max(0, x1), max(0, y1)
    x2, y2 = min(rgba.shape[1], x2), min(rgba.shape[0], y2)
    crop = rgba[y1:y2, x1:x2].copy()
    crop_mask = mask[y1:y2, x1:x2]
    crop[:, :, 3] = np.where(crop_mask > 0, rgba[y1:y2, x1:x2, 3], 0)
    return Image.fromarray(crop, "RGBA")


def inpaint_background(rgba: np.ndarray, mask: np.ndarray) -> np.ndarray:
    """Inpaint regions where mask=255 using surrounding pixels."""
    rgb = rgba[:, :, :3].copy()
    mask_inpaint = mask  # cv2.inpaint expects 8-bit mask
    inpainted = cv2.inpaint(rgb, mask_inpaint, inpaintRadius=3, flags=cv2.INPAINT_TELEA)
    result = rgba.copy()
    result[:, :, :3] = inpainted
    return result


def pixelate_8bit(pil_img: Image.Image, scale: float = 0.25) -> Image.Image:
    """Reduce to 8-bit style: scale down then up with NEAREST."""
    w, h = pil_img.size
    small_w = max(4, int(w * scale))
    small_h = max(4, int(h * scale))
    small = pil_img.resize((small_w, small_h), Image.Resampling.NEAREST)
    return small.resize((w, h), Image.Resampling.NEAREST)


def reduce_palette(pil_img: Image.Image, colors: int = 16) -> Image.Image:
    """Reduce color palette for 8-bit look."""
    return pil_img.quantize(colors=colors, method=Image.Quantize.FASTOCTREE).convert("RGBA")


def rotate_cutout(pil_img: Image.Image, angle: float) -> Image.Image:
    """Rotate RGBA image, preserving transparency."""
    return pil_img.rotate(angle, resample=Image.Resampling.NEAREST, expand=True)


def composite_frame(
    background: np.ndarray,
    cutouts: list[tuple[int, int, Image.Image]],
) -> Image.Image:
    """Composite cutouts onto background at (x,y) positions."""
    result = Image.fromarray(background).convert("RGBA")
    for x, y, cutout in cutouts:
        result.paste(cutout, (x, y), cutout)
    return result


def run_pipeline(
    mural_path: Path,
    out_dir: Path,
    num_rotations: int = 4,
    pixel_scale: float = 0.25,
    min_animal_area: int = 150,
) -> None:
    ensure_deps()
    out_dir.mkdir(parents=True, exist_ok=True)

    rgba, pil = load_image(mural_path)
    h, w = rgba.shape[:2]

    print("Segmenting foreground...")
    mask = segment_foreground(pil, rgba)
    cv2.imwrite(str(out_dir / "mask_full.png"), mask)

    print("Finding animal components...")
    components = connected_components(mask, min_area=min_animal_area)
    print(f"Found {len(components)} animal regions")

    # Extract cutouts
    cutouts = []
    for i, (_, comp_mask) in enumerate(components):
        cutout = extract_cutout(rgba, comp_mask)
        if cutout is None:
            continue
        cutout_path = out_dir / f"animal_{i:02d}_raw.png"
        cutout.save(cutout_path)
        cutouts.append((comp_mask, cutout))

    # Inpaint background
    print("Inpainting background...")
    filled_bg = inpaint_background(rgba, mask)
    bg_pil = Image.fromarray(filled_bg, "RGBA")
    bg_pil.save(out_dir / "background_filled.png")

    # Pixelate and rotate each cutout, save sprite sheet
    print("Generating 8-bit rotated sprites...")
    for i, (comp_mask, cutout) in enumerate(cutouts):
        pixelated = pixelate_8bit(cutout, scale=pixel_scale)
        pixelated = reduce_palette(pixelated, colors=16)
        angles = [360 * j / num_rotations for j in range(num_rotations)]
        for j, angle in enumerate(angles):
            rotated = rotate_cutout(pixelated, angle)
            rotated.save(out_dir / f"animal_{i:02d}_rot{j:02d}.png")

    # Composite sample frame: place pixelated animals back at original positions
    print("Compositing sample frame...")
    ys, xs = np.where(mask > 0)
    # Use centroid of each component for placement
    placements = []
    for i, (_, comp_mask) in enumerate(components):
        ys_c, xs_c = np.where(comp_mask > 0)
        cx, cy = int(xs_c.mean()), int(ys_c.mean())
        # Load pixelated version (angle 0)
        sprite = Image.open(out_dir / f"animal_{i:02d}_rot00.png")
        sw, sh = sprite.size
        placements.append((cx - sw // 2, cy - sh // 2, sprite))
    frame = composite_frame(filled_bg, placements)
    frame.save(out_dir / "frame_sample.png")

    # Build sprite sheet: pack animal_XX_rotYY into grid for mural-wasm
    _build_sprite_sheet(out_dir, len(cutouts), num_rotations)

    print(f"Done. Outputs in {out_dir}")


def _build_sprite_sheet(out_dir: Path, num_animals: int, num_rots: int, max_cell: int = 128) -> None:
    """Pack rotated sprites into a single sprite sheet (cols=num_rots, rows=num_animals)."""
    sprites = []
    max_w, max_h = 0, 0
    for i in range(num_animals):
        row = []
        for j in range(num_rots):
            p = out_dir / f"animal_{i:02d}_rot{j:02d}.png"
            if p.exists():
                img = Image.open(p).convert("RGBA")
                if img.width > max_cell or img.height > max_cell:
                    r = min(max_cell / img.width, max_cell / img.height)
                    new_w, new_h = int(img.width * r), int(img.height * r)
                    img = img.resize((new_w, new_h), Image.Resampling.NEAREST)
                row.append(img)
                max_w = max(max_w, img.width)
                max_h = max(max_h, img.height)
        if row:
            sprites.append(row)
    if not sprites:
        return
    cols = num_rots
    rows = len(sprites)
    sheet_w = cols * max_w
    sheet_h = rows * max_h
    sheet = Image.new("RGBA", (sheet_w, sheet_h), (0, 0, 0, 0))
    for r, row in enumerate(sprites):
        for c, img in enumerate(row):
            sheet.paste(img, (c * max_w, r * max_h), img)
    sheet.save(out_dir / "claymation_spritesheet.png")
    print(f"Sprite sheet: {cols}×{rows} cells, {max_w}×{max_h} each")


def main():
    parser = argparse.ArgumentParser(description="Mural claymation asset pipeline")
    parser.add_argument("mural", type=Path, nargs="?", default=Path("../../assets/mural.png"))
    parser.add_argument("-o", "--out", type=Path, default=Path("out_claymation"))
    parser.add_argument("--rotations", type=int, default=4)
    parser.add_argument("--pixel-scale", type=float, default=0.25)
    parser.add_argument("--min-area", type=int, default=150)
    args = parser.parse_args()

    script_dir = Path(__file__).resolve().parent
    mural = (script_dir / args.mural).resolve()
    for fallback in [script_dir.parent / "assets" / "mural.png", script_dir.parent.parent / "assets" / "mural.png"]:
        if not mural.exists() and fallback.exists():
            mural = fallback
            break
    if not mural.exists():
        mural = Path("mural.png").resolve()
    if not mural.exists():
        print(f"Mural not found: {mural}", file=sys.stderr)
        sys.exit(1)

    out = script_dir / args.out
    run_pipeline(
        mural_path=mural,
        out_dir=out,
        num_rotations=args.rotations,
        pixel_scale=args.pixel_scale,
        min_animal_area=args.min_area,
    )


if __name__ == "__main__":
    main()

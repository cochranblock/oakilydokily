#!/usr/bin/env python3
# Unlicense — cochranblock.org
# Mural claymation pipeline: segment animals from mural, inpaint background,
# pixelate cutouts, generate rotated poses, composite frames.
# Use --regions animal_regions.json for manual bboxes (rembg-only, no auto segmentation).

import argparse
import json
import sys
from pathlib import Path
from typing import Optional

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
        print("Note: rembg not installed; using corner-sampling + color-mask (pip install rembg for fallback)")


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


def refine_crop_with_corner_sampling(
    pil_img: Image.Image,
    crop_bbox: tuple,
    tolerance: int = 40,
    edge_strip: int = 8,
) -> Optional[Image.Image]:
    """Pic2Pix-style: sample edge strips of crop, mask out pixels matching edge colors.
    Edge sampling (full top/bottom/left/right strips) is more robust than corners alone.
    Works when object is centered and crop edges contain background (grass, soil, etc)."""
    x1, y1, x2, y2 = crop_bbox
    crop = pil_img.crop((x1, y1, x2, y2))
    cw, ch = crop.size
    if cw < 16 or ch < 16:
        return None
    arr = np.array(crop)
    if arr.ndim != 3 or arr.shape[2] < 3:
        return None
    strip = min(edge_strip, cw // 4, ch // 4, max(3, cw // 8), max(3, ch // 8))
    if strip < 2:
        return None
    # Sample 4 edge strips: top, bottom, left, right
    samples = [
        arr[0:strip, :],  # top
        arr[ch - strip : ch, :],  # bottom
        arr[:, 0:strip],  # left
        arr[:, cw - strip : cw],  # right
    ]
    comp = [np.mean(s[:, :, :3], axis=(0, 1)) for s in samples]
    low = [np.maximum(c - tolerance, 0).astype(np.int32) for c in comp]
    high = [np.minimum(c + tolerance, 255).astype(np.int32) for c in comp]
    rgb = arr[:, :, :3].astype(np.int32)
    alpha = arr[:, :, 3].copy()
    for i in range(4):
        lo, hi = low[i], high[i]
        match = (
            (rgb[:, :, 0] >= lo[0]) & (rgb[:, :, 0] <= hi[0])
            & (rgb[:, :, 1] >= lo[1]) & (rgb[:, :, 1] <= hi[1])
            & (rgb[:, :, 2] >= lo[2]) & (rgb[:, :, 2] <= hi[2])
        )
        alpha[match] = 0
    arr[:, :, 3] = alpha
    out = Image.fromarray(arr)
    fg_pixels = np.count_nonzero(alpha > 128)
    if fg_pixels < 50:
        return None
    return out


def refine_crop_with_rembg(pil_img: Image.Image, crop_bbox: tuple) -> Optional[Image.Image]:
    """Crop image to bbox, run rembg on crop. Returns RGBA with clean foreground. rembg works best when object is main subject."""
    x1, y1, x2, y2 = crop_bbox
    crop = pil_img.crop((x1, y1, x2, y2))
    if crop.size[0] < 8 or crop.size[1] < 8:
        return None
    try:
        out = remove(crop)
        return out
    except Exception:
        return None


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
    """Extract animal-like regions: not grass, water, soil, trunk. OpenCV H in 0-180."""
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


def _is_mostly_green(rgba: np.ndarray, comp_mask: np.ndarray, thresh: float = 0.12) -> bool:
    """True if >thresh of masked pixels are green (palm fronds, grass)."""
    rgb = rgba[:, :, :3]
    hsv = cv2.cvtColor(rgb, cv2.COLOR_RGB2HSV)
    h, s = hsv[:, :, 0], hsv[:, :, 1]
    green = (h >= 17) & (h <= 42) & (s > 40)
    masked = comp_mask > 0
    n_masked = np.count_nonzero(masked)
    if n_masked == 0:
        return False
    n_green = np.count_nonzero(masked & green)
    return n_green / n_masked >= thresh


def _is_mostly_brown_trunk(rgba: np.ndarray, comp_mask: np.ndarray, thresh: float = 0.38) -> bool:
    """True if >thresh of masked pixels are trunk/soil brown (H 18-35, S 45-90, V 55-140)."""
    rgb = rgba[:, :, :3]
    hsv = cv2.cvtColor(rgb, cv2.COLOR_RGB2HSV)
    h, s, v = hsv[:, :, 0], hsv[:, :, 1], hsv[:, :, 2]
    trunk = (h >= 18) & (h <= 35) & (s >= 45) & (s <= 90) & (v >= 55) & (v <= 140)
    masked = comp_mask > 0
    n_masked = np.count_nonzero(masked)
    if n_masked == 0:
        return False
    return np.count_nonzero(masked & trunk) / n_masked >= thresh


def _has_fur_colors(rgba: np.ndarray, comp_mask: np.ndarray, min_frac: float = 0.08) -> bool:
    """True if >=min_frac of masked pixels are animal fur colors (orange, tan, white, pink, grey, black)."""
    rgb = rgba[:, :, :3]
    hsv = cv2.cvtColor(rgb, cv2.COLOR_RGB2HSV)
    h, s, v = hsv[:, :, 0], hsv[:, :, 1], hsv[:, :, 2]
    orange_tan = (h >= 8) & (h <= 38) & (s >= 25) & (s <= 95) & (v >= 70)
    white = (s <= 45) & (v >= 170)
    pink = ((h <= 15) | (h >= 160)) & (s >= 12) & (s <= 75) & (v >= 100)
    grey = (s <= 60) & (v >= 35) & (v <= 190)
    black = (s <= 90) & (v <= 55)  # dark fur, spots (exclude saturated dark soil)
    fur = orange_tan | white | pink | grey | black
    masked = comp_mask > 0
    n_masked = np.count_nonzero(masked)
    if n_masked == 0:
        return False
    return np.count_nonzero(masked & fur) / n_masked >= min_frac


def _contour_solidity(comp_mask: np.ndarray) -> float:
    """Solidity = area / convex_hull_area. Compact blobs ~0.8+, palm fronds ~0.5-0.7."""
    contours, _ = cv2.findContours(comp_mask, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
    if not contours:
        return 0.0
    c = max(contours, key=cv2.contourArea)
    area = cv2.contourArea(c)
    if area < 10:
        return 0.0
    hull = cv2.convexHull(c)
    hull_area = cv2.contourArea(hull)
    return area / hull_area if hull_area > 0 else 0.0


def connected_components(
    mask: np.ndarray,
    rgba: np.ndarray,
    min_area: int = 200,
    max_area: int = 12000,
    max_aspect: float = 2.2,
) -> list[tuple[int, np.ndarray, tuple[int, int, int, int]]]:
    """Find connected components. Returns (label, comp_mask, (x1,y1,x2,y2))."""
    num_labels, labels, stats, _ = cv2.connectedComponentsWithStats(mask, connectivity=8)
    components = []
    for i in range(1, num_labels):
        area = stats[i, cv2.CC_STAT_AREA]
        if area < min_area or area > max_area:
            continue
        x1 = stats[i, cv2.CC_STAT_LEFT]
        y1 = stats[i, cv2.CC_STAT_TOP]
        bw = stats[i, cv2.CC_STAT_WIDTH]
        bh = stats[i, cv2.CC_STAT_HEIGHT]
        x2, y2 = x1 + bw, y1 + bh
        aspect = max(bw, bh) / (min(bw, bh) or 1)
        if aspect > max_aspect:
            continue
        comp_mask = (labels == i).astype(np.uint8) * 255
        if _is_mostly_green(rgba, comp_mask):
            continue
        if _is_mostly_brown_trunk(rgba, comp_mask):
            continue
        if not _has_fur_colors(rgba, comp_mask):
            continue
        if _contour_solidity(comp_mask) < 0.42:
            continue
        if area > mask.size * 0.2 and num_labels == 2:
            split = _watershed_split(comp_mask, min_area, max_area, max_aspect, rgba)
            return split
        components.append((i, comp_mask, (x1, y1, x2, y2)))
    return components if components else [(1, mask, (0, 0, mask.shape[1], mask.shape[0]))]


def _watershed_split(
    mask: np.ndarray,
    min_area: int,
    max_area: int = 12000,
    max_aspect: float = 2.2,
    rgba: Optional[np.ndarray] = None,
) -> list[tuple[int, np.ndarray, tuple[int, int, int, int]]]:
    """Use distance transform + watershed to split one blob into many."""
    dist = cv2.distanceTransform(mask, cv2.DIST_L2, 5)
    dist_uint8 = np.uint8(np.clip(dist / (dist.max() or 1) * 255, 0, 255))
    _, sure_fg = cv2.threshold(dist_uint8, 0.15 * (dist_uint8.max() or 1), 255, 0)
    sure_fg = np.uint8(sure_fg)
    num_labels, labels, stats, _ = cv2.connectedComponentsWithStats(sure_fg, connectivity=8)
    if num_labels < 3:
        ys, xs = np.where(mask > 0)
        bbox = (xs.min(), ys.min(), xs.max() + 1, ys.max() + 1) if len(xs) else (0, 0, 0, 0)
        return [(1, mask, bbox)]
    out = []
    for i in range(1, num_labels):
        comp = (labels == i).astype(np.uint8) * 255
        kernel = np.ones((7, 7), np.uint8)
        comp = cv2.dilate(comp, kernel)
        comp = cv2.bitwise_and(comp, mask)
        area = np.count_nonzero(comp)
        if not (min_area <= area <= min(max_area, int(mask.size * 0.15))):
            continue
        ys, xs = np.where(comp > 0)
        if len(xs) == 0:
            continue
        x1, x2 = xs.min(), xs.max() + 1
        y1, y2 = ys.min(), ys.max() + 1
        asp = max(x2 - x1, y2 - y1) / (min(x2 - x1, y2 - y1) or 1)
        if asp <= max_aspect:
            if rgba is not None and _is_mostly_green(rgba, comp):
                continue
            if rgba is not None and _is_mostly_brown_trunk(rgba, comp):
                continue
            if rgba is not None and not _has_fur_colors(rgba, comp):
                continue
            if _contour_solidity(comp) < 0.42:
                continue
            out.append((i, comp, (x1, y1, x2, y2)))
    if not out:
        ys, xs = np.where(mask > 0)
        bbox = (xs.min(), ys.min(), xs.max() + 1, ys.max() + 1) if len(xs) else (0, 0, 0, 0)
        return [(1, mask, bbox)]
    return out


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
    return Image.fromarray(crop)


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


def run_pipeline_from_regions(
    mural_path: Path,
    out_dir: Path,
    regions_path: Path,
    num_rotations: int = 4,
    pixel_scale: float = 0.25,
) -> None:
    """Manual mode: use curated bboxes, rembg-only on each crop. No auto segmentation."""
    ensure_deps()
    if not HAS_REMBG:
        print("Manual regions require rembg: pip install rembg", file=sys.stderr)
        sys.exit(1)
    out_dir.mkdir(parents=True, exist_ok=True)

    data = json.loads(regions_path.read_text())
    regions = data.get("regions", data) if isinstance(data, dict) else data
    if not isinstance(regions, list):
        regions = [regions]

    rgba, pil = load_image(mural_path)
    h, w = rgba.shape[:2]

    cutouts = []
    for i, r in enumerate(regions):
        if isinstance(r, (list, tuple)) and len(r) >= 4:
            x1, y1, x2, y2 = int(r[0]), int(r[1]), int(r[2]), int(r[3])
        else:
            x1 = int(r.get("x1", 0))
            y1 = int(r.get("y1", 0))
            x2 = int(r.get("x2", 0))
            y2 = int(r.get("y2", 0))
        x1, x2 = min(x1, x2), max(x1, x2)
        y1, y2 = min(y1, y2), max(y1, y2)
        if x2 - x1 < 16 or y2 - y1 < 16:
            continue
        pad = max(8, min(24, (x2 - x1) // 4, (y2 - y1) // 4))
        cx1 = max(0, x1 - pad)
        cy1 = max(0, y1 - pad)
        cx2 = min(w, x2 + pad)
        cy2 = min(h, y2 + pad)
        crop_bbox = (cx1, cy1, cx2, cy2)

        refined = refine_crop_with_rembg(pil, crop_bbox)
        if refined is None:
            continue
        arr = np.array(refined)
        fg = np.count_nonzero(arr[:, :, 3] > 128)
        crop_area = arr.shape[0] * arr.shape[1]
        if fg < 80 or fg > crop_area * 0.95:
            continue
        cutout = refined

        comp_mask = np.zeros((h, w), dtype=np.uint8)
        cw, ch = cutout.size
        ox, oy = cx1, cy1
        alpha = np.array(cutout.split()[-1])
        mask_region = (alpha > 128).astype(np.uint8) * 255
        y_end = min(oy + ch, h)
        x_end = min(ox + cw, w)
        sy, sx = min(ch, y_end - oy), min(cw, x_end - ox)
        comp_mask[oy : oy + sy, ox : ox + sx] = np.maximum(
            comp_mask[oy : oy + sy, ox : ox + sx],
            mask_region[:sy, :sx],
        )

        cutout.save(out_dir / f"animal_{i:02d}_raw.png")
        cutouts.append((comp_mask, cutout))

    if not cutouts:
        print("No valid cutouts from regions. Check bboxes or install rembg.", file=sys.stderr)
        sys.exit(1)

    print(f"Extracted {len(cutouts)} animals from {len(regions)} regions (rembg)")

    animal_mask = np.zeros((h, w), dtype=np.uint8)
    for comp_mask, _ in cutouts:
        animal_mask = np.maximum(animal_mask, comp_mask)
    kernel = np.ones((5, 5), np.uint8)
    animal_mask = cv2.dilate(animal_mask, kernel)
    filled_bg = inpaint_background(rgba, animal_mask)
    bg_pil = Image.fromarray(filled_bg)
    bg_pil.save(out_dir / "background_filled.png")

    for i, (comp_mask, cutout) in enumerate(cutouts):
        pixelated = pixelate_8bit(cutout, scale=pixel_scale)
        pixelated = reduce_palette(pixelated, colors=16)
        angles = [360 * j / num_rotations for j in range(num_rotations)]
        for j, angle in enumerate(angles):
            rotated = rotate_cutout(pixelated, angle)
            rotated.save(out_dir / f"animal_{i:02d}_rot{j:02d}.png")

    placements = []
    for i, (comp_mask, _) in enumerate(cutouts):
        ys_c, xs_c = np.where(comp_mask > 0)
        if len(xs_c) == 0:
            continue
        cx, cy = int(xs_c.mean()), int(ys_c.mean())
        sprite = Image.open(out_dir / f"animal_{i:02d}_rot00.png")
        sw, sh = sprite.size
        placements.append((cx - sw // 2, cy - sh // 2, sprite))
    frame = composite_frame(filled_bg, placements)
    frame.save(out_dir / "frame_sample.png")

    _build_sprite_sheet(out_dir, len(cutouts), num_rotations)
    print(f"Done. Outputs in {out_dir}")


def run_pipeline(
    mural_path: Path,
    out_dir: Path,
    num_rotations: int = 4,
    pixel_scale: float = 0.25,
    min_animal_area: int = 200,
    max_animal_area: int = 12000,
) -> None:
    ensure_deps()
    out_dir.mkdir(parents=True, exist_ok=True)

    rgba, pil = load_image(mural_path)
    h, w = rgba.shape[:2]

    print("Segmenting foreground (color-based)...")
    mask = _segment_animals_by_color(rgba)
    cv2.imwrite(str(out_dir / "mask_full.png"), mask)

    print("Finding animal components (corner-sampling primary, rembg fallback)...")
    components = connected_components(
        mask, rgba, min_area=min_animal_area, max_area=max_animal_area, max_aspect=2.2
    )
    print(f"Found {len(components)} candidate regions")

    cutouts = []
    for i, (_, comp_mask, (x1, y1, x2, y2)) in enumerate(components):
        pad = max(24, int(0.4 * max(x2 - x1, y2 - y1)))
        cx1 = max(0, x1 - pad)
        cy1 = max(0, y1 - pad)
        cx2 = min(w, x2 + pad)
        cy2 = min(h, y2 + pad)
        crop_bbox = (cx1, cy1, cx2, cy2)

        cutout = None
        # Primary: Pic2Pix-style edge sampling (no ML deps, works for centered objects)
        refined = refine_crop_with_corner_sampling(pil, crop_bbox, tolerance=42, edge_strip=8)
        if refined is not None:
            arr = np.array(refined)
            fg = np.count_nonzero(arr[:, :, 3] > 128)
            crop_area = arr.shape[0] * arr.shape[1]
            if fg >= 120 and fg <= crop_area * 0.95:
                cutout = refined

        if cutout is None and HAS_REMBG:
            refined = refine_crop_with_rembg(pil, crop_bbox)
            if refined is not None:
                arr = np.array(refined)
                if np.count_nonzero(arr[:, :, 3] > 128) >= min_animal_area:
                    cutout = refined

        if cutout is None:
            cutout_pil = extract_cutout(rgba, comp_mask, padding=4)
            if cutout_pil is not None:
                cutout = cutout_pil

        if cutout is None:
            continue
        # Post-filter: reject cutouts that are mostly green or blue (foliage, water)
        arr = np.array(cutout)
        if arr.shape[2] >= 3:
            fg = arr[:, :, 3] > 128
            n_fg = np.count_nonzero(fg)
            if n_fg > 0:
                hsv = cv2.cvtColor(arr[:, :, :3], cv2.COLOR_RGB2HSV)
                green = (hsv[:, :, 0] >= 17) & (hsv[:, :, 0] <= 42) & (hsv[:, :, 1] > 40)
                blue = (hsv[:, :, 0] >= 42) & (hsv[:, :, 0] <= 70)
                n_green = np.count_nonzero(fg & green)
                n_blue = np.count_nonzero(fg & blue)
                if n_green / n_fg > 0.18 or n_blue / n_fg > 0.20:
                    continue
        cutout_path = out_dir / f"animal_{i:02d}_raw.png"
        cutout.save(cutout_path)
        cutouts.append((comp_mask, cutout))

    # Inpaint background: only the animal regions we extracted (not palm trees, etc.)
    print("Inpainting background (animal regions only)...")
    animal_mask = np.zeros((h, w), dtype=np.uint8)
    for comp_mask, _ in cutouts:
        animal_mask = np.maximum(animal_mask, comp_mask)
    kernel = np.ones((5, 5), np.uint8)
    animal_mask = cv2.dilate(animal_mask, kernel)
    filled_bg = inpaint_background(rgba, animal_mask)
    bg_pil = Image.fromarray(filled_bg)
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
    placements = []
    for i, (comp_mask, _) in enumerate(cutouts):
        ys_c, xs_c = np.where(comp_mask > 0)
        if len(xs_c) == 0:
            continue
        cx, cy = int(xs_c.mean()), int(ys_c.mean())
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
    meta = {"rows": rows, "cols": cols, "cell_w": max_w, "cell_h": max_h}
    import json
    (out_dir / "claymation_meta.json").write_text(json.dumps(meta))
    print(f"Sprite sheet: {cols}×{rows} cells, {max_w}×{max_h} each")


def main():
    parser = argparse.ArgumentParser(description="Mural claymation asset pipeline")
    parser.add_argument("mural", type=Path, nargs="?", default=Path("../../assets/mural.png"))
    parser.add_argument("-o", "--out", type=Path, default=Path("out_claymation"))
    parser.add_argument("--regions", type=Path, default=None, help="JSON with manual bboxes (rembg-only, no auto)")
    parser.add_argument("--rotations", type=int, default=4)
    parser.add_argument("--pixel-scale", type=float, default=1.0)
    parser.add_argument("--min-area", type=int, default=200)
    parser.add_argument("--max-area", type=int, default=12000)
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
    if args.regions is not None:
        regions_path = (script_dir / args.regions).resolve() if not args.regions.is_absolute() else args.regions
        if not regions_path.exists():
            print(f"Regions file not found: {regions_path}", file=sys.stderr)
            sys.exit(1)
        run_pipeline_from_regions(
            mural_path=mural,
            out_dir=out,
            regions_path=regions_path,
            num_rotations=args.rotations,
            pixel_scale=args.pixel_scale,
        )
    else:
        run_pipeline(
            mural_path=mural,
            out_dir=out,
            num_rotations=args.rotations,
            pixel_scale=args.pixel_scale,
            min_animal_area=args.min_area,
            max_animal_area=args.max_area,
        )


if __name__ == "__main__":
    main()

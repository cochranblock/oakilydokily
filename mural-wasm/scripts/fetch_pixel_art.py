#!/usr/bin/env python3
# Unlicense — cochranblock.org
# Fetch CC0 pixel art (CatnDog + Kenney) and build 4×3 sprite sheet for mural-wasm.
# Run from mural-wasm/scripts/ or project root.

import os
import subprocess
import sys
from pathlib import Path
from typing import Optional

from PIL import Image

CELL_W = 64
CELL_H = 64
COLS = 4
ROWS = 3

CATNDOG_URL = "https://opengameart.org/sites/default/files/CatnDog.zip"


def draw_guinea_pig_cell() -> Image.Image:
    """Cute pixel-art guinea pig, side view, 64x64. Matches CatnDog style."""
    img = Image.new("RGBA", (CELL_W, CELL_H), (0, 0, 0, 0))
    px = img.load()
    # Palette: warm browns, cream
    outline = (80, 50, 30, 255)
    dark = (140, 95, 55, 255)
    mid = (185, 140, 90, 255)
    light = (220, 190, 150, 255)
    cream = (245, 230, 210, 255)
    eye = (30, 20, 10, 255)

    def oval(cx: int, cy: int, rx: int, ry: int, color: tuple) -> None:
        for y in range(max(0, cy - ry), min(CELL_H, cy + ry + 1)):
            for x in range(max(0, cx - rx), min(CELL_W, cx + rx + 1)):
                if ((x - cx) / rx) ** 2 + ((y - cy) / ry) ** 2 <= 1:
                    px[x, y] = color

    # Body (compact oval, bottom)
    oval(32, 42, 18, 14, mid)
    oval(32, 42, 14, 10, light)
    # Head (round, front)
    oval(32, 28, 14, 12, light)
    oval(32, 28, 10, 8, cream)
    # Snout
    oval(38, 28, 6, 5, cream)
    # Ears (small rounded)
    oval(22, 18, 5, 4, mid)
    oval(42, 18, 5, 4, mid)
    # Eyes
    px[28, 26] = eye
    px[36, 26] = eye
    # Outline hint on body
    for (x, y) in [(16, 38), (48, 38), (20, 52), (44, 52)]:
        if 0 <= x < CELL_W and 0 <= y < CELL_H:
            px[x, y] = outline
    return img


def fetch_zip(url: str, dest: Path) -> Path:
    dest.mkdir(parents=True, exist_ok=True)
    zip_path = dest / "archive.zip"
    subprocess.run(
        ["curl", "-sL", url, "-o", str(zip_path)],
        check=True,
        capture_output=True,
    )
    subprocess.run(
        ["unzip", "-o", str(zip_path), "-d", str(dest)],
        check=True,
        capture_output=True,
    )
    zip_path.unlink(missing_ok=True)
    return dest


def build_sprite_sheet(out_path: Path, catndog_dir: Path) -> None:
    sheet = Image.new("RGBA", (COLS * CELL_W, ROWS * CELL_H), (0, 0, 0, 0))

    def paste_cell(img: Image.Image, col: int, row: int) -> None:
        img = img.convert("RGBA")
        if img.width > CELL_W or img.height > CELL_H:
            r = min(CELL_W / img.width, CELL_H / img.height)
            new_w, new_h = int(img.width * r), int(img.height * r)
            img = img.resize((new_w, new_h), Image.Resampling.NEAREST)
        x = col * CELL_W + (CELL_W - img.width) // 2
        y = (row + 1) * CELL_H - img.height
        sheet.paste(img, (x, y), img)

    # Row 0: Cat Walk 1-4 (pzUH, CC0)
    cat_dir = catndog_dir / "png" / "cat"
    for i in range(4):
        f = cat_dir / f"Walk ({i + 1}).png"
        if f.exists():
            paste_cell(Image.open(f), i, 0)

    # Row 1: Dog Walk 1-4
    dog_dir = catndog_dir / "png" / "dog"
    for i in range(4):
        f = dog_dir / f"Walk ({i + 1}).png"
        if f.exists():
            paste_cell(Image.open(f), i, 1)

    # Row 2: Guinea pig (embedded minimal sprite, repeated 4x)
    gp = draw_guinea_pig_cell()
    for i in range(4):
        sheet.paste(gp, (i * CELL_W, 2 * CELL_H), gp)

    sheet.save(out_path)
    print(f"Wrote {out_path}")


def find_dir_containing(parent: Path, rel: Path) -> Optional[Path]:
    """Find a directory (parent or descendant) that contains rel."""
    if (parent / rel).exists():
        return parent
    for d in parent.iterdir():
        if d.is_dir() and (d / rel).exists():
            return d
    return None


def main() -> None:
    script_dir = Path(__file__).resolve().parent
    cache = script_dir / ".pixel_art_cache"
    cache.mkdir(parents=True, exist_ok=True)
    out_dir = script_dir.parent / "assets"
    out_dir.mkdir(parents=True, exist_ok=True)
    out_path = out_dir / "pets_spritesheet.png"

    catndog = find_dir_containing(cache, Path("png/cat"))
    if not catndog:
        print("Fetching CatnDog (pzUH, CC0)...")
        fetch_zip(CATNDOG_URL, cache)
        catndog = find_dir_containing(cache, Path("png/cat"))
    if not catndog:
        print("CatnDog not found", file=sys.stderr)
        sys.exit(1)

    build_sprite_sheet(out_path, catndog)
    print("Done. pets_spritesheet.png ready.")


if __name__ == "__main__":
    main()

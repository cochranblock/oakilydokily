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
    """Minimal CC0-style guinea pig: round body, small ears, 64x64."""
    img = Image.new("RGBA", (CELL_W, CELL_H), (0, 0, 0, 0))
    px = img.load()
    # Brown/tan palette
    dark = (139, 90, 43, 255)
    mid = (180, 130, 70, 255)
    light = (210, 170, 120, 255)
    # Round body (ellipse)
    for y in range(20, 56):
        for x in range(16, 48):
            dx, dy = (x - 32) / 16, (y - 38) / 18
            if dx * dx + dy * dy < 1.0:
                px[x, y] = mid if (x + y) % 4 < 2 else light
    # Head
    for y in range(12, 28):
        for x in range(20, 44):
            if (x - 32) ** 2 / 64 + (y - 20) ** 2 / 36 < 1:
                px[x, y] = light if y < 22 else mid
    # Ears
    for (ex, ey) in [(18, 14), (46, 14)]:
        for dy in range(6):
            for dx in range(-2, 3):
                if abs(dx) + dy < 4:
                    px[ex + dx, ey + dy] = mid
    # Eye dots
    px[26, 22] = (40, 25, 15, 255)
    px[38, 22] = (40, 25, 15, 255)
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

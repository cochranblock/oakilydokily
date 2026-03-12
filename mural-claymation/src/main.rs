//! Unlicense — cochranblock.org
//! Mural claymation pipeline: segment animals, inpaint, pixelate, rotate, composite.

use std::path::PathBuf;

use clap::Parser;
use image::{imageops::FilterType, GenericImageView, ImageBuffer, Luma, Rgba, RgbaImage};
use imageproc::geometric_transformations::{rotate_about_center, Interpolation};
use imageproc::region_labelling::{connected_components, Connectivity};
use inpaint::prelude::*;

#[derive(Parser)]
#[command(about = "Mural claymation asset pipeline")]
struct Args {
    #[arg(default_value = "../../assets/mural.png")]
    mural: PathBuf,
    #[arg(short, long, default_value = "out_claymation")]
    out: PathBuf,
    #[arg(long, default_value = "4")]
    rotations: usize,
    #[arg(long, default_value = "0.25")]
    pixel_scale: f32,
    #[arg(long, default_value = "150")]
    min_area: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mural = resolve_mural(&args.mural)?;
    run_pipeline(&mural, &args.out, args.rotations, args.pixel_scale, args.min_area)?;
    Ok(())
}

fn resolve_mural(mural: &PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
    if mural.exists() {
        return Ok(mural.clone());
    }
    let candidates = [
        "mural.png",
        "assets/mural.png",
        "oakilydokily/assets/mural.png",
        "oakilydokily/mural-wasm/assets/mural.png",
    ];
    for c in candidates {
        let p = PathBuf::from(c);
        if p.exists() {
            return Ok(p);
        }
    }
    Err(format!("Mural not found: {}", mural.display()).into())
}

fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let v = max;
    let s = if max > 0.0 { (max - min) / max } else { 0.0 };
    let h = if s == 0.0 {
        0.0
    } else {
        let d = max - min;
        let h = match max {
            x if (x - r).abs() < 1e-6 => (g - b) / d + (if g < b { 6.0 } else { 0.0 }),
            x if (x - g).abs() < 1e-6 => (b - r) / d + 2.0,
            _ => (r - g) / d + 4.0,
        };
        (h / 6.0) * 360.0
    };
    (h, s * 100.0, v * 100.0)
}

fn segment_animals_by_color(img: &RgbaImage) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let (w, h) = img.dimensions();
    let mut mask = ImageBuffer::new(w, h);
    for y in 0..h {
        for x in 0..w {
            let p = img.get_pixel(x, y);
            let (h, s, v) = rgb_to_hsv(p[0], p[1], p[2]);
            let h_180 = h / 2.0;
            let green = (17.0..=42.0).contains(&h_180) && s > 40.0;
            let blue = (45.0..=65.0).contains(&h_180);
            let background = green || blue || s < 25.0;
            let animal = !background && v > 30.0;
            mask.put_pixel(x, y, Luma([if animal { 255 } else { 0 }]));
        }
    }
    let mut m = mask;
    m = morph_erode(&m);
    m = morph_dilate(&m);
    m = morph_dilate(&m);
    m = morph_erode(&m);
    m
}

fn morph_erode(img: &ImageBuffer<Luma<u8>, Vec<u8>>) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let (w, h) = img.dimensions();
    let mut out = ImageBuffer::new(w, h);
    for y in 0..h {
        for x in 0..w {
            let mut min_val = 255u8;
            for dy in -1..=1i32 {
                for dx in -1..=1i32 {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && nx < w as i32 && ny >= 0 && ny < h as i32 {
                        min_val = min_val.min(img.get_pixel(nx as u32, ny as u32)[0]);
                    }
                }
            }
            out.put_pixel(x, y, Luma([min_val]));
        }
    }
    out
}

fn morph_dilate(img: &ImageBuffer<Luma<u8>, Vec<u8>>) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let (w, h) = img.dimensions();
    let mut out = ImageBuffer::new(w, h);
    for y in 0..h {
        for x in 0..w {
            let mut max_val = 0u8;
            for dy in -1..=1i32 {
                for dx in -1..=1i32 {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && nx < w as i32 && ny >= 0 && ny < h as i32 {
                        max_val = max_val.max(img.get_pixel(nx as u32, ny as u32)[0]);
                    }
                }
            }
            out.put_pixel(x, y, Luma([max_val]));
        }
    }
    out
}


fn run_pipeline(
    mural_path: &PathBuf,
    out_dir: &PathBuf,
    num_rotations: usize,
    pixel_scale: f32,
    min_area: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(out_dir)?;
    let img = image::open(mural_path)?.to_rgba8();

    println!("Segmenting foreground...");
    let mask = segment_animals_by_color(&img);
    mask.save(out_dir.join("mask_full.png"))?;

    println!("Finding animal components...");
    let mut components = Vec::new();
    let labeled = connected_components(&mask, Connectivity::Eight, Luma([0u8]));
    let max_label = labeled.pixels().map(|p| p[0]).max().unwrap_or(0);
    for label in 1..=max_label {
        let mut comp_mask = ImageBuffer::new(labeled.width(), labeled.height());
        let mut area = 0usize;
        for y in 0..labeled.height() {
            for x in 0..labeled.width() {
                let l = labeled.get_pixel(x, y)[0];
                if l == label {
                    comp_mask.put_pixel(x, y, Luma([255]));
                    area += 1;
                }
            }
        }
        if area >= min_area {
            components.push(comp_mask);
        }
    }
    println!("Found {} animal regions", components.len());

    if components.is_empty() {
        components.push(mask.clone());
    }

    let mut cutouts: Vec<(RgbaImage, (u32, u32))> = Vec::new();
    for (i, comp_mask) in components.iter().enumerate() {
        let cutout = extract_cutout(&img, comp_mask, 2);
        if let Some((crop, _)) = &cutout {
            crop.save(out_dir.join(format!("animal_{:02}_raw.png", i)))?;
            cutouts.push(cutout.unwrap());
        }
    }

    println!("Inpainting background...");
    let filled = inpaint_background(&img, &mask);
    filled.save(out_dir.join("background_filled.png"))?;

    println!("Generating 8-bit rotated sprites...");
    for (i, (cutout, _)) in cutouts.iter().enumerate() {
        let pixelated = pixelate_8bit(cutout, pixel_scale);
        let reduced = reduce_palette(&pixelated, 16);
        for j in 0..num_rotations {
            let angle = 360.0 * j as f32 / num_rotations as f32;
            let rotated = rotate(&reduced, angle);
            rotated.save(out_dir.join(format!("animal_{:02}_rot{:02}.png", i, j)))?;
        }
    }

    println!("Compositing sample frame...");
    let mut placements = Vec::new();
    for (i, comp_mask) in components.iter().enumerate() {
        let (cx, cy) = centroid(comp_mask);
        let sprite_path = out_dir.join(format!("animal_{:02}_rot00.png", i));
        if let Ok(sprite) = image::open(&sprite_path) {
            let sprite = sprite.to_rgba8();
            let (sw, sh) = sprite.dimensions();
            let x = cx.saturating_sub(sw / 2) as i32;
            let y = cy.saturating_sub(sh / 2) as i32;
            placements.push((x, y, sprite));
        }
    }
    let frame = composite(&filled, &placements);
    frame.save(out_dir.join("frame_sample.png"))?;

    println!("Building sprite sheet...");
    build_sprite_sheet(out_dir, cutouts.len(), num_rotations)?;

    println!("Done. Outputs in {}", out_dir.display());
    Ok(())
}

fn extract_cutout(
    img: &RgbaImage,
    mask: &ImageBuffer<Luma<u8>, Vec<u8>>,
    padding: u32,
) -> Option<(RgbaImage, (u32, u32))> {
    let (w, h) = img.dimensions();
    let mut x1 = w;
    let mut x2 = 0u32;
    let mut y1 = h;
    let mut y2 = 0u32;
    for y in 0..h {
        for x in 0..w {
            if mask.get_pixel(x, y)[0] > 0 {
                x1 = x1.min(x);
                x2 = x2.max(x);
                y1 = y1.min(y);
                y2 = y2.max(y);
            }
        }
    }
    if x2 < x1 {
        return None;
    }
    let x1 = x1.saturating_sub(padding).max(0);
    let y1 = y1.saturating_sub(padding).max(0);
    let x2 = (x2 + padding + 1).min(w);
    let y2 = (y2 + padding + 1).min(h);
    let crop = image::imageops::crop_imm(img, x1, y1, x2 - x1, y2 - y1);
    let mut out = ImageBuffer::new(x2 - x1, y2 - y1);
    for y in 0..(y2 - y1) {
        for x in 0..(x2 - x1) {
            let p = crop.get_pixel(x, y);
            let m = mask.get_pixel(x1 + x, y1 + y)[0];
            out.put_pixel(
                x,
                y,
                Rgba([p[0], p[1], p[2], if m > 0 { p[3] } else { 0 }]),
            );
        }
    }
    Some((out, (x1, y1)))
}

fn inpaint_background(img: &RgbaImage, mask: &ImageBuffer<Luma<u8>, Vec<u8>>) -> RgbaImage {
    let mut img_copy = img.clone();
    img_copy.telea_inpaint(mask, 3).unwrap();
    img_copy
}

fn pixelate_8bit(img: &RgbaImage, scale: f32) -> RgbaImage {
    let (w, h) = img.dimensions();
    let sw = (w as f32 * scale).max(4.0) as u32;
    let sh = (h as f32 * scale).max(4.0) as u32;
    let small = image::imageops::resize(img, sw, sh, FilterType::Nearest);
    image::imageops::resize(&small, w, h, FilterType::Nearest)
}

fn reduce_palette(img: &RgbaImage, _colors: usize) -> RgbaImage {
    img.clone()
}

fn rotate(img: &RgbaImage, angle_deg: f32) -> RgbaImage {
    rotate_about_center(
        img,
        angle_deg.to_radians(),
        Interpolation::Nearest,
        Rgba([0, 0, 0, 0]),
    )
}

fn centroid(mask: &ImageBuffer<Luma<u8>, Vec<u8>>) -> (u32, u32) {
    let (w, h) = mask.dimensions();
    let mut sx = 0u64;
    let mut sy = 0u64;
    let mut n = 0u64;
    for y in 0..h {
        for x in 0..w {
            if mask.get_pixel(x, y)[0] > 0 {
                sx += x as u64;
                sy += y as u64;
                n += 1;
            }
        }
    }
    if n == 0 {
        return (w / 2, h / 2);
    }
    ((sx / n) as u32, (sy / n) as u32)
}

fn composite(
    bg: &RgbaImage,
    placements: &[(i32, i32, RgbaImage)],
) -> RgbaImage {
    let mut out = bg.clone();
    for (ox, oy, sprite) in placements {
        let (sw, sh) = sprite.dimensions();
        for y in 0..sh {
            for x in 0..sw {
                let nx = *ox + x as i32;
                let ny = *oy + y as i32;
                if nx >= 0 && ny >= 0 {
                    let p = sprite.get_pixel(x, y);
                    if p[3] > 0 {
                        let (bw, bh) = out.dimensions();
                        if (nx as u32) < bw && (ny as u32) < bh {
                            out.put_pixel(nx as u32, ny as u32, *p);
                        }
                    }
                }
            }
        }
    }
    out
}

fn build_sprite_sheet(
    out_dir: &PathBuf,
    num_animals: usize,
    num_rots: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    const MAX_CELL: u32 = 128;
    let mut sprites: Vec<Vec<RgbaImage>> = Vec::new();
    let mut max_w = 0u32;
    let mut max_h = 0u32;
    for i in 0..num_animals {
        let mut row = Vec::new();
        for j in 0..num_rots {
            let p = out_dir.join(format!("animal_{:02}_rot{:02}.png", i, j));
            if p.exists() {
                let mut img = image::open(&p)?.to_rgba8();
                let (w, h) = img.dimensions();
                if w > MAX_CELL || h > MAX_CELL {
                    let r = (MAX_CELL as f32 / w.max(h) as f32).min(1.0);
                    img = image::imageops::resize(
                        &img,
                        (w as f32 * r) as u32,
                        (h as f32 * r) as u32,
                        FilterType::Nearest,
                    );
                }
                let (w, h) = img.dimensions();
                max_w = max_w.max(w);
                max_h = max_h.max(h);
                row.push(img);
            }
        }
        if !row.is_empty() {
            sprites.push(row);
        }
    }
    if sprites.is_empty() {
        return Ok(());
    }
    let cols = num_rots;
    let rows = sprites.len();
    let sheet_w = cols as u32 * max_w;
    let sheet_h = rows as u32 * max_h;
    let mut sheet = ImageBuffer::from_fn(sheet_w, sheet_h, |_, _| Rgba([0, 0, 0, 0]));
    for (r, row) in sprites.iter().enumerate() {
        for (c, sprite) in row.iter().enumerate() {
            let (sw, sh) = sprite.dimensions();
            for y in 0..sh {
                for x in 0..sw {
                    let sheet_x = c as u32 * max_w + x;
                    let sheet_y = r as u32 * max_h + y;
                    let p = sprite.get_pixel(x, y);
                    if p[3] > 0 {
                        sheet.put_pixel(sheet_x, sheet_y, *p);
                    }
                }
            }
        }
    }
    sheet.save(out_dir.join("claymation_spritesheet.png"))?;
    let meta = format!(
        r#"{{"rows":{},"cols":{},"cell_w":{},"cell_h":{}}}"#,
        rows, cols, max_w, max_h
    );
    std::fs::write(out_dir.join("claymation_meta.json"), meta)?;
    println!("Sprite sheet: {}×{} cells, {}×{} each", cols, rows, max_w, max_h);
    Ok(())
}

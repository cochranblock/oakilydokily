// Unlicense — cochranblock.org
// Claymation sheet (preferred) or fallback 4×3 sprite sheet.

use macroquad::prelude::*;
use macroquad::texture::FilterMode;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ClaymationMeta {
    rows: u32,
    cols: u32,
    cell_w: u32,
    cell_h: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Species {
    Cat,
    Dog,
    GuineaPig,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Animation {
    Walk,
    Interaction,
    Sleeping,
}

/// Claymation sprite sheet from Python pipeline (crop-then-rembg).
#[derive(Debug)]
pub struct ClaymationSheet {
    pub texture: Texture2D,
    pub rows: u32,
    pub cols: u32,
    pub cell_w: f32,
    pub cell_h: f32,
}

impl ClaymationSheet {
    pub async fn load() -> Option<Self> {
        let texture = match load_texture("/assets/claymation_spritesheet.png").await {
            Ok(t) => t,
            Err(_) => return None,
        };
        texture.set_filter(FilterMode::Nearest);
        let (w, h) = (texture.width(), texture.height());
        let meta: ClaymationMeta = match macroquad::file::load_string("/assets/claymation_meta.json").await {
            Ok(s) => serde_json::from_str(&s).ok()?,
            Err(_) => ClaymationMeta {
                rows: (h / 64.).ceil() as u32,
                cols: 4,
                cell_w: 64,
                cell_h: 64,
            },
        };
        let cell_w = if meta.cols > 0 { w / meta.cols as f32 } else { 32. };
        let cell_h = if meta.rows > 0 { h / meta.rows as f32 } else { 32. };
        Some(ClaymationSheet {
            texture,
            rows: meta.rows,
            cols: meta.cols,
            cell_w,
            cell_h,
        })
    }

    pub fn frame(&self, animal: u32, rotation: u32) -> Rect {
        let row = animal.min(self.rows.saturating_sub(1));
        let col = rotation % self.cols;
        let x = col as f32 * self.cell_w;
        let y = row as f32 * self.cell_h;
        Rect::new(x, y, self.cell_w, self.cell_h)
    }
}

/// Fallback: 4×3 sprite sheet (Cat, Dog, GuineaPig).
#[derive(Debug)]
pub struct SpriteSheet {
    pub texture: Texture2D,
    pub cols: u32,
    pub rows: u32,
    pub cell_w: f32,
    pub cell_h: f32,
}

impl SpriteSheet {
    pub async fn load(path: &str) -> Self {
        let texture = match load_texture(path).await {
            Ok(t) => t,
            Err(_) => {
                let img = Image::gen_image_color(128, 96, WHITE);
                let t = Texture2D::from_image(&img);
                t.set_filter(FilterMode::Nearest);
                return SpriteSheet {
                    texture: t,
                    cols: 4,
                    rows: 3,
                    cell_w: 32.,
                    cell_h: 32.,
                };
            }
        };
        texture.set_filter(FilterMode::Nearest);
        let (w, h) = (texture.width(), texture.height());
        SpriteSheet {
            texture,
            cols: 4,
            rows: 3,
            cell_w: w / 4.,
            cell_h: h / 3.,
        }
    }

    pub fn cell_rect(&self, col: u32, row: u32) -> Rect {
        Rect::new(
            col as f32 * self.cell_w,
            row as f32 * self.cell_h,
            self.cell_w,
            self.cell_h,
        )
    }
}

#[derive(Debug)]
pub struct TextureAtlas<'a> {
    sheet: &'a SpriteSheet,
}

impl<'a> TextureAtlas<'a> {
    pub fn from_sheet(sheet: &'a SpriteSheet) -> Self {
        TextureAtlas { sheet }
    }

    pub fn frame(&self, species: Species, anim: Animation, frame: u32) -> Rect {
        let row = match species {
            Species::Cat => 0,
            Species::Dog => 1,
            Species::GuineaPig => 2,
        };
        let col = match anim {
            Animation::Walk => 0,
            Animation::Interaction => 1,
            Animation::Sleeping => 2,
        };
        self.sheet.cell_rect((col + frame) % self.sheet.cols, row)
    }

    pub fn kiss_frame(&self, frame: u32) -> Rect {
        self.sheet.cell_rect(3.min(frame), 2)
    }

    pub fn texture(&self) -> &Texture2D {
        &self.sheet.texture
    }
}

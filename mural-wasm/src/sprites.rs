// Unlicense — cochranblock.org
// Contributors: mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
//! Claymation sprite sheet: animals from mural, cols=rotations, rows=animals.

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

/// Claymation sprite sheet: 8-bit animals from mural, grid cols=rotations (4), rows=animals.
#[derive(Debug)]
pub struct ClaymationSheet {
    pub texture: Texture2D,
    pub rows: u32,
    pub cols: u32,
    pub cell_w: f32,
    pub cell_h: f32,
}

impl ClaymationSheet {
    /// Load claymation_spritesheet.png + claymation_meta.json. Fallback to placeholder if missing.
    pub async fn load() -> Self {
        let texture = match load_texture("/assets/claymation_spritesheet.png").await {
            Ok(t) => t,
            Err(_) => {
                let img = Image::gen_image_color(128, 96, WHITE);
                let t = Texture2D::from_image(&img);
                t.set_filter(FilterMode::Nearest);
                return ClaymationSheet {
                    texture: t,
                    rows: 3,
                    cols: 4,
                    cell_w: 32.,
                    cell_h: 32.,
                };
            }
        };
        texture.set_filter(FilterMode::Nearest);
        let (w, h) = (texture.width(), texture.height());

        let meta: ClaymationMeta = match macroquad::file::load_string("/assets/claymation_meta.json").await {
            Ok(s) => serde_json::from_str(&s).unwrap_or_else(|_| ClaymationMeta {
                rows: (h / 128.).ceil() as u32,
                cols: 4,
                cell_w: 128,
                cell_h: 128,
            }),
            Err(_) => ClaymationMeta {
                rows: (h / 128.).ceil() as u32,
                cols: 4,
                cell_w: 128,
                cell_h: 128,
            },
        };

        let cell_w = if meta.cols > 0 {
            w / meta.cols as f32
        } else {
            32.
        };
        let cell_h = if meta.rows > 0 {
            h / meta.rows as f32
        } else {
            32.
        };

        ClaymationSheet {
            texture,
            rows: meta.rows,
            cols: meta.cols,
            cell_w,
            cell_h,
        }
    }

    pub fn cell_rect(&self, col: u32, row: u32) -> Rect {
        let x = col as f32 * self.cell_w;
        let y = row as f32 * self.cell_h;
        Rect::new(x, y, self.cell_w, self.cell_h)
    }

    /// Frame for animal at row, rotation at col. Rotation: 0=right, 1=down, 2=left, 3=up.
    pub fn frame(&self, animal: u32, rotation: u32) -> Rect {
        let row = animal.min(self.rows.saturating_sub(1));
        let col = rotation % self.cols;
        self.cell_rect(col, row)
    }
}

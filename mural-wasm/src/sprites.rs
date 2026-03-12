// Unlicense — cochranblock.org
// Contributors: mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
//! SpriteSheet: 4×3 grid. TextureAtlas: Cats row 0, Dogs row 1, Guinea Pigs row 2.

use macroquad::prelude::*;
use macroquad::texture::FilterMode;

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
        let x = col as f32 * self.cell_w;
        let y = row as f32 * self.cell_h;
        Rect::new(x, y, self.cell_w, self.cell_h)
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
        let col = (col + frame) % self.sheet.cols;
        self.sheet.cell_rect(col, row)
    }

    pub fn kiss_frame(&self, frame: u32) -> Rect {
        self.sheet.cell_rect(3.min(frame), 2)
    }

    pub fn texture(&self) -> &Texture2D {
        &self.sheet.texture
    }
}

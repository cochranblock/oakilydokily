// Unlicense — cochranblock.org
// Contributors: mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
//! Claymation animal: wandering, exodus. Rotation from velocity direction.

use macroquad::prelude::*;
use crate::sprites::ClaymationSheet;

/// t124=PetState.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PetState {
    Wandering,
    /// Moving toward house exit (Doggy Door exodus)
    Exodus,
}

/// t123=Pet. Claymation animal from mural.
pub struct Pet {
    pub animal: u32,
    pub pos: Vec2,
    pub vel: Vec2,
    pub state: PetState,
    pub anim_frame: u32,
    pub anim_timer: f32,
}

impl Pet {
    pub fn new(animal: u32, pos: Vec2) -> Self {
        Pet {
            animal,
            pos,
            vel: vec2(20., 0.),
            state: PetState::Wandering,
            anim_frame: 0,
            anim_timer: 0.,
        }
    }

    pub fn enter_exodus(&mut self) {
        self.state = PetState::Exodus;
        self.vel = vec2(-80., 0.);
    }

    pub fn update(&mut self, dt: f32, _sheet: &ClaymationSheet, mural_w: f32, _mural_h: f32) {
        self.anim_timer += dt;
        if self.anim_timer > 0.15 {
            self.anim_timer = 0.;
            self.anim_frame = (self.anim_frame + 1) % 4;
        }

        match self.state {
            PetState::Wandering => {
                self.pos += self.vel * dt;
                if self.pos.x < 32. || self.pos.x > mural_w - 32. {
                    self.vel.x = -self.vel.x;
                }
            }
            PetState::Exodus => {
                self.pos += self.vel * dt;
            }
        }
    }

    /// Rotation index from velocity: 0=right, 2=left for 4-col sheet.
    fn rotation_from_vel(&self) -> u32 {
        if self.vel.x >= 0. {
            0
        } else {
            2
        }
    }

    pub fn draw(&self, sheet: &ClaymationSheet, scale: f32, ox: f32, oy: f32) {
        if self.state == PetState::Exodus && self.pos.x < -50. {
            return;
        }
        let s = scale;
        let x = ox + self.pos.x * s;
        let y = oy + self.pos.y * s;
        let rot = self.rotation_from_vel();
        let uv = sheet.frame(self.animal, rot);
        let w = uv.w * s;
        let h = uv.h * s;
        draw_texture_ex(
            &sheet.texture,
            x - w / 2.,
            y - h,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(w, h)),
                source: Some(uv),
                ..Default::default()
            },
        );
    }
}

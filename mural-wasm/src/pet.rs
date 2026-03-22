// Unlicense — cochranblock.org
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

// Pet: claymation animal or fallback Cat/Dog/GuineaPig.

use macroquad::prelude::*;
use crate::sprites::{ClaymationSheet, ForgedSheet, TextureAtlas, Species, Animation};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PetKind {
    Claymation(u32),
    Forged(u32),
    Sprite(Species),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PetState {
    Wandering,
    Sleeping,
    Interacting,
    Exodus,
}

pub struct Pet {
    pub kind: PetKind,
    pub pos: Vec2,
    pub vel: Vec2,
    pub state: PetState,
    pub anim_frame: u32,
    pub anim_timer: f32,
    pub interaction_timer: f32,
    hearts: Vec<HeartParticle>,
}

struct HeartParticle {
    pos: Vec2,
    vel: Vec2,
    life: f32,
}

impl Pet {
    pub fn claymation(animal: u32, pos: Vec2) -> Self {
        Pet {
            kind: PetKind::Claymation(animal),
            pos,
            vel: vec2(20., 0.),
            state: PetState::Wandering,
            anim_frame: 0,
            anim_timer: 0.,
            interaction_timer: 0.,
            hearts: vec![],
        }
    }

    pub fn forged(index: u32, pos: Vec2) -> Self {
        Pet {
            kind: PetKind::Forged(index),
            pos,
            vel: vec2(20., 0.),
            state: PetState::Wandering,
            anim_frame: 0,
            anim_timer: 0.,
            interaction_timer: 0.,
            hearts: vec![],
        }
    }

    pub fn sprite(species: Species, pos: Vec2, _atlas: &TextureAtlas) -> Self {
        Pet {
            kind: PetKind::Sprite(species),
            pos,
            vel: vec2(20., 0.),
            state: PetState::Wandering,
            anim_frame: 0,
            anim_timer: 0.,
            interaction_timer: 0.,
            hearts: vec![],
        }
    }

    pub fn trigger_kiss(&mut self) {
        self.hearts.push(HeartParticle {
            pos: self.pos + vec2(0., -20.),
            vel: vec2(0., -30.),
            life: 1.,
        });
    }

    pub fn enter_exodus(&mut self) {
        self.state = PetState::Exodus;
        self.vel = vec2(-80., 0.);
    }

    pub fn same_kind(&self, other: &Pet) -> bool {
        match (&self.kind, &other.kind) {
            (PetKind::Sprite(a), PetKind::Sprite(b)) => a == b,
            (PetKind::Forged(a), PetKind::Forged(b)) => a == b,
            _ => false,
        }
    }

    pub fn update(&mut self, dt: f32, mural_w: f32, _mural_h: f32) {
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
            PetState::Interacting => {
                self.interaction_timer += dt;
                if self.interaction_timer > 2.0 {
                    self.interaction_timer = 0.;
                    self.state = PetState::Wandering;
                    let dir = if (self.pos.x * 0.1) as i32 % 2 == 0 { 1. } else { -1. };
                    self.vel = vec2(20. * dir, 0.);
                }
            }
            PetState::Sleeping => {}
            PetState::Exodus => self.pos += self.vel * dt,
        }

        self.hearts.retain_mut(|h| {
            h.pos += h.vel * dt;
            h.vel.y -= 50. * dt;
            h.life -= dt;
            h.life > 0.
        });
    }

    pub fn draw_claymation(&self, sheet: &ClaymationSheet, scale: f32, ox: f32, oy: f32) {
        if self.state == PetState::Exodus && self.pos.x < -50. {
            return;
        }
        let PetKind::Claymation(animal) = self.kind else { return };
        let s = scale;
        let x = ox + self.pos.x * s;
        let y = oy + self.pos.y * s;
        let rot = if self.vel.x >= 0. { 0 } else { 2 };
        let uv = sheet.frame(animal, rot);
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

    pub fn draw_forged(&self, sheet: &ForgedSheet, scale: f32, ox: f32, oy: f32) {
        if self.state == PetState::Exodus && self.pos.x < -50. {
            return;
        }
        let PetKind::Forged(index) = self.kind else { return };
        let s = scale;
        let x = ox + self.pos.x * s;
        let y = oy + self.pos.y * s;
        let uv = sheet.frame(index);
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

    pub fn draw_sprite(&self, atlas: &TextureAtlas, scale: f32, ox: f32, oy: f32) {
        if self.state == PetState::Exodus && self.pos.x < -50. {
            return;
        }
        let PetKind::Sprite(species) = self.kind else { return };
        let s = scale;
        let x = ox + self.pos.x * s;
        let y = oy + self.pos.y * s;
        let w = 48. * s;
        let h = 56. * s;
        let anim = match self.state {
            PetState::Wandering | PetState::Exodus => Animation::Walk,
            PetState::Sleeping => Animation::Sleeping,
            PetState::Interacting => {
                if species == Species::GuineaPig {
                    let uv = atlas.kiss_frame(self.anim_frame);
                    draw_texture_ex(
                        atlas.texture(),
                        x - w / 2.,
                        y - h,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(w, h)),
                            source: Some(uv),
                            ..Default::default()
                        },
                    );
                    for h in &self.hearts {
                        draw_circle(ox + h.pos.x * s, oy + h.pos.y * s, 4. * s, RED);
                    }
                    return;
                }
                Animation::Interaction
            }
        };
        let uv = atlas.frame(species, anim, self.anim_frame);
        draw_texture_ex(
            atlas.texture(),
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
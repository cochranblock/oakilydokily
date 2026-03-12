// Unlicense — cochranblock.org
// Contributors: mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
//! Pet entity: Wandering, Sleeping, Interacting. Proximity detection. Guinea Pig kiss + hearts.

use macroquad::prelude::*;
use crate::sprites::{TextureAtlas, Species, Animation};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PetState {
    Wandering,
    Sleeping,
    Interacting,
    Exodus,
}

pub struct Pet {
    pub species: Species,
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
    pub fn new(species: Species, pos: Vec2, _atlas: &TextureAtlas) -> Self {
        Pet {
            species,
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

    pub fn update(&mut self, dt: f32, _atlas: &TextureAtlas, mural_w: f32, _mural_h: f32) {
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
            PetState::Exodus => {
                self.pos += self.vel * dt;
            }
        }

        self.hearts.retain_mut(|h| {
            h.pos += h.vel * dt;
            h.vel.y -= 50. * dt;
            h.life -= dt;
            h.life > 0.
        });
    }

    pub fn draw(&self, atlas: &TextureAtlas, scale: f32, ox: f32, oy: f32) {
        if self.state == PetState::Exodus && self.pos.x < -50. {
            return;
        }
        let s = scale;
        let x = ox + self.pos.x * s;
        let y = oy + self.pos.y * s;
        let w = 32. * s;
        let h = 48. * s;
        let anim = match self.state {
            PetState::Wandering | PetState::Exodus => Animation::Walk,
            PetState::Sleeping => Animation::Sleeping,
            PetState::Interacting => {
                if self.species == Species::GuineaPig {
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
                        let hx = ox + h.pos.x * s;
                        let hy = oy + h.pos.y * s;
                        draw_circle(hx, hy, 4. * s, RED);
                    }
                    return;
                }
                Animation::Interaction
            }
        };
        let uv = atlas.frame(self.species, anim, self.anim_frame);
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

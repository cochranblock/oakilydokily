// Unlicense — cochranblock.org
// Contributors: mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
//! Pet entity: Wandering, Sleeping, Interacting. Proximity detection. Guinea Pig kiss + hearts.

use macroquad::prelude::*;
use crate::sprites::{TextureAtlas, Species, Animation};

/// t124=PetState.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PetState {
    Wandering,
    Sleeping,
    Interacting,
    /// Moving toward house exit (Doggy Door exodus)
    Exodus,
}

/// t123=Pet.
pub struct Pet {
    pub species: Species,
    pub pos: Vec2,
    pub vel: Vec2,
    pub state: PetState,
    pub anim_frame: u32,
    pub anim_timer: f32,
    /// Seconds in Interacting before returning to Wandering.
    pub interaction_timer: f32,
    pub hearts: Vec<HeartParticle>,
}

/// t125=HeartParticle.
pub(crate) struct HeartParticle {
    pos: Vec2,
    vel: Vec2,
    life: f32,
}

impl Pet {
    /// f133=Pet::new.
    pub fn new(species: Species, pos: Vec2, _atlas: &TextureAtlas) -> Self {
        Pet {
            species,
            pos,
            vel: Vec2::new(20., 0.),
            state: PetState::Wandering,
            anim_frame: 0,
            anim_timer: 0.,
            interaction_timer: 0.,
            hearts: vec![],
        }
    }

    /// f134=Pet::trigger_kiss. Guinea Pig hearts.
    pub fn trigger_kiss(&mut self) {
        self.hearts.push(HeartParticle {
            pos: self.pos + vec2(0., -20.),
            vel: vec2(0., -30.),
            life: 1.,
        });
    }

    /// f135=Pet::enter_exodus. Doggy Door → move left.
    pub fn enter_exodus(&mut self) {
        self.state = PetState::Exodus;
        self.vel = vec2(-80., 0.); // move left toward house
    }

    /// f136=Pet::update. Anim timer, state logic, hearts.
    pub fn update(&mut self, dt: f32, _atlas: &TextureAtlas) {
        self.anim_timer += dt;
        if self.anim_timer > 0.15 {
            self.anim_timer = 0.;
            self.anim_frame = (self.anim_frame + 1) % 4;
        }

        match self.state {
            PetState::Wandering => {
                self.pos += self.vel * dt;
                let w = macroquad::prelude::screen_width();
                if self.pos.x < 32. || self.pos.x > w - 32. {
                    self.vel.x = -self.vel.x;
                }
            }
            PetState::Interacting => {
                self.interaction_timer += dt;
                if self.interaction_timer > 2.0 {
                    self.interaction_timer = 0.;
                    self.state = PetState::Wandering;
                    // Alternate direction so pets separate naturally
                    let dir = if (self.pos.x * 0.1) as i32 % 2 == 0 { 1. } else { -1. };
                    self.vel = vec2(20. * dir, 0.);
                }
            }
            PetState::Sleeping => {}
            PetState::Exodus => {
                self.pos += self.vel * dt;
            }
        }

        // Update heart particles (Guinea Pig kiss)
        self.hearts.retain_mut(|h| {
            h.pos += h.vel * dt;
            h.vel.y -= 50. * dt;
            h.life -= dt;
            h.life > 0.
        });
    }

    /// f137=Pet::draw. Occlusion: Exodus off-screen skip.
    pub fn draw(&self, atlas: &TextureAtlas) {
        if self.state == PetState::Exodus && self.pos.x < -50. {
            return; // off-screen, don't draw
        }
        let anim = match self.state {
            PetState::Wandering | PetState::Exodus => Animation::Walk,
            PetState::Sleeping => Animation::Sleeping,
            PetState::Interacting => {
                if self.species == Species::GuineaPig {
                    let uv = atlas.kiss_frame(self.anim_frame);
                    draw_texture_ex(
                        atlas.texture(),
                        self.pos.x - 16.,
                        self.pos.y - 24.,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(32., 48.)),
                            source: Some(uv),
                            ..Default::default()
                        },
                    );
                    for h in &self.hearts {
                        draw_circle(h.pos.x, h.pos.y, 4., RED);
                    }
                    return;
                }
                Animation::Interaction
            }
        };
        let uv = atlas.frame(self.species, anim, self.anim_frame);
        draw_texture_ex(
            atlas.texture(),
            self.pos.x - 16.,
            self.pos.y - 24.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(32., 48.)),
                source: Some(uv),
                ..Default::default()
            },
        );
    }
}

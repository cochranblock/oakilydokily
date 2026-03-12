// Unlicense — cochranblock.org
//! Interactive 2D mural: claymation animals (preferred) or Cat/Dog/GuineaPig sprites.

#![allow(dead_code)]

use macroquad::prelude::*;

mod bridge;
mod landscape;
mod sprites;
mod pet;
mod scenes;

use bridge::{get_scroll_x, get_scroll_y};
use sprites::{ClaymationSheet, SpriteSheet, TextureAtlas, Species};
use pet::{Pet, PetKind, PetState};
use scenes::SceneState;

fn mural_layout(tex: &Texture2D, screen_w: f32, screen_h: f32) -> (f32, f32, f32, f32, f32) {
    let (mw, mh) = (tex.width(), tex.height());
    let fit = (screen_w / mw).min(screen_h / mh);
    let scale = if fit >= 1. { fit.floor().max(1.) } else { fit };
    let draw_w = mw * scale;
    let draw_h = mh * scale;
    let ox = (screen_w - draw_w) / 2.;
    let oy = (screen_h - draw_h) / 2.;
    (mw, mh, scale, ox, oy)
}

fn window_conf() -> Conf {
    Conf {
        window_title: "OakilyDokily Mural".to_owned(),
        window_resizable: true,
        platform: miniquad::conf::Platform {
            framebuffer_alpha: true,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let landscape = match landscape::load("/assets/background.png").await {
        Some(t) => Some(t),
        None => landscape::load("/assets/mural.png").await,
    };
    let claymation = ClaymationSheet::load().await;
    let sprite_sheet = SpriteSheet::load("/assets/pets_spritesheet.png").await;
    let atlas = TextureAtlas::from_sheet(&sprite_sheet);

    let (mw, mh) = landscape
        .as_ref()
        .map(|t| (t.width(), t.height()))
        .unwrap_or((1024., 558.));

    let mut pets: Vec<Pet> = if let Some(ref sheet) = claymation {
        let n = sheet.rows.min(12).max(1);
        (0..n)
            .map(|i| {
                let t = i as f32 / n as f32;
                Pet::claymation(
                    i,
                    vec2(mw * (0.15 + t * 0.7), mh * (0.48 + (i as f32 * 0.03) % 0.1)),
                )
            })
            .collect()
    } else {
        vec![
            Pet::sprite(Species::Cat, vec2(mw * 0.15, mh * 0.55), &atlas),
            Pet::sprite(Species::Dog, vec2(mw * 0.45, mh * 0.50), &atlas),
            Pet::sprite(Species::GuineaPig, vec2(mw * 0.70, mh * 0.52), &atlas),
        ]
    };

    let mut scene = SceneState::default();
    let mut last_scroll_x: f32 = 0.;
    let mut last_scroll_y: f32 = 0.;

    loop {
        let scroll_x = get_scroll_x();
        let scroll_y = get_scroll_y();
        let _mouse = bridge::get_mouse_pos();

        let was_triggered = scene.doggy_door_triggered;
        scene.update(scroll_x, scroll_y, last_scroll_x, last_scroll_y);
        if scene.doggy_door_triggered && !was_triggered {
            for p in &mut pets {
                p.enter_exodus();
            }
        }
        last_scroll_x = scroll_x;
        last_scroll_y = scroll_y;

        let viewport = Rect::new(0., 0., mw, mh);
        let visible: Vec<usize> = pets
            .iter()
            .enumerate()
            .filter(|(_, p)| {
                viewport.contains(p.pos) || (p.state == PetState::Exodus && p.pos.x >= -50.)
            })
            .map(|(i, _)| i)
            .collect();

        for &i in &visible {
            for &j in &visible {
                if i >= j {
                    continue;
                }
                if pets[i].same_kind(&pets[j]) && pets[i].pos.distance(pets[j].pos) < 30. {
                    pets[i].state = PetState::Interacting;
                    pets[i].interaction_timer = 0.;
                    pets[j].state = PetState::Interacting;
                    pets[j].interaction_timer = 0.;
                    if let PetKind::Sprite(Species::GuineaPig) = pets[i].kind {
                        pets[i].trigger_kiss();
                        pets[j].trigger_kiss();
                    }
                }
            }
        }

        let dt = get_frame_time();
        for &i in &visible {
            pets[i].update(dt, mw, mh);
        }

        let (screen_w, screen_h) = (screen_width(), screen_height());
        clear_background(Color::from_rgba(0x1a, 0x1a, 0x2e, 255));

        if let Some(ref tex) = landscape {
            let (_, _, scale, ox, oy) = mural_layout(tex, screen_w, screen_h);
            let draw_w = tex.width() * scale;
            let draw_h = tex.height() * scale;
            draw_texture_ex(
                tex,
                ox,
                oy,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(draw_w, draw_h)),
                    ..Default::default()
                },
            );
            scene.draw();
            if let Some(ref sheet) = claymation {
                for &i in &visible {
                    pets[i].draw_claymation(sheet, scale, ox, oy);
                }
            } else {
                for &i in &visible {
                    pets[i].draw_sprite(&atlas, scale, ox, oy);
                }
            }
        } else {
            clear_background(Color::from_rgba(0xe8, 0xee, 0xf2, 255));
            scene.draw();
            if let Some(ref sheet) = claymation {
                for &i in &visible {
                    pets[i].draw_claymation(sheet, 1., 0., 0.);
                }
            } else {
                for &i in &visible {
                    pets[i].draw_sprite(&atlas, 1., 0., 0.);
                }
            }
        }

        next_frame().await;
    }
}

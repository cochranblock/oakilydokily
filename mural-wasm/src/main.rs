// Unlicense — cochranblock.org
// Contributors: mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
//! Interactive 2D mural: claymation animals from mural, scroll-triggered scenes.
//! Build: cargo build --target wasm32-unknown-unknown -p mural-wasm --release

#![allow(dead_code)]

use macroquad::prelude::*;

mod bridge;
mod landscape;
mod sprites;
mod pet;
mod scenes;

use bridge::{get_scroll_x, get_scroll_y};
use sprites::ClaymationSheet;
use pet::{Pet, PetState};
use scenes::SceneState;

/// Aspect-preserving mural layout: integer scale when upscaling (crisp), fractional when downscaling.
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

/// f119=window_conf. Transparent canvas, resizable.
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
    let landscape = landscape::load("/assets/mural.png").await;
    let sheet = ClaymationSheet::load().await;

    let (mw, mh) = landscape
        .as_ref()
        .map(|t| (t.width(), t.height()))
        .unwrap_or((1024., 558.));

    // One pet per claymation animal, spread across grassy areas
    let num_animals = sheet.rows.max(1);
    let mut pets: Vec<Pet> = (0..num_animals)
        .map(|i| {
            let t = i as f32 / num_animals as f32;
            let x = mw * (0.15 + t * 0.7);
            let y = mh * (0.48 + (i as f32 * 0.03) % 0.1);
            Pet::new(i, vec2(x, y))
        })
        .collect();

    let mut scene = SceneState::default();
    let mut last_scroll_x: f32 = 0.;
    let mut last_scroll_y: f32 = 0.;

    loop {
        let scroll_x = get_scroll_x();
        let scroll_y = get_scroll_y();
        let _mouse_pos = bridge::get_mouse_pos();

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
        let visible_pets: Vec<usize> = pets
            .iter()
            .enumerate()
            .filter(|(_, p)| {
                viewport.contains(p.pos)
                    || (p.state == PetState::Exodus && p.pos.x >= -50.)
            })
            .map(|(i, _)| i)
            .collect();

        let dt = get_frame_time();
        for &i in &visible_pets {
            pets[i].update(dt, &sheet, mw, mh);
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
            for &i in &visible_pets {
                pets[i].draw(&sheet, scale, ox, oy);
            }
        } else {
            clear_background(Color::from_rgba(0xe8, 0xee, 0xf2, 255));
            scene.draw();
            for &i in &visible_pets {
                pets[i].draw(&sheet, 1., 0., 0.);
            }
        }

        next_frame().await;
    }
}

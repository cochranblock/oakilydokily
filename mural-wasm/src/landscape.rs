//! Landscape texture: mural.png background with crisp 8-bit pixels.

// Unlicense — cochranblock.org
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

use macroquad::prelude::*;
use macroquad::texture::FilterMode;

/// f120=landscape_load. Landscape background from mural.png. None if load fails.
pub async fn load(path: &str) -> Option<Texture2D> {
    match load_texture(path).await {
        Ok(t) => {
            t.set_filter(FilterMode::Nearest);
            Some(t)
        }
        Err(_) => None,
    }
}
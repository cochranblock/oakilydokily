// Unlicense — cochranblock.org
// Contributors: mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
//! Scroll-triggered scenes: Cozy Nook, Winter Tubing, Doggy Door footer.

use macroquad::prelude::*;

/// t126=SceneState. s73=cozy_nook_visible, s74=cozy_nook_x, s75=tubing_visible, s76=tubing_y, s77=tubing_vel, s78=doggy_door_triggered.
#[derive(Default)]
pub struct SceneState {
    pub cozy_nook_visible: bool,
    pub cozy_nook_x: f32,
    pub tubing_visible: bool,
    pub tubing_y: f32,
    pub tubing_vel: f32,
    pub doggy_door_triggered: bool,
}

impl SceneState {
    /// f138=SceneState::update. Cozy Nook, Tubing, Doggy Door thresholds.
    pub fn update(&mut self, scroll_x: f32, scroll_y: f32, _last_x: f32, _last_y: f32) {
        // Cozy Nook: slide in at scroll_x threshold
        if scroll_x > 100. {
            self.cozy_nook_visible = true;
            self.cozy_nook_x = (self.cozy_nook_x + (200. - self.cozy_nook_x) * 0.1).min(200.);
        } else {
            self.cozy_nook_x *= 0.9;
        }

        // Winter Tubing: inner tube with momentum at scroll_y
        if scroll_y > 300. {
            self.tubing_visible = true;
            self.tubing_vel += 2.;
            self.tubing_y += self.tubing_vel * 0.016;
        }

        // Doggy Door: at footer (scroll_y), trigger exodus for all active pets
        if scroll_y > 800. {
            self.doggy_door_triggered = true;
        }
    }

    /// f139=SceneState::draw. Subtle visual cues — no crude rectangles.
    pub fn draw(&self) {
        // Scene draw is intentionally minimal — the mural artwork provides the visuals.
        // State is used to drive pet behavior (exodus etc.), not to draw shapes.
    }
}

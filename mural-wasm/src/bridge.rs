//! JS/Rust bridge: scrollY, mouse_pos passed from JS into the engine.
//! Uses #[no_mangle] extern "C" for miniquad gl.js compatibility (no wasm-bindgen).

// Unlicense — cochranblock.org
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

use std::cell::{Cell, RefCell};

thread_local! {
    static SCROLL_X: Cell<f32> = const { Cell::new(0.) };
    static SCROLL_Y: Cell<f32> = const { Cell::new(0.) };
    static MOUSE_X: Cell<f32> = const { Cell::new(0.) };
    static MOUSE_Y: Cell<f32> = const { Cell::new(0.) };
    /// Pending forged sprite data: (rgba_bytes, sprite_count, cell_w, cell_h)
    static FORGED_PENDING: RefCell<Option<(Vec<u8>, u32, u32, u32)>> = const { RefCell::new(None) };
}

/// f127=mural_set_scroll_x. Exported for gl.js; called from JS as wasm_exports.mural_set_scroll_x(x).
#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn mural_set_scroll_x(x: f32) {
    SCROLL_X.with(|c| c.set(x));
}

/// f128=mural_set_scroll_y.
#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn mural_set_scroll_y(y: f32) {
    SCROLL_Y.with(|c| c.set(y));
}

#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn mural_set_mouse(x: f32, y: f32) {
    MOUSE_X.with(|c| c.set(x));
    MOUSE_Y.with(|c| c.set(y));
}


pub fn get_scroll_x() -> f32 {
    SCROLL_X.with(|c| c.get())
}

/// f131=get_scroll_y.
pub fn get_scroll_y() -> f32 {
    SCROLL_Y.with(|c| c.get())
}

/// f132=get_mouse_pos.
pub fn get_mouse_pos() -> (f32, f32) {
    (MOUSE_X.with(|c| c.get()), MOUSE_Y.with(|c| c.get()))
}

/// Allocate WASM memory for JS to write sprite RGBA data into.
/// Returns a pointer JS can write to via new Uint8Array(wasm_memory.buffer, ptr, len).
#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn mural_alloc(len: u32) -> *mut u8 {
    let mut buf = Vec::with_capacity(len as usize);
    buf.resize(len as usize, 0u8);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

/// Load forged sprites from RGBA bytes written by JS into WASM memory.
/// ptr/len = RGBA pixel data for the entire sheet (grid of sprites).
/// count = number of sprites, cell_w/cell_h = dimensions of each sprite cell.
/// The sheet is laid out as a single row: count columns × 1 row.
#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn mural_load_sprites(ptr: *const u8, len: u32, count: u32, cell_w: u32, cell_h: u32) {
    let bytes = unsafe { std::slice::from_raw_parts(ptr, len as usize) }.to_vec();
    FORGED_PENDING.with(|f| {
        *f.borrow_mut() = Some((bytes, count, cell_w, cell_h));
    });
}

/// Take pending forged sprite data (consumed once by main loop).
pub fn take_forged_pending() -> Option<(Vec<u8>, u32, u32, u32)> {
    FORGED_PENDING.with(|f| f.borrow_mut().take())
}
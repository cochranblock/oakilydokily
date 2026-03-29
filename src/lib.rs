#![allow(non_camel_case_types, non_snake_case, dead_code)]

// Unlicense — cochranblock.org
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

pub mod d1_auth;
#[cfg(feature = "tests")]
pub mod mock_google;
#[cfg(feature = "tests")]
pub mod screenshot;
#[cfg(feature = "tests")]
pub mod tests;
#[cfg(all(target_os = "android", feature = "android"))]
pub mod android;
#[cfg(target_os = "ios")]
pub mod ios;
pub mod waiver;
pub mod web;

use sqlx::sqlite::SqlitePool;

/// t0 = AppState. s0=waiver pool s1=optional D1 auth (when OD_AUTH_D1=1) s2=forge cache
#[derive(Clone)]
pub struct t0 {
    pub s0: SqlitePool,
    pub s1: Option<d1_auth::t78>,
    pub s2: web::forge::ForgeCache,
}
pub use t0 as AppState;
#![allow(non_camel_case_types, non_snake_case)]

// Unlicense — cochranblock.org
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

use axum::{routing::{get, post}, Router};
use axum::response::Redirect;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

use super::{assets, auth, forge, govdocs, pages, waiver};
use crate::AppState;

/// f1 = router. Why: Single entry for all OD routes; state shared via Arc.
pub fn f1(state: AppState) -> Router {
    let state = std::sync::Arc::new(state);
    Router::new()
        .route("/", get(pages::home))
        .route("/about", get(pages::about))
        .route("/contact", get(pages::contact))
        .route("/waiver", get(waiver::f74).post(waiver::f75))
        .route("/waiver/confirmed", get(waiver::confirmed))
        .route("/auth/google", get(auth::f82))
        .route("/auth/google/callback", get(auth::f83))
        .route("/auth/facebook", get(auth::f98))
        .route("/auth/facebook/callback", get(auth::f99))
        .route("/auth/apple", get(auth::f91))
        .route("/auth/apple/callback", get(auth::f92))
        .route("/auth/login", get(auth::f100).post(auth::f101))
        .route("/auth/signup", get(auth::f102).post(auth::f103))
        .route("/auth/logout", get(auth::f84))
        .route("/api/forge", post(forge::handler))
        .route("/govdocs", get(govdocs::index))
        .route("/govdocs/sbom", get(govdocs::sbom))
        .route("/govdocs/security", get(govdocs::security))
        .route("/govdocs/ssdf", get(govdocs::ssdf))
        .route("/govdocs/supply-chain", get(govdocs::supply_chain))
        .route("/govdocs/privacy", get(govdocs::privacy))
        .route("/govdocs/fips", get(govdocs::fips))
        .route("/govdocs/fedramp", get(govdocs::fedramp))
        .route("/govdocs/cmmc", get(govdocs::cmmc))
        .route("/govdocs/itar-ear", get(govdocs::itar_ear))
        .route("/govdocs/accessibility", get(govdocs::accessibility))
        .route("/govdocs/federal-use-cases", get(govdocs::federal_use_cases))
        .route("/govdocs/supply-chain-audit", get(govdocs::supply_chain_audit))
        .route("/health", get(pages::health))
        .route("/favicon.ico", get(|| async { Redirect::permanent("/assets/favicon.svg") }))
        .route("/sitemap.xml", get(pages::sitemap))
        .route("/assets/*path", get(assets::serve))
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
pub use f1 as router;
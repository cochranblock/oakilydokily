<!-- Unlicense — cochranblock.org -->

# Supply Chain Security Audit

> Per EO 14028. Last audit: 2026-03-30.

## cargo audit — Known Vulnerabilities

| Crate | Advisory | Severity | In Release? | Status |
|-------|----------|----------|-------------|--------|
| rustls-webpki 0.103.9 | RUSTSEC-2026-0049 — CRL distribution point logic error | - | Yes | **FIXED** → 0.103.10 |
| aws-lc-sys 0.38.0 | RUSTSEC-2026-0044 — X.509 name constraint bypass | - | No (test-only via exopack) | Accepted (test-only) |
| aws-lc-sys 0.38.0 | RUSTSEC-2026-0048 — CRL scope check error | High (7.4) | No (test-only) | Accepted (test-only) |
| idna 0.3.0 | RUSTSEC-2024-0421 — Punycode label validation bypass | - | Yes (via reqwest 0.11 → cookie_store) | Accepted (pinned by reqwest 0.11) |
| rsa 0.9.10 | RUSTSEC-2023-0071 — Marvin timing attack | Medium (5.9) | No (test-only via sqlx-mysql) | Accepted (not used, no fix available) |

### Unmaintained Crates (Warnings)

| Crate | In Release? | Notes |
|-------|-------------|-------|
| fxhash 0.2.1 | No (test-only via scraper) | Used only in test binary |
| paste 1.0.15 | No (test-only via rav1e → image → exopack) | Deep transitive, test-only |
| proc-macro-error 1.0.4 | Yes (via include-flate-codegen → rust-embed) | Compile-time only, no runtime impact |
| rustls-pemfile 1.0.4 | Yes (via reqwest 0.11) | Pinned by reqwest 0.11 |

## Duplicate Dependencies (Release Binary)

| Crate | Versions | Root Cause |
|-------|----------|------------|
| base64 | 0.21, 0.22 | oakilydokily uses 0.21, bcrypt/jsonwebtoken/sqlx use 0.22 |
| hyper | 0.14, 1.8 | reqwest 0.11 (hyper 0.14) vs axum 0.7 (hyper 1.x) |
| http | 0.2, 1.4 | Same as hyper split |
| http-body | 0.4, 1.0 | Same as hyper split |
| rustls | 0.21, 0.23 | reqwest 0.11 (rustls 0.21) vs sqlx (rustls 0.23) |
| getrandom | 0.2, 0.3, 0.4 | Different consumers at different versions |
| sync_wrapper | 0.1, 1.0 | reqwest 0.11 vs axum 0.7 |

**Fix path:** Upgrade reqwest from 0.11 to 0.12+ (unifies hyper at 1.x, rustls at 0.23, eliminates http 0.2). Blocked by approuter-client pinning reqwest 0.11.

## Cargo.lock

**PASS** — committed and pinned.

## Typosquatting Check

**PASS** — all dependency names verified against crates.io. No suspicious names found.

## Yanked Versions

**PASS** — no yanked crates in Cargo.lock.

## Deep Code Review

### Unsafe Usage (Manual Audit)

| Crate | Unsafe? | Notes |
|-------|---------|-------|
| ring | Yes (crypto assembly) | Expected — hardware-accelerated crypto |
| libsqlite3-sys | Yes (C FFI) | Expected — SQLite C binding |
| tokio | Yes (runtime internals) | Expected — async runtime |
| axum | No | Safe Rust only |
| reqwest | No | Safe Rust only |
| sqlx | Minimal (FFI boundary) | Expected for database drivers |
| serde/serde_json | No | Safe Rust only |
| bcrypt | Minimal (blowfish internals) | Expected for crypto |

No unexpected `unsafe` in application-level deps.

### Network-Calling Deps

| Crate | Expected? | Purpose |
|-------|-----------|---------|
| reqwest | Yes | HTTP client for OAuth, Turnstile, email APIs |
| sqlx | No network | SQLite only (no network driver used) |
| tracing | No network | Logs to stdout only |
| rust-embed | No network | Compile-time asset embedding |

No unexpected network calls. No telemetry. No analytics beacons.

### Environment Variable / File Access

| Crate | Access | Expected? |
|-------|--------|-----------|
| dotenvy | Reads .env file | Yes (explicit) |
| dirs | Reads platform data dir | Yes (data directory fallback) |
| All others | Only via oakilydokily code | Yes |

No deps read secrets or env vars without going through application code.

### Crypto Review

| Primitive | Library | Constant-Time? | NIST Approved? |
|-----------|---------|---------------|----------------|
| SHA-256 | sha2 (RustCrypto) | Yes | Yes |
| HMAC-SHA256 | hmac (RustCrypto) | Yes | Yes |
| bcrypt | bcrypt crate | Yes (blowfish constant-time) | No (not FIPS) |
| RSA-2048 | ring | Yes | Yes |
| TLS 1.2/1.3 | rustls + ring | Yes | Yes |

No custom crypto. No rolled primitives. All established algorithms.

## Dead Files Removed

| File | Size | Reason |
|------|------|--------|
| models/silueta.onnx | 42 MB | ONNX model not referenced by any code |
| data/waivers.sqlite | runtime | SQLite database should not be in git |

## Recommended Actions

| Priority | Action | Status |
|----------|--------|--------|
| 1 | Upgrade rustls-webpki | **Done** (0.103.9 → 0.103.10) |
| 2 | Remove silueta.onnx from git | **Done** |
| 3 | Gitignore data/ | **Done** |
| 4 | Upgrade reqwest 0.11 → 0.12 | Blocked (approuter-client) |
| 5 | Replace base64 0.21 with 0.22 | Low priority (minor dup) |

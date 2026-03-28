<!-- Unlicense — cochranblock.org -->

# Software Bill of Materials (SBOM)

> Per EO 14028, Section 4. Generated from `cargo tree --depth 1`.
> Last updated: 2026-03-27

## Runtime Dependencies (--features approuter)

| Crate | Version | License | Purpose |
|-------|---------|---------|---------|
| approuter-client | 0.2.0 | Unlicense | Reverse proxy registration |
| axum | 0.7.9 | MIT | HTTP framework |
| axum-extra | 0.9.6 | MIT | Cookie extraction |
| base64 | 0.21.7 | MIT/Apache-2.0 | Base64 encoding for Gmail API |
| bcrypt | 0.17.1 | MIT | Password hashing |
| chrono | 0.4.44 | MIT/Apache-2.0 | Date/time handling |
| dirs | 5.0.1 | MIT/Apache-2.0 | Platform data directory |
| dotenvy | 0.15.7 | MIT | .env file loading |
| flate2 | 1.1.9 | MIT/Apache-2.0 | Gzip compression for waiver archive |
| hmac | 0.12.1 | MIT/Apache-2.0 | HMAC-SHA256 session signing |
| jsonwebtoken | 9.3.1 | MIT | JWT for Google service account auth |
| mime_guess | 2.0.5 | MIT | MIME type detection for static assets |
| rand | 0.8.5 | MIT/Apache-2.0 | Random number generation |
| reqwest | 0.11.27 | MIT/Apache-2.0 | HTTP client (Turnstile, email APIs) |
| rust-embed | 8.11.0 | MIT | Static asset embedding in binary |
| serde | 1.0.228 | MIT/Apache-2.0 | Serialization framework |
| serde_json | 1.0.149 | MIT/Apache-2.0 | JSON parsing |
| sha2 | 0.10.9 | MIT/Apache-2.0 | SHA-256 for terms versioning, email hashing |
| sqlx | 0.8.6 | MIT/Apache-2.0 | SQLite async driver |
| time | 0.3.47 | MIT/Apache-2.0 | Cookie expiry handling |
| tokio | 1.49.0 | MIT | Async runtime |
| tower-http | 0.5.2 | MIT | HTTP middleware (compression, tracing) |
| tracing | 0.1.44 | MIT | Structured logging |
| tracing-subscriber | 0.3.22 | MIT | Log output formatting |
| urlencoding | 2.1.3 | MIT | URL encoding for redirects |
| uuid | 1.21.0 | MIT/Apache-2.0 | UUID v4 for waiver reference IDs |

## Dev Dependencies (not in release binary)

| Crate | Version | License | Purpose |
|-------|---------|---------|---------|
| proptest | 1.10.0 | MIT/Apache-2.0 | Property-based testing |

## Test-Only Dependencies (--features tests, not in release)

| Crate | Version | License | Purpose |
|-------|---------|---------|---------|
| exopack | 0.1.0 | Proprietary | Triple-sims test harness |
| colored | 2.2.0 | MPL-2.0 | Terminal color output |
| scraper | 0.19.1 | MIT | HTML parsing for test assertions |

## TLS Stack

Release binary uses **rustls** (pure Rust TLS). No OpenSSL. No native-tls.

## Embedded C Code

- **libsqlite3-sys** (bundled SQLite 3.x) — compiled from source via sqlx

## Regenerate

```bash
cargo tree -p oakilydokily --features approuter --depth 1
```

<!-- Unlicense — cochranblock.org -->

# Security Posture

> Last updated: 2026-03-27

## Cryptographic Primitives

| Primitive | Implementation | Purpose |
|-----------|---------------|---------|
| SHA-256 | sha2 crate (RustCrypto) | Terms versioning hash, email hashing for D1 sharding |
| HMAC-SHA256 | hmac crate (RustCrypto) | Session cookie signing and verification |
| bcrypt (cost 12) | bcrypt crate | Password hashing for manual auth users |
| RSA-256 (RS256) | jsonwebtoken crate + ring | JWT signing for Google service account auth |
| TLS 1.2/1.3 | rustls (pure Rust) | All outbound HTTPS (OAuth, email APIs, Turnstile) |

## No Plaintext Secrets

- All secrets loaded from environment variables (never hardcoded)
- `.env` is in `.gitignore`
- No secrets in source code, comments, or docs
- Session tokens are HMAC-signed, not stored server-side
- Password hashes stored via bcrypt (never plaintext)

## Input Validation

| Boundary | Validation |
|----------|-----------|
| Waiver name | Non-empty, max 200 chars, HTML-escaped on output |
| Waiver email | Non-empty, max 254 chars, must contain @ (not at start/end) |
| Waiver signature | Non-empty, max 200 chars |
| Consent checkboxes | Must be explicitly "1" |
| OAuth redirect | Must start with `/`, no protocol injection (`//`, `://` blocked) |
| Session cookie | HMAC-SHA256 verified, 7-day expiry, HttpOnly, SameSite=Lax |
| Turnstile token | Server-side verification with Cloudflare API (when configured) |

## Attack Surface

| Surface | Mitigation |
|---------|-----------|
| SQL injection | sqlx parameterized queries — no string concatenation in SQL |
| XSS | All user input HTML-escaped (f71, f79, html_escape_attr) before rendering |
| CSRF | Cloudflare Turnstile (optional). Auth required for state-changing operations. |
| Session hijacking | HMAC-signed cookies, HttpOnly flag, Secure flag on HTTPS |
| Path traversal | Static assets served via rust-embed (compiled in, not filesystem) |
| DoS | Single-threaded SQLite with WAL. No unbounded allocations. |
| Dependency confusion | Cargo.lock pinned. Only crates.io + explicit GitHub repos. |

## Known Limitations

- No CSRF token on waiver form (mitigated by Turnstile when configured)
- No rate limiting on form submissions
- Email confirmation is fire-and-forget (no delivery guarantee)
- SESSION_SECRET must be 32+ chars; if shorter, auth silently fails (logged as warning)

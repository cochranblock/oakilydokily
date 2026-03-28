<!-- Unlicense — cochranblock.org -->

# NIST SP 800-218 — Secure Software Development Framework (SSDF)

> Mapping of oakilydokily practices to SSDF tasks.
> Last updated: 2026-03-27

## PS: Prepare the Organization

| Practice | Implementation |
|----------|---------------|
| PS.1 — Define security requirements | Waiver system requires ESIGN compliance: terms versioning (SHA-256), IP/UA logging, 7-year retention, consent capture. Documented in PROOF_OF_ARTIFACTS.md. |
| PS.2 — Implement roles and responsibilities | Single-developer project. All code reviewed by human pilot + AI pair (documented in TIMELINE_OF_INVENTION.md). CODEOWNERS file present. |
| PS.3 — Implement supporting toolchains | Rust compiler (memory-safe language), cargo clippy (static analysis), triple-sims test gate (3-pass verification), exopack test harness. |

## PW: Protect the Software

| Practice | Implementation |
|----------|---------------|
| PW.1 — Design software to meet security requirements | No `unsafe` blocks. Input validation at all boundaries (waiver form: name length, email format, signature presence). HMAC-SHA256 session cookies. bcrypt password hashing. |
| PW.2 — Review design for compliance | ESIGN Act compliance verified: electronic consent checkbox, typed signature field, terms hash versioning, IP/UA audit trail, gzip archive with 7-year retention. |
| PW.4 — Reuse existing well-secured components | All crypto via established crates: sha2, hmac, bcrypt, jsonwebtoken. TLS via rustls (audited). No custom crypto. |
| PW.5 — Create source code following secure coding practices | Rust memory safety. No SQL injection (sqlx parameterized queries). HTML escaping on all user input (f71, f79, html_escape_attr). No `eval()`. No `unsafe`. |
| PW.6 — Configure the build process | `Cargo.lock` pinned. Release profile: strip=true, lto=true, codegen-units=1. No debug symbols in production. |
| PW.7 — Review and test code for vulnerabilities | cargo clippy on every build. Triple-sims gate (25 checks x 3 passes). Property-based testing via proptest. |
| PW.9 — Configure software securely by default | Server binds 0.0.0.0:3000. Session cookies: HttpOnly, SameSite=Lax, Secure (when HTTPS). No default credentials. |

## RV: Respond to Vulnerabilities

| Practice | Implementation |
|----------|---------------|
| RV.1 — Identify and confirm vulnerabilities | `cargo audit` available for dependency vulnerability scanning. Cargo.lock enables reproducible builds for patch verification. |
| RV.2 — Assess and prioritize vulnerabilities | Single-binary architecture limits attack surface. No external services except optional Cloudflare D1 and email APIs. |
| RV.3 — Remediate vulnerabilities | Dependency updates via `cargo update`. Single-binary deployment: new binary replaces old, restart via systemd. Zero-downtime path available via approuter. |

## PO: Protect Operations

| Practice | Implementation |
|----------|---------------|
| PO.1 — Provision and de-provision securely | Single binary deployment. No installer. No registry entries. Remove binary + data directory to fully de-provision. |
| PO.2 — Collect operational data | Structured logging via tracing crate. Optional GA4 analytics. Waiver audit trail (IP, User-Agent, timestamp, terms hash). |
| PO.3 — Respond to operational events | Health endpoint at /health for monitoring. tracing output to stdout for log aggregation. Archive prune runs at startup. |

<!-- Unlicense — cochranblock.org -->

# Proof of Artifacts

*Concrete evidence that this project works, ships, and is real.*

> A veterinary professional services site with multi-auth, ESIGN-compliant waivers, federal compliance docs, and AI sprite generation. Part of [CochranBlock](https://cochranblock.org).

## Architecture

```mermaid
flowchart TD
    User[User] --> Axum[Axum Server :3000]
    Axum --> Pages[Pages: Home / About / Contact / Waiver]
    Axum --> Auth[OAuth: Google / Facebook / Apple / Manual]
    Axum --> Mural[Static Mural — CSS animated hero image]
    Axum --> Forge["/api/forge → pixel-forge SSH"]
    Forge --> GPU["IRONHIVE GPU node — kova C2"]
    Axum --> Waiver[Waiver System]
    Waiver --> SQLite[(SQLite WAL)]
    Auth --> D1[Cloudflare D1 sharded auth — optional]
    Auth --> SQLite
```

## Build Output

| Metric | Value |
|--------|-------|
| Release binary | 8.7 MB (down from 42 MB — strip, LTO, zero JS) |
| Lines of Rust | 2,748 (backend) + 1,192 (mural-wasm + mural-claymation, archived) |
| JavaScript | 0 lines in release binary |
| Direct dependencies | 28 (release with approuter) |
| Android AAB | 4.6 MB (Pocket Server scaffold) |
| Platforms | 12 targets (macOS, Linux, Android, iOS, Windows, FreeBSD, RISC-V, POWER, PWA) |
| Auth providers | 4 (Google, Facebook, Apple, manual email/password) |
| Waiver compliance | ESIGN-compliant, SHA256 terms versioning, typed signature, 7-year retention |
| Database | SQLite with WAL mode (production durability) |
| Federal compliance docs | 12 (SBOM, SSDF, FIPS, CMMC, supply chain audit, etc.) |
| Hot reload | Zero downtime deploy via SO_REUSEPORT + PID lockfile |
| Rate limiting | 10 requests/IP/60s on auth endpoints |

## Key Artifacts

| Artifact | Description |
|----------|-------------|
| Static Mural | Server-rendered mural image with CSS gradient overlay (zero JS) |
| Waiver System | Full audit trail: IP, User-Agent, terms hash, consent checkbox, signature. SQLite + gzip archive with auto-prune |
| Multi-Auth Stack | Google/Facebook/Apple OAuth + manual signup. HMAC-SHA256 signed session cookies |
| D1 Sharded Auth | Optional Cloudflare D1 backend — active when `OD_AUTH_D1=1` + D1 env vars set |
| Pixel Forge | /api/forge — SSH dispatches to [pixel-forge](https://github.com/cochranblock/pixel-forge) on [kova](https://github.com/cochranblock/kova) IRONHIVE GPU node |
| Rate Limiting | IP-based sliding window (10/60s) on login and signup endpoints |
| Async I/O | All external HTTP calls (OAuth, email, Turnstile) use async reqwest |

## QA Results (2026-04-02)

| Pass | Checks | Result |
|------|--------|--------|
| Triple Sims 1/3 | 25 checks (21 pass, 4 skip — auth-gated) | OK |
| Triple Sims 2/3 | 25 checks | OK |
| Triple Sims 3/3 | 25 checks | OK |
| Clippy (release) | 0 warnings (`-D warnings`) | Clean |
| Clippy (tests) | 0 warnings (`-D warnings`) | Clean |
| Route coverage | 19 routes tested, 0 unexpected 404s | Pass |
| Binary size | 8.7 MB release (strip + LTO + zero JS) | Target met |
| Supply chain audit | 1 CVE fixed, 0 in release binary | Pass |
| dead_code allows | 0 (removed from all 9 files) | Clean |

## How to Verify

```bash
cargo build --release -p oakilydokily --features approuter
ls -lh target/release/oakilydokily  # should be ~8.7 MB
cargo run -p oakilydokily --bin oakilydokily-test --features tests
# Open localhost:3000 — static mural hero, CSS animated
# Visit /waiver — complete ESIGN flow with typed signature
# Visit /about — print-ready resume
# Visit /govdocs — federal compliance docs
```

## Sibling Repos

| Repo | Role |
|------|------|
| [approuter](https://github.com/cochranblock/approuter) | Reverse proxy, production hosting |
| [pixel-forge](https://github.com/cochranblock/pixel-forge) | AI sprite generation (forge backend) |
| [kova](https://github.com/cochranblock/kova) | Augment engine, IRONHIVE GPU cluster |
| [exopack](https://github.com/cochranblock/exopack) | Test framework (triple sims, screenshots) |
| [cochranblock](https://github.com/cochranblock/cochranblock) | Main site |
| [pocket-server](https://github.com/cochranblock/pocket-server) | Android pocket server scaffold |

---

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. All source under the [Unlicense](LICENSE).*

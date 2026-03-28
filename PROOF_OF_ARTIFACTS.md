<!-- Unlicense — cochranblock.org -->

# Proof of Artifacts

*Concrete evidence that this project works, ships, and is real.*

> A veterinary professional services site with an interactive mural, multi-auth, and ESIGN-compliant waivers.

## Architecture

```mermaid
flowchart TD
    User[User] --> Axum[Axum Server :3000]
    Axum --> Pages[Pages: Home / About / Contact / Waiver]
    Axum --> Auth[OAuth: Google / Facebook / Apple / Manual]
    Axum --> WASM[mural-wasm Canvas]
    WASM --> Pets[Pet Entities: wander / sleep / interact]
    WASM --> Scenes[Scroll Scenes: Cozy Nook / Tubing / Doggy Door]
    WASM --> Forge[/api/forge → Pixel Forge SSH]
    Axum --> Waiver[Waiver System]
    Waiver --> SQLite[(SQLite WAL)]
    Auth --> SQLite
```

## Build Output

| Metric | Value |
|--------|-------|
| Release binary | 9.1 MB (down from 42 MB after optimization) |
| Lines of Rust | ~3,200 (2,443 backend + 755 mural-wasm) |
| Direct dependencies | 26 (release), 27 (dev) |
| WASM binary | 878 KB (mural-wasm.wasm) |
| Auth providers | 4 (Google, Facebook, Apple, manual email/password) |
| Waiver compliance | ESIGN-compliant, SHA256 terms versioning, typed signature, 7-year retention |
| Pet types | 3 (claymation, forged AI sprites, CC0 pixel art fallback) |
| Scroll scenes | 3 (Cozy Nook, Winter Tubing, Doggy Door) |
| Database | SQLite with WAL mode (production durability) |
| Federal compliance docs | 11 (SBOM, SSDF, FIPS, CMMC, etc.) |

## Key Artifacts

| Artifact | Description |
|----------|-------------|
| Interactive Mural | Macroquad 2D engine targeting wasm32 — pets wander, interact, respond to scroll |
| Claymation Pipeline | Pure Rust: segment animals from mural → inpaint background → pixelate → rotate → composite |
| Scroll-Triggered Scenes | Zero clicks required — pure scroll-driven storytelling with momentum physics |
| Waiver System | Full audit trail: IP, User-Agent, terms hash, consent checkbox, signature. SQLite + gzip archive with auto-prune |
| Multi-Auth Stack | Google/Facebook/Apple OAuth + manual signup. HMAC-SHA256 signed session cookies |
| D1 Sharded Auth | Optional Cloudflare D1 backend — email-hash sharding across N databases |
| Pixel Forge Integration | /api/forge SSH-dispatches to GPU node for AI sprite generation, LRU cached |

## QA Results (2026-03-27)

| Pass | Checks | Result |
|------|--------|--------|
| Triple Sims 1/3 | 25 checks (21 pass, 4 skip — auth-gated) | OK |
| Triple Sims 2/3 | 25 checks | OK |
| Triple Sims 3/3 | 25 checks | OK |
| Clippy (release) | 0 warnings | Clean |
| Clippy (tests) | 0 warnings | Clean |
| Route coverage | 19 routes tested, 0 unexpected 404s | Pass |
| Binary size | 9.1 MB release (strip + LTO) | Target met |

## How to Verify

```bash
cargo build --release -p oakilydokily --features approuter
ls -lh target/release/oakilydokily  # should be ~9 MB
cargo run -p oakilydokily --bin oakilydokily-test --features tests
# Open localhost:3000 — scroll the mural, watch pets interact
# Visit /waiver — complete ESIGN flow with typed signature
# Visit /about — print-ready resume
```

---

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. All source under the Unlicense.*

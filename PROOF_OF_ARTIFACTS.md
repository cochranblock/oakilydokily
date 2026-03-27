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
| Lines of Rust | ~3,200 (backend + mural-wasm) |
| WASM binary | 878 KB (mural-wasm.wasm) |
| Auth providers | 4 (Google, Facebook, Apple, manual email/password) |
| Waiver compliance | ESIGN-compliant, SHA256 terms versioning, 7-year retention |
| Pet types | 3 (claymation, forged AI sprites, CC0 pixel art fallback) |
| Scroll scenes | 3 (Cozy Nook, Winter Tubing, Doggy Door) |
| Database | SQLite with WAL mode (production durability) |

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

## How to Verify

```bash
cargo build --release -p oakilydokily --features approuter
# Open localhost:3000 — scroll the mural, watch pets interact
# Visit /waiver — complete ESIGN flow
# Visit /about — print-ready resume
```

---

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. All source under the Unlicense.*

<!-- Unlicense — cochranblock.org -->
<!-- Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3 -->

> **It's not the Mech — it's the pilot.**
>
> This repo is part of [CochranBlock](https://cochranblock.org) — 14 Unlicense Rust repositories that power an entire company on a **single <10MB binary**, a laptop, and a **$10/month** Cloudflare tunnel. No AWS. No Kubernetes. No six-figure DevOps team. Zero cloud.
>
> **[cochranblock.org](https://cochranblock.org)** is a live demo of this architecture. You're welcome to read every line of source code — it's all public domain.
>
> Every repo ships with **[Proof of Artifacts](PROOF_OF_ARTIFACTS.md)** (wire diagrams, screenshots, and build output proving the work is real) and a **[Timeline of Invention](TIMELINE_OF_INVENTION.md)** (dated commit-level record of what was built, when, and why — proving human-piloted AI development, not generated spaghetti).
>
> **Looking to cut your server bill by 90%?** → [Zero-Cloud Tech Intake Form](https://cochranblock.org/deploy)

---

<p align="center">
  <img src="https://raw.githubusercontent.com/cochranblock/oakilydokily/main/assets/favicon.svg" alt="OakilyDokily" width="64">
</p>

# oakilydokily

Veterinary professional services site with interactive mural, multi-auth, ESIGN-compliant waivers, and pixel forge sprite generation. Rust Axum server + mural-wasm (Macroquad WASM).

## Architecture

```mermaid
flowchart TD
    User[User] --> Axum[Axum Server :3000]
    Axum --> Pages[Pages: Home / About / Contact / Waiver]
    Axum --> Auth[OAuth: Google / Facebook / Apple / Manual]
    Axum --> WASM[mural-wasm Canvas]
    WASM --> Pets[Pet Entities: wander / sleep / interact]
    WASM --> Scenes[Scroll Scenes: Cozy Nook / Tubing / Doggy Door]
    Axum --> Forge[/api/forge → Pixel Forge SSH to GPU node]
    Axum --> Waiver[Waiver System]
    Waiver --> SQLite[(SQLite WAL)]
    Auth --> D1[Cloudflare D1 sharded auth — optional]
    Auth --> SQLite
```

## Modules

| Module | Purpose |
|--------|---------|
| `src/main.rs` | Entry point, approuter registration, server bind |
| `src/waiver.rs` | SQLite waiver persistence, gzip archive, user CRUD |
| `src/d1_auth.rs` | Sharded Cloudflare D1 auth storage (optional) |
| `src/web/router.rs` | All routes: pages, auth, waiver, forge, assets |
| `src/web/auth.rs` | Google/Facebook/Apple OAuth + manual email/password |
| `src/web/pages.rs` | Home, about, contact, sitemap |
| `src/web/waiver.rs` | Waiver form GET/POST, Turnstile verification |
| `src/web/email.rs` | Gmail API + Resend fallback for waiver confirmation |
| `src/web/forge.rs` | /api/forge — SSH to GPU node for AI sprite generation |
| `src/web/head.rs` | GA4, nav, shared HTML head helpers |
| `src/web/assets.rs` | Static asset serving via rust-embed |
| `mural-wasm/` | Macroquad 2D mural targeting wasm32 |

## Run

```bash
cargo run -p oakilydokily --features approuter
```

Build release:

```bash
cargo build --release -p oakilydokily --features approuter
```

## mural-wasm

See [mural-wasm/README.md](mural-wasm/README.md) for the interactive mural crate.
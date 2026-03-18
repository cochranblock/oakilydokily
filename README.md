<!-- Unlicense — cochranblock.org -->
<!-- Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3 -->

<p align="center">
  <img src="https://raw.githubusercontent.com/cochranblock/oakilydokily/main/assets/favicon.svg" alt="OakilyDokily" width="64">
</p>

# oakilydokily

Hero site with interactive mural. Rust Axum server + mural-wasm (Macroquad).

## Proof of Artifacts

*Wire diagrams and demos for quick review.*

### Wire / Architecture

```mermaid
flowchart LR
    User[User] --> Server[oakilydokily]
    Server --> Pages[Pages: / /about etc]
    Server --> Mural[mural-wasm embed]
    Mural --> Pets[Pets + scroll-triggered scenes]
```

### Screenshots

| View | Description |
|------|-------------|
| Hero | Landing with mural embed |
| Mural | Interactive pets, Cozy Nook, Winter Tubing, Doggy Door |

### Demo

*Add `docs/artifacts/demo-scroll.gif` for scroll + mural interaction.*

---

## Run

```bash
cargo run -p oakilydokily --features approuter
```

## mural-wasm

See [mural-wasm/README.md](mural-wasm/README.md) for the interactive mural crate.
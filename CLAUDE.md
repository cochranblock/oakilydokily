<!-- Unlicense — cochranblock.org -->
<!-- Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3 -->

# oakilydokily

Veterinary professional services site. Multi-auth, ESIGN waivers, federal compliance docs, AI sprite generation.

- mural-wasm is a sub-project in this repo (`./mural-wasm/`, archived — static mural active)
- Build: `cargo build --release -p oakilydokily --features approuter`
- Test: `cargo run -p oakilydokily --bin oakilydokily-test --features tests`
- Port: `$PORT` (default 3000), bind: `$BIND` (default 0.0.0.0)
- Data: `$OAKILYDOKILY_DATA_DIR` or platform data dir
- Auth: Google/Facebook/Apple OAuth + manual email/password
- Email: Gmail API (Workspace) → Resend fallback
- DB: SQLite WAL, 7-year waiver retention
- Forge: `/api/forge` → SSH to kova GPU node → [pixel-forge](https://github.com/cochranblock/pixel-forge)
- Required env: `SESSION_SECRET` (32+ bytes), `OD_BASE_URL`
- See [`.env.example`](.env.example) for all options

<!-- Unlicense — cochranblock.org -->

# Backlog

Prioritized work items for oakilydokily. Most important at top. Max 20.

> Tags: `[build]` `[test]` `[fix]` `[feature]` `[docs]` `[research]`
>
> Cross-project deps in **bold**. This backlog self-reorganizes based on recency and relevance.

---

1. `[fix][CRITICAL]` Forge RCE + unauthed endpoint — `/api/forge` is a public POST endpoint with no session check; `class`/`palette` fields go directly into a bash shell string via SSH, enabling shell injection → RCE on `gd`. Fix: (a) add session auth check at handler entry (same pattern as `/waiver`), (b) eliminate shell string interpolation — pass the JSON via a temp file or `--` arg array instead of `echo '...'`. Both fixes in `src/web/forge.rs`.
2. `[fix][HIGH]` Android build broken + Apple JWT unverified — `android.rs` constructs `AppState` without `s3` (rate limiter field), failing to compile for Android target. Also: `f93()` in `auth.rs` decodes Apple `id_token` by base64-decoding the payload without verifying the RS256 signature against Apple's public keys, enabling identity forgery. Fix: (a) add `s3: Arc::new(Mutex::new(HashMap::new()))` to `android.rs` AppState construction; (b) fetch Apple JWKS from `https://appleid.apple.com/auth/keys`, cache, verify signature before trusting claims.
3. `[feature][UX]` Login-wall explanation + `/waiver/status` portal — When `/waiver` redirects to login, the user sees a bare login form with no context. Add a query param (`?reason=waiver`) and render a contextual message ("Create an account so your signature is legally attributable"). After signing, add `GET /waiver/status` showing the user's signed waivers with reference IDs, timestamps, and terms version — satisfying the 7-year retention visibility requirement.
4. `[build]` Upgrade reqwest 0.11 → 0.12 — current version is 2 minors behind, 0.12 has better async and smaller binary
5. `[feature]` Forge UI — add `/forge` page with form to submit sprite generation requests and display results. Currently API-only (POST `/api/forge`). **Depends on [pixel-forge](https://github.com/cochranblock/pixel-forge) deployed to GPU node**
6. `[fix]` Apple OAuth marked deprecated in code comment but still routed — either remove or undeprecate
7. `[test]` Adversarial input tests — XSS in waiver name/email/signature fields, SQL injection attempts, oversized payloads
8. `[feature]` Login rate limit feedback — show "too many attempts, try again in X seconds" instead of bare 429 text
9. `[build]` Android pocket server — wire WebView + Rust server for mobile. **Depends on [pocket-server](https://github.com/cochranblock/pocket-server)**
10. `[docs]` Add CONTRIBUTING.md — build instructions, test instructions, env var requirements for new contributors
11. `[research]` Evaluate replacing bcrypt with argon2id for password hashing — bcrypt DEFAULT_COST may be too low for 2026
12. `[feature]` Booking system — replace external Calendly link with built-in availability calendar at `/book`
13. `[test]` Visual regression baseline — exopack screenshot diff against stored baselines, fail on pixel drift > threshold
14. `[research]` P23 follow-up — confidence calibration for kova pyramid T1→T2 escalation affects forge dispatch quality. **Depends on [kova](https://github.com/cochranblock/kova) pyramid shipping**
15. `[build]` Shrink mural.png (1.1 MB) — convert to WebP or optimize PNG, embedded in binary via rust-embed

<!-- Unlicense — cochranblock.org -->

# Backlog

Prioritized work items for oakilydokily. Most important at top. Max 20.

> Tags: `[build]` `[test]` `[fix]` `[feature]` `[docs]` `[research]`
>
> Cross-project deps in **bold**. This backlog self-reorganizes based on recency and relevance.

---

1. `[fix]` SESSION_SECRET length check at startup, not first auth request — fail fast with clear error on boot
2. `[test]` Add auth-gated waiver tests to triple sims — 4 checks currently SKIP (set OD_TEST_WAIVER_BYPASS=1 path in test binary)
3. `[feature]` Rate limiter cleanup task — spawn tokio task to prune expired entries from s3 every 60s, prevent unbounded HashMap growth
4. `[fix]` Forge fallback — `/api/forge` returns 502 when GPU node `gd` is unreachable; add graceful error page or retry with backoff. **Depends on [kova](https://github.com/cochranblock/kova) C2 node availability**
5. `[feature]` Waiver email retry — if Gmail + Resend both fail, queue for retry instead of one-shot. Current: user sees warning but email is lost
6. `[test]` Add `/govdocs/*` route coverage to test binary — 13 govdoc routes untested (only page routes in f30 today)
7. `[fix]` Remove unused `State(_s)` from page handlers (pages.rs home/about/contact) — extracting AppState for no reason
8. `[feature]` Client waiver portal — after signing, let authenticated users view their waiver status and reference ID at `/waiver/status`
9. `[build]` Upgrade reqwest 0.11 → 0.12 — current version is 2 minors behind, 0.12 has better async and smaller binary
10. `[feature]` Forge UI — add `/forge` page with form to submit sprite generation requests and display results. Currently API-only (POST `/api/forge`). **Depends on [pixel-forge](https://github.com/cochranblock/pixel-forge) deployed to GPU node**
11. `[fix]` Apple OAuth marked deprecated in code comment but still routed — either remove or undeprecate
12. `[test]` Adversarial input tests — XSS in waiver name/email/signature fields, SQL injection attempts, oversized payloads
13. `[feature]` Login rate limit feedback — show "too many attempts, try again in X seconds" instead of bare 429 text
14. `[build]` Android pocket server — wire WebView + Rust server for mobile. **Depends on [pocket-server](https://github.com/cochranblock/pocket-server)**
15. `[docs]` Add CONTRIBUTING.md — build instructions, test instructions, env var requirements for new contributors
16. `[research]` Evaluate replacing bcrypt with argon2id for password hashing — bcrypt DEFAULT_COST may be too low for 2026
17. `[feature]` Booking system — replace external Calendly link with built-in availability calendar at `/book`
18. `[test]` Visual regression baseline — exopack screenshot diff against stored baselines, fail on pixel drift > threshold
19. `[research]` P23 follow-up — confidence calibration for kova pyramid T1→T2 escalation affects forge dispatch quality. **Depends on [kova](https://github.com/cochranblock/kova) pyramid shipping**
20. `[build]` Shrink mural.png (1.1 MB) — convert to WebP or optimize PNG, embedded in binary via rust-embed

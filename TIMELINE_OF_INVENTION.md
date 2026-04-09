<!-- Unlicense — cochranblock.org -->

# Timeline of Invention

*Dated, commit-level record of what was built, when, and why. Proves human-piloted AI development — not generated spaghetti.*

> Every entry maps to real commits. Run `git log --oneline` to verify.

## How to Read This Document

Each entry follows this format:

- **Date**: When the work shipped (not when it was started)
- **What**: Concrete deliverable — binary, feature, fix, architecture change
- **Why**: Business or technical reason driving the decision
- **Commit**: Short hash(es) for traceability
- **AI Role**: What the AI did vs. what the human directed

This document exists because AI-assisted code has a trust problem. Anyone can generate 10,000 lines of spaghetti. This timeline proves that a human pilot directed every decision, verified every output, and shipped working software.

---

## Human Revelations — Invented Techniques

*Novel ideas that came from human insight, not AI suggestion. These are original contributions to the field.*

### ESIGN-Compliant Waiver with 7-Year Retention in Single Binary (March 2026)

**Invention:** A legally compliant electronic signature waiver system (ESIGN Act, 15 U.S.C. 7001-7031) with 7-year retention, audit trail, and multi-auth — all in a single Rust binary with SQLite WAL, no external services.

**The Problem:** Veterinary service businesses need signed liability waivers before boarding animals. Paper waivers get lost. DocuSign costs $25/month and requires cloud connectivity. Self-hosted solutions (HelloSign alternatives) are PHP/Python stacks with PostgreSQL that need a full server.

**The Insight:** ESIGN compliance requires three things: (1) intent to sign (a separate signature field from the name field), (2) consent to electronic records, and (3) retention of the signed record. None of these require a cloud service. A SQLite database with WAL mode, a separate `signature` input distinct from `full_name`, and a 2,557-day (7-year) retention policy in the archive pruner — that's the entire legal requirement, in a single binary.

**The Technique:**
1. Separate `signature` field from `full_name` — ESIGN requires demonstrated intent, not just typing a name
2. SQLite WAL mode: concurrent reads during writes, crash-safe, single-file database
3. `archive_prune`: 2,557-day retention (7 years) — was incorrectly set to 365 days, caught and fixed during audit
4. Audit trail: timestamped record of who signed what, when, from which IP
5. Multi-auth: Google/Facebook/Apple/manual — signer identity verified before signing
6. Zero external services: no DocuSign, no Adobe Sign, no cloud storage

**Result:** A veterinary business gets ESIGN-compliant waivers in a 9.1MB binary. 7-year retention. Audit trail. Multi-auth identity verification. No monthly subscription. No cloud dependency.

**Named:** Single-Binary ESIGN Waiver
**Commit:** `79d816d` (signature field + 7-year retention)
**Origin:** A real business requirement — the first paying partnership needed liability waivers for animal boarding. Michael Cochran realized the entire ESIGN compliance requirement fits in a SQLite schema and a Rust handler. No SaaS needed.

### Zero-JavaScript Architecture (March 2026)

**Invention:** A production web application with 2,496 lines of Rust and 0 lines of JavaScript — server-rendered HTML with CSS animations, no WASM, no client-side framework, no build step for frontend code.

**The Problem:** Modern web apps ship megabytes of JavaScript to the client. React, Vue, Angular — they all require Node.js build pipelines, npm dependencies, and client-side rendering. The JavaScript ecosystem has more supply chain vulnerabilities than any other language ecosystem. For a simple waiver form, this is massive overkill.

**The Insight:** HTML forms work without JavaScript. CSS animations work without JavaScript. The `<form>` element with `action` and `method` attributes is the original client-server protocol — it sends data to the server and the server responds with a new page. This is sufficient for a waiver form, a login page, a dashboard, and a mural. Zero JavaScript means zero client-side supply chain risk.

**The Technique:**
1. All pages server-rendered via Axum handlers returning HTML strings
2. All interactivity via HTML forms with POST actions
3. All animations via CSS (gradients, transitions, keyframes)
4. Static mural via CSS gradient instead of WASM canvas (replaced 67KB of JavaScript)
5. No build step for frontend — `cargo build` is the only build command

**Result:** 2,496 lines of Rust. 0 lines of JavaScript. No Node.js. No npm. No webpack. No client-side rendering. The entire site loads in one HTTP response with zero additional script fetches.

**Named:** Zero-JS Architecture
**Commit:** `cb0cbf6` (JS removal)
**Origin:** Removing the WASM canvas mural (which required gl.js, mural-bridge.js) and realizing that a CSS gradient looked just as good. If the most complex visual element doesn't need JavaScript, nothing does.

### 2026-04-08 — Human Revelations Documentation Pass

**What:** Documented novel human-invented techniques across the full CochranBlock portfolio. Added Human Revelations section with Single-Binary ESIGN Waiver and Zero-JS Architecture.
**Commit:** See git log
**AI Role:** AI formatted and wrote the sections. Human identified which techniques were genuinely novel, provided the origin stories, and directed the documentation pass.

---

## Entries

### 2026-04-09 — Docs Refresh: Timeline + Proof Audit

**What:** Full audit of TIMELINE_OF_INVENTION.md and PROOF_OF_ARTIFACTS.md against current source. Backfilled the 2026-04-03 security/test burst that was missing from the timeline. Updated Proof metrics: binary 8.7 → 8.8 MB, Rust LOC 2,748 → 3,211 (+16.8%), integration checks 25 → 59, added 15 unit tests, route count raised from 19 tested → 32 registered. Added "How to Read This Document" section (cochranblock pattern). Added Named Techniques section to Proof.
**Why:** Docs drifted after the 2026-04-03 security burst — the forge RCE fix and backlog sprint weren't represented in the invention timeline, and Proof metrics were stale by several hundred lines of code. Federal procurement reviewers and security auditors check these files first; they must match what's in git.
**Commit:** (this commit)
**AI Role:** AI read git log, counted routes/tests/LOC, wrote new entries, synchronized metrics. Human directed the refresh pass.

### 2026-04-03 — Forge RCE Close + Backlog Sprint (Security + Tests)

**What:** Closed a critical RCE in `/api/forge`: (1) added `auth::f88(&jar)` auth gate at handler entry — unauthenticated POSTs now get 401 before any SSH is attempted, (2) replaced shell-string interpolation with a compile-time-constant `remote_cmd()` and stdin-only JSON delivery — user fields never touch a shell. Also executed backlog items 1-3, 6-7: SESSION_SECRET fail-fast at startup (exits if <32 chars when any auth provider is configured), `OD_TEST_WAIVER_BYPASS=1` unlocks 4 previously-skipped waiver checks (test count 25 → 38), rate-limiter HashMap pruned every 60s by background tokio task (prevents unbounded memory growth under auth traffic), 13 govdocs route checks added, unused `State` extractor removed from page handlers. Added 12 unit tests in `waiver.rs` (validation, insert roundtrip, user CRUD, terms_hash determinism) and 3 in `forge.rs` (`remote_cmd_is_constant`, `injection_payload_not_in_remote_cmd`, `cache_key_includes_all_fields`). Added adversarial POST /waiver tests (XSS in name/email/signature, SQL injection, oversized payloads, missing consent), forge auth-gate integration test, 4 forge injection integration tests (single-quote, backtick, subshell, newline), and 8 snapshot content checks. Created BACKLOG.md with 20 prioritized work items.
**Why:** The P23 Triple Lens analysis flagged `/api/forge` as the highest-severity gap in the portfolio — an anonymous caller could trigger SSH to the production GPU node with attacker-controlled shell metacharacters. Verdict from that session: "solid machine, no locks on the doors." This commit puts the locks on. Triple sims: 3/3 pass, 38 checks each, 0 skips. Clippy: 0 warnings (release + tests).
**Commits:** `640e5ae` (RCE close — auth gate + stdin-only JSON), `d8a0663` (forge retry/backoff + 12 waiver unit tests + adversarial integration tests), `d0fa1a1` (SESSION_SECRET fail-fast + bypass env + rate-limiter cleanup + govdocs coverage + State removal), `58168a5` (BACKLOG.md), `8655e26` (P23 readjust fire)
**AI Role:** AI identified attack surfaces via code audit, wrote all tests, implemented all fixes, ran triple sims to green. Human directed triage priorities after P23 synthesis and approved remediation approach.

### 2026-04-03 — P23 Triple Lens Synthesis: Pyramid Architecture Risk/Opportunity

**What:** Ran P23 Triple Lens Research Protocol across the fleet to assess the [kova](https://github.com/cochranblock/kova) pyramid architecture (subatomic model inference, mmap'd shared weights, tournament training, IRONHIVE GPU cluster). Three panes: rogue-repo (optimist), ronin-sites (pessimist), illbethejudgeofthat (paranoia). Synthesis produced from oakilydokily pane. Key findings: (1) infrastructure is real — 10K+ lines, 3 trained models, working tournament, (2) mmap'd nanobyte format needs Ed25519 signature and offset bounds checking before ship, (3) confidence calibration is the silent killer — overconfident T1 means wrong answers never escalate, (4) never delete the Claude API key — graduate to "Claude-rare" not "Claude-impossible", (5) training corpus (crates.io) needs filtering (min downloads, known-author whitelist). One-sentence verdict: solid machine, no locks on the doors.
**Commit:** (this commit)
**AI Role:** AI ran optimist/paranoia analysis and synthesized all three lenses. Human directed the P23 protocol and dispatched panes.

### 2026-04-02 — Async I/O, Rate Limiting, Dead Code Cleanup, Docs Truth Audit

**What:** Fixed 8 instances of `reqwest::blocking::Client` in async handlers (auth.rs, email.rs) — replaced with async `reqwest::Client`. Added IP-based rate limiting (10/60s) on login and signup endpoints. Email confirmation failures now surfaced to user on waiver confirmed page. Removed `#![allow(dead_code)]` from all 9 source files, 0 clippy warnings. Removed `"blocking"` feature from reqwest dep. Full docs truth audit: README, POA, TOI updated to match code. Fixed false "separate repo" claim in CLAUDE.md. Updated mermaid diagrams to show static mural (not WASM). Forge status corrected from "Coming Soon" to "Implemented". Added supply-chain-audit govdocs route. Added Production Features section to README. Cross-linked all sibling repos.
**Commits:** `7575531`, (this commit)
**AI Role:** AI executed full guest analysis, identified bugs via code audit, implemented all fixes. Human directed priorities and approved plan.

### 2026-04-02 — Dependency Chain Visibility

**What:** Added "Dependencies on Other CochranBlock Projects" table to README. All cross-project dependencies now link to `github.com/cochranblock/{project}`. Updated mermaid diagrams.
**Commit:** `1470bdd`
**AI Role:** AI implemented. Human directed the linking pattern.

### 2026-03-30 — Zero Downtime Hot Reload

**What:** SO_REUSEPORT socket binding, PID lockfile management, SIGTERM/SIGKILL old instance. New binary binds port before killing old — zero dropped requests during deploy.
**Commit:** (this commit)
**AI Role:** AI implemented hot reload pattern. Human specified the deployment model.

### 2026-03-30 — Supply Chain Security Audit

**What:** Full EO 14028 audit: cargo audit (fixed RUSTSEC-2026-0049), duplicate dep analysis, deep code review (unsafe, network, crypto), file cleanup (removed 42 MB dead ONNX model). govdocs/SUPPLY_CHAIN_AUDIT.md.
**Commit:** `db89576`
**AI Role:** AI ran all audit tools and wrote findings. Human approved remediation.

### 2026-03-29 — iOS + PWA + Multi-Arch (12 Targets)

**What:** iOS scaffold (Swift WebView + Rust staticlib). PWA (manifest.json, service worker, offline-first). Multi-arch build script for 12 targets. Supported platforms table in README.
**Commit:** `e56fc16`
**AI Role:** AI created all platform scaffolds. Human directed architecture.

### 2026-03-29 — Android AAB: 4.6 MB

**What:** Full Gradle project, cargo-ndk build, JNI bridge, launcher icon. AAB builds successfully targeting API 35. Pocket Server: Rust web server + Android WebView.
**Commits:** `0df7ec6`, `db89576`
**AI Role:** AI built Gradle project and resolved NDK cross-compilation. Human directed Pocket Server architecture.

### 2026-03-28 — Zero JavaScript: 2,496 Rust / 0 JS

**What:** Removed all JavaScript (67 KB gl.js, mural-bridge.js, mural-wasm.wasm). Replaced WASM canvas with static mural + CSS gradient. Added /govdocs routes serving 11 compliance docs at runtime via include_str!.
**Commit:** `cb0cbf6`
**AI Role:** AI removed JS and added govdocs routes. Human approved zero-JS architecture.

### 2026-03-27 — Federal Compliance Documentation

**What:** Full govdocs/ suite: SBOM (EO 14028), SSDF (NIST SP 800-218), supply chain, security posture, accessibility (Section 508), privacy impact, FIPS 140-2/3, FedRAMP, CMMC L1-2, ITAR/EAR, federal use cases.
**Commit:** `5355134`
**AI Role:** AI drafted all compliance docs. Human directed scope and verified claims against source code.

### 2026-03-27 — User Story Analysis + Error UX

**What:** Full user walkthrough analysis (USER_STORY_ANALYSIS.md). Waiver validation errors now return styled HTML pages instead of raw text. Added .env.example.
**Commit:** `5220766`
**AI Role:** AI conducted user story walkthrough and implemented error page fix. Human directed analysis scope.

### 2026-03-27 — Binary Optimization: 42 MB → 9.1 MB

**What:** Release binary cut from 42 MB to 9.1 MB. Removed rust-gmail dep (killed openssl chain), added strip/LTO/codegen-units=1, trimmed tokio features, removed 25 MB of unused source PNGs from embedded assets.
**Commit:** `19f481b`
**AI Role:** AI audited dependency tree and identified bloat sources. Human approved removals.

### 2026-03-27 — Mobile CSS + 480px Breakpoint

**What:** Added phone-width (480px) breakpoint for iPhone SE / small Android. Full-width CTAs, compact nav, tighter padding.
**Commit:** `46db290`
**AI Role:** AI identified missing breakpoint and implemented responsive rules.

### 2026-03-27 — Waiver: Signature Field + 7-Year Retention + Signup Routes

**What:** Added dedicated signature input (ESIGN: typed name separate from full_name). Fixed archive_prune from 365 → 2557 days. Wired /auth/signup routes. Added email format validation.
**Commit:** `79d816d`
**AI Role:** AI audited ESIGN compliance and implemented fixes. Human directed legal requirements.

### 2026-03-27 — QA Round 1: Clippy + Triple Sims

**What:** Fixed clippy warning (Range::contains). Full health check: release build clean, triple sims 3/3, all routes verified.
**Commit:** `5576efe`
**AI Role:** AI ran full QA pass and fixed warning.

### 2026-03-27 — Docs Update: README, Compression Map, Production Checklist

**What:** Updated README (14 repos, architecture diagram, module table). Added D1 auth + forge entries to compression map. Added missing env vars to PRODUCTION.md.
**Commit:** `3a9a547`
**AI Role:** AI audited docs against code and fixed stale references.

### 2026-03-22 — Pixel Forge Mural Integration

**What:** /api/forge endpoint — SSH dispatches to GPU node for AI sprite generation. WASM loads forged sprites dynamically.
**Commit:** `b066b52`
**AI Role:** AI built SSH dispatch and WASM loader. Human designed the cache strategy and sprite format.

### 2026-03-20 — Claymation: Manual Regions Mode

**What:** Added manual region selection for claymation extraction. Prefer rembg-only segmentation over auto.
**Why:** Auto segmentation was inconsistent. Manual regions + rembg gives reliable extraction from the mural.
**Commit:** `01a8c09`
**AI Role:** AI implemented region selection. Human identified which animals to extract and quality-checked results.

### 2026-03-19 — Mural-WASM: Sprite Fallback Chain

**What:** Three-tier fallback: claymation → CC0 pixel art → mural-only (no sprites). Resilient rendering.
**Commits:** `2377100`, `f64d519`
**AI Role:** AI built fallback chain. Human designed degradation strategy.

### 2026-03-18 — Claymation Pipeline: Pure Rust

**What:** Full image processing pipeline in Rust — segment, inpaint, pixelate, rotate, composite. No Python, no external tools.
**Commit:** `e48f3ee`
**AI Role:** AI wrote image processing code. Human directed the artistic pipeline and validated output quality.

### 2026-03-17 — Mural-WASM: CC0 Pixel Art + Scroll Scenes

**What:** CatnDog + Kenney CC0 sprites as fallback. Scroll-triggered scenes: Cozy Nook, Winter Tubing, Doggy Door.
**Commits:** `21e6ac8`, `ed02152`
**AI Role:** AI implemented scroll detection and scene transitions. Human designed the storytelling flow.

### 2026-03-14 — Waiver System + Multi-Auth

**What:** ESIGN-compliant liability waiver with audit trail. Google/Facebook/Apple/manual OAuth.
**Why:** Real business requirement — veterinary services need signed liability waivers before boarding animals.
**AI Role:** AI built auth flows and waiver persistence. Human specified legal compliance requirements.

---

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. All source under the Unlicense.*

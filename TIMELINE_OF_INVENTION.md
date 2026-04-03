<!-- Unlicense — cochranblock.org -->

# Timeline of Invention

*Dated, commit-level record of what was built, when, and why.*

> Every entry maps to real commits. Run `git log --oneline` to verify.

---

## Entries

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

<!-- Unlicense — cochranblock.org -->

# Timeline of Invention

*Dated, commit-level record of what was built, when, and why.*

> Every entry maps to real commits. Run `git log --oneline` to verify.

---

## Entries

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

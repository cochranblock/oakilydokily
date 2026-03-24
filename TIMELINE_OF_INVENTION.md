<!-- Unlicense — cochranblock.org -->

# Timeline of Invention

*Dated, commit-level record of what was built, when, and why.*

> Every entry maps to real commits. Run `git log --oneline` to verify.

---

## Entries

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

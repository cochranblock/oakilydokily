# Assumed Breach Threat Model

> **Operating assumption: every component below is already compromised. Design for damage containment and loud detection, not for prevention.**

This document is the canonical threat model for every project in the `cochranblock/*` portfolio. Each project adapts the Threat Surface section for its own context but shares the same first principles, mitigations, and verification protocol.

---

## First Principles

1. **Every record that matters has an external witness.** Hashes published to public git (or equivalent neutral timestamp authority) so tampering requires simultaneously corrupting your system AND the public chain.
2. **No single point of compromise.** Signing keys in hardware (YubiKey / TPM / Secure Enclave). Never in software. Never in env vars. Never in config files.
3. **Default air-gap.** No network dependency for correctness. Network is for backup + publishing hashes, both signed, both verifiable post-hoc.
4. **Append-only everything.** No delete path in any storage layer. Corrections are reversing entries referencing the original. Standard accounting discipline, enforced in code.
5. **Cryptographic audit chain.** Every day's state derives from the previous day's hash. Tampering with any day invalidates every subsequent day.
6. **Disclosure of methodology is a security feature.** If an auditor can independently verify the algorithm, they can independently verify the outputs. No "trust us" layers.
7. **Separation of duties enforced in software.** Entry, approval, and audit live in different trust zones. Compromise of one does not compromise the others.
8. **Redundancy across trust zones.** Local + different-cloud + different-format + offline. Attacker must compromise all to hide damage.
9. **Test breach scenarios regularly.** Triple Sims applied to tamper detection. If the chain does not detect a simulated tamper, the chain is broken.

---

## Threat Surface — oakilydokily

### Records of consequence

| Record | Storage | Retention | Why it matters |
|--------|---------|-----------|----------------|
| ESIGN waivers | `waivers.sqlite` (WAL) + gzipped archive in `waivers/` | 7 years (federal) | Forging or deleting a signed waiver = federal ESIGN fraud. Each row captures full_name, email, signed_at, IP, user_agent, terms_version, terms_hash, consent flag, signature_text. |
| User credentials | `users` table in same SQLite DB | Account lifetime | Email/password hashes. Compromise = identity theft + waiver forgery under another user's name. |
| OAuth tokens / sessions | In-memory session store, signed with `SESSION_SECRET` | Session lifetime | SESSION_SECRET compromise = impersonate any user, sign waivers on their behalf. |
| Waiver confirmation emails | Gmail Workspace → Resend fallback | Sent copy = user's external witness | If email provider is compromised, user loses their independent receipt of signing. |
| Forge job dispatch | SSH to n1 (gd) → pixel-forge binary | Transient (cache only) | Not a legal record, but the SSH channel is a lateral-movement vector into the kova C2 fleet. |

### Threat enumeration

| # | Assume compromised | oakilydokily-specific impact |
|---|-------------------|------------------------------|
| 1 | **SQLite DB file exfiltrated** | Single `.sqlite` file contains every waiver ever signed + every user credential hash. One `scp` = full history. Gzipped archive dir doubles the exposure surface. |
| 2 | **SESSION_SECRET leaked** | Attacker mints valid session cookies → acts as any user → signs waivers, accesses forge, reads waiver history. No per-user key rotation possible; secret rotation invalidates all sessions. |
| 3 | **OAuth provider breach** (Google/Facebook/Apple) | Attacker authenticates as the victim user without touching oakilydokily infrastructure. Waiver signing proceeds normally under stolen identity. |
| 4 | **Email provider compromise** (Gmail Workspace / Resend) | Waiver confirmation emails suppressed or forged. User's external witness of signing is destroyed or fabricated. |
| 5 | **Forge SSH lateral movement** | Forge endpoint SSHs to n1 (gd). If n1 compromised, attacker pivots to kova C2 fleet. If oakilydokily compromised, attacker gains SSH access to n1. Auth gate + stdin-only JSON delivery (patched 2026-04-03) limits shell injection but does not eliminate the trust relationship. |
| 6 | **Binary / server process compromise** | Attacker serves altered waiver terms, captures signatures under false pretenses, or silently drops waiver records before they reach SQLite. |
| 7 | **Clock manipulation** | ESIGN timestamps are the legal proof of when a waiver was signed. Shifted clock = backdated or future-dated signatures. |
| 8 | **Supply chain (crate deps)** | `sqlx`, `axum`, `sha2`, `flate2`, `chrono`, `uuid` — any backdoored dep can exfiltrate waiver PII or inject false records. |
| 9 | **Backup compromise** | If SQLite backups are the only copy, tampering the backup = rewriting legal history. |
| 10 | **Insider / self-tampering** | No admin delete path exists in code (append-only schema), but raw SQLite access bypasses application-layer controls. |

### N/A or modified applicability

- **Default air-gap (Principle 3):** N/A as stated. oakilydokily is an always-online HTTP service. Adapted: waiver records should be exportable offline and verifiable without network access.
- **Hardware-key signing of every output:** N/A per-record. The *user* ESIGN-signs; the service does not countersign individual waivers with a hardware key. Adapted: server identity (TLS cert) and deploy artifacts can be hardware-signed; individual waiver rows cannot without changing the signing UX.
- **Public-chain for waiver records:** MUST NOT publish waiver bodies (PII). Adapted: daily BLAKE3 roll-up hash of all waivers signed that day can be committed to `cochranblock/oakilydokily-chain` without exposing PII. Record bodies stay private; hash proves they existed at commit time.

---

## Mitigations

| Assume | Mitigation | Verification |
|--------|-----------|--------------|
| Binary compromised | Hardware-key signatures for every output of consequence | Anyone can verify the public key matches expected fingerprint |
| Storage compromised | Append-only sled trees. Delete is not a function, not a policy. | Hash chain breaks on any rewrite. External witness detects. |
| Network MITM | Air-gap capable. Network used only for signed backups + hash publishing. | NTP + GitHub timestamp + hardware counter cross-checked. |
| Signing key stolen | Daily hash committed to public git. Stolen key cannot retroactively change committed days. | Any day older than the public commit is immutable in evidence. |
| Audit log tampered | Separate sled tree, write-only from main app. Auditor tool reads both + cross-checks. | Compromise of main app leaves audit log intact. |
| Backup tampered | 3 different targets with 3 different credentials (local USB + off-site cloud + paper). | Attacker needs all three to hide damage. |
| Insider / self-tampering | No admin role. No delete. Reversing entries only. | Legal record immune to author second-thoughts. |
| Clock manipulation | Multiple time sources: local clock, NTP, git commit timestamp, hardware-key counter. | Divergence flags exception requiring supervisor approval. |
| Supply chain (deps) | `cargo audit` in CI. Pinned SBOM. Reproducible builds where possible. | Anyone can reproduce the binary from source + lockfile. |
| Physical device seizure | Full-disk encryption. Hardware key physically separate from device. | Stolen laptop without key is useless for forgery. |

---

## Public-Chain Deployment

This project publishes tamper-evident hashes to a public companion repo: `cochranblock/<project>-chain` (where `<project>` is the project name).

- **Daily cycle:** at 23:59 local, compute BLAKE3 of all records-of-consequence from the day. Sign with hardware key. Commit to chain repo. Push.
- **GitHub timestamp** on the commit = neutral third-party witness. Anyone can cold-verify records were not rewritten after commit time.
- **Verification:** `<project> verify` reads the chain and re-derives hashes. Any divergence = tampering detected.

This pattern is a private Certificate Transparency log for project state. Same primitive Google uses for TLS certs, applied to whatever the project tracks.

---

## Triple Sims for Tamper Detection

Standard Triple Sims gate (run 3x identically) extended with a tamper-scenario sim:

1. Normal run → produce canonical output
2. Simulated tampering (flip one bit in storage) → `verify` must flag it
3. Simulated clock rewind → `verify` must flag it

If any sim fails to detect, the chain is broken. Fix before merge.

---

## Scope of this Document

- Covers: any artifact this project emits that has legal, financial, or audit consequence.
- Does NOT cover: source code itself (public under Unlicense, not sensitive), build outputs (reproducible), marketing content (public by design).
- If your project emits no records of consequence, the relevant sections are zero-length and the public-chain deployment is skipped. Document that explicitly.

---

## Relation to Other Docs

- **TIMELINE_OF_INVENTION.md** — establishes priority dates for contributions. Feeds into the chain's initial state.
- **PROOF_OF_ARTIFACTS.md** — cryptographic signatures on release artifacts. Adjacent pattern, same first principles.
- **DCAA_COMPLIANCE.md** (where applicable) — how this threat model satisfies FAR/DFARS audit requirements.

---

## Status

- [x] Threat Surface section adapted for this project
- [ ] Hardware-key signing integrated or N/A documented
- [ ] Public-chain repo created and connected or N/A documented
- [ ] Triple Sims tamper-detection test present or N/A documented
- [ ] External verification procedure documented

---

*Unlicensed. Public domain. Fork, strip attribution, adapt, ship.*

*Canonical source: cochranblock.org/threat-model — last revision 2026-04-14*

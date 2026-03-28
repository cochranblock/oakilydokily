<!-- Unlicense — cochranblock.org -->

# FIPS 140-2/3 Status

> Last updated: 2026-03-27

## Current Crypto Primitives

| Primitive | Library | FIPS Validated? | FIPS Algorithm? |
|-----------|---------|----------------|-----------------|
| SHA-256 | sha2 (RustCrypto) | No | Yes (FIPS 180-4) |
| HMAC-SHA256 | hmac (RustCrypto) | No | Yes (FIPS 198-1) |
| bcrypt | bcrypt crate | No | No (not FIPS-approved) |
| RSA-2048 (RS256) | ring via jsonwebtoken | No (ring is not FIPS-validated) | Yes (FIPS 186-4) |
| TLS 1.2/1.3 | rustls + ring | No | Algorithms are FIPS-approved |
| Random | rand + getrandom | No | Uses OS CSPRNG (acceptable) |

## FIPS Status: NOT FIPS-VALIDATED

This project uses **FIPS-approved algorithms** (SHA-256, HMAC, RSA, AES via TLS) but the **implementations are not FIPS 140-2/3 validated modules**.

### bcrypt Exception

bcrypt is not a FIPS-approved algorithm. It is used for password hashing of manual-auth users. FIPS-compliant alternative: PBKDF2 with SHA-256 (NIST SP 800-132).

## Path to FIPS Compliance

1. **Replace ring with aws-lc-rs**: AWS-LC is pursuing FIPS 140-3 validation. rustls supports aws-lc-rs as a backend. Switch reqwest and jsonwebtoken to use aws-lc-rs.
2. **Replace bcrypt with PBKDF2-SHA256**: Use the `pbkdf2` crate from RustCrypto with SHA-256.
3. **Enable FIPS mode in aws-lc-rs**: Set `fips` feature flag when available.
4. **Validate deployment**: FIPS validation is per-module, per-platform. The deployed binary on the target OS must use the validated module.

## Deployment Context

This application runs **on-premise** behind a Cloudflare tunnel. TLS termination happens at Cloudflare edge (Cloudflare holds FIPS-validated certificates). The application's internal crypto is for:
- Session cookie signing (HMAC-SHA256)
- Password storage (bcrypt)
- Waiver terms integrity (SHA-256)
- OAuth JWT signing (RS256) — only for Gmail API service account

None of these handle classified data or CUI. FIPS validation may not be required depending on the contract.

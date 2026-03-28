<!-- Unlicense — cochranblock.org -->

# Export Control Classification (ITAR/EAR)

> Last updated: 2026-03-27

## ITAR (International Traffic in Arms Regulations)

**Not ITAR controlled.** This software is not on the United States Munitions List (USML). It is a veterinary professional services website with waiver management. No defense articles, no defense services, no technical data related to defense.

## EAR (Export Administration Regulations)

### Classification

**EAR99** — Not specifically controlled under any Export Control Classification Number (ECCN).

### EAR Category 5 Part 2 — Cryptography

This software includes encryption functionality:

| Function | Crypto Used | Classification |
|----------|------------|----------------|
| TLS (HTTPS) | rustls (TLS 1.2/1.3, AES-128/256-GCM, ChaCha20-Poly1305) | Mass-market encryption exception (EAR 740.17(b)) |
| Session signing | HMAC-SHA256 | Ancillary to primary function |
| Password hashing | bcrypt | Authentication, not encryption |
| JWT signing | RS256 (RSA-2048) | Standard authentication protocol |
| Terms hashing | SHA-256 | Integrity verification, not encryption |

### Encryption Exception

This software qualifies for the **mass-market encryption exception** under EAR Section 740.17(b):
- Encryption is standard TLS for web communication
- No custom or proprietary encryption algorithms
- Encryption functionality is ancillary to the primary purpose (waiver management)
- Uses publicly available encryption libraries

### TSU Exception

Additionally qualifies for **Technology and Software Unrestricted (TSU)** exception under EAR 740.13:
- Source code is publicly available (Unlicense, published on GitHub)
- No restrictions on who can access the source

## Summary

| Regulation | Status |
|-----------|--------|
| ITAR | Not controlled |
| EAR ECCN | EAR99 |
| Crypto export | Mass-market exception (740.17(b)) + TSU (740.13) |
| License required | No |

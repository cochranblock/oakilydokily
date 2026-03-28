<!-- Unlicense — cochranblock.org -->

# CMMC Level 1-2 Practices

> Mapping oakilydokily to CMMC 2.0 domains.
> Last updated: 2026-03-27

## CMMC Level 1 (Basic Cyber Hygiene)

### AC — Access Control

| Practice | Status | Implementation |
|----------|--------|---------------|
| AC.L1-3.1.1 Limit system access to authorized users | Met | Auth required for waiver signing (Google/Facebook/Apple/manual). Session cookies with HMAC-SHA256 verification. |
| AC.L1-3.1.2 Limit system access to authorized functions | Met | No admin interface exposed. Static pages are public. Write operations (waiver POST) require authentication. |
| AC.L1-3.1.20 Verify external connections | Met | All outbound connections use HTTPS (rustls TLS 1.2/1.3). No unencrypted external calls. |
| AC.L1-3.1.22 Control public information | Met | Public pages contain no sensitive data. Waiver records not publicly accessible. |

### IA — Identification and Authentication

| Practice | Status | Implementation |
|----------|--------|---------------|
| IA.L1-3.5.1 Identify system users | Met | OAuth identity providers (Google, Facebook, Apple) or email/password with bcrypt. |
| IA.L1-3.5.2 Authenticate users | Met | Multi-provider OAuth + manual auth. 7-day session expiry. HMAC-signed cookies. |

### MP — Media Protection

| Practice | Status | Implementation |
|----------|--------|---------------|
| MP.L1-3.8.3 Sanitize media before disposal | N/A | No removable media. Data is SQLite on disk. Standard file deletion applies. |

### PE — Physical Protection

| Practice | Status | Implementation |
|----------|--------|---------------|
| PE.L1-3.10.1 Limit physical access | N/A | Software product. Physical security is the deployer's responsibility. |

### SC — System and Communications Protection

| Practice | Status | Implementation |
|----------|--------|---------------|
| SC.L1-3.13.1 Monitor communications at boundaries | Partial | Structured logging via tracing. HTTP request tracing via tower-http. No IDS/IPS (deployer responsibility). |
| SC.L1-3.13.5 Implement subnetworks for public components | N/A | Single binary. No internal network segmentation needed. |

### SI — System and Information Integrity

| Practice | Status | Implementation |
|----------|--------|---------------|
| SI.L1-3.14.1 Identify and fix flaws | Met | cargo clippy static analysis. cargo audit for dependency vulnerabilities. Triple-sims test gate. |
| SI.L1-3.14.2 Provide protection from malicious code | Met | No dynamic code execution. No eval(). No user-uploaded executables. Input validation on all boundaries. |
| SI.L1-3.14.4 Update malicious code protection | Met | Dependency updates via cargo update + Cargo.lock. |
| SI.L1-3.14.5 Perform system scans | Partial | cargo audit for known CVEs. No runtime scanning (deployer responsibility). |

## CMMC Level 2 (Advanced) — Partial Coverage

| Domain | Coverage | Notes |
|--------|----------|-------|
| AU — Audit | Partial | Waiver audit trail (IP, UA, timestamp, terms hash). HTTP request logging. No centralized SIEM integration. |
| CM — Configuration Management | Met | Cargo.lock pinned. .env.example documented. No default credentials. |
| IR — Incident Response | Gap | No incident response plan in project. Deployer responsibility. |
| RM — Risk Management | Partial | SECURITY.md documents attack surfaces. No formal risk assessment. |
| CA — Security Assessment | Partial | Triple-sims automated testing. No penetration test report. |

## Summary

oakilydokily meets **CMMC Level 1** requirements for the application layer. Level 2 requires deployer-side controls (SIEM, incident response, penetration testing) that are outside the application boundary.

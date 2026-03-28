<!-- Unlicense — cochranblock.org -->

# FedRAMP Applicability Notes

> Last updated: 2026-03-27

## Deployment Model

**On-premise single binary.** Not a cloud service. Not SaaS.

oakilydokily is a self-contained Rust binary (9.1 MB) that runs on a single host. It embeds all assets, uses a local SQLite database, and requires no external infrastructure beyond optional email APIs.

## FedRAMP Applicability

**FedRAMP does not apply.** FedRAMP governs cloud service providers (CSPs) offering services to federal agencies. oakilydokily is:

- Not a cloud service
- Not multi-tenant
- Not offered as SaaS/PaaS/IaaS
- Deployed and operated by the entity that owns it

## Authorization Boundary

If deployed for a federal client, the authorization boundary is:

```
┌─────────────────────────────────────┐
│ Host machine (physical or VM)       │
│  ├─ oakilydokily binary             │
│  ├─ SQLite database (local disk)    │
│  ├─ Waiver archive (local disk)     │
│  └─ .env configuration              │
│                                      │
│  Outbound connections (optional):    │
│  ├─ Gmail API (email)               │
│  ├─ Resend API (email)              │
│  ├─ Cloudflare Turnstile (captcha)  │
│  └─ OAuth providers (Google/FB/Apple)│
└─────────────────────────────────────┘
```

All data at rest is on the host machine. No data stored in cloud services.

## If a Federal Agency Wants This as a Service

If oakilydokily were offered as a hosted service to federal agencies, FedRAMP authorization would be required. The path would be:
- **FedRAMP Li-SaaS** (Low Impact SaaS) — if the data is not sensitive
- **FedRAMP Low** — for public-facing waiver systems
- The authorization boundary would expand to include the hosting infrastructure

## Current Recommendation

Deploy on agency-owned infrastructure (on-prem or GovCloud VM). No FedRAMP authorization needed. The agency's existing ATO covers the host.

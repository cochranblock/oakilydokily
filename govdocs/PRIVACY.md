<!-- Unlicense — cochranblock.org -->

# Privacy Impact Assessment

> Last updated: 2026-03-27

## Data Collected

| Data | Where Stored | Purpose | Retention |
|------|-------------|---------|-----------|
| Full legal name | SQLite `waivers` table + gzip archive | Waiver signatory identification | 7 years |
| Email address | SQLite `waivers` table + gzip archive | Waiver confirmation email | 7 years |
| Typed signature | SQLite `waivers` table | ESIGN compliance — signature record | 7 years |
| IP address | SQLite `waivers` table + gzip archive | Audit trail for legal compliance | 7 years |
| User-Agent string | SQLite `waivers` table + gzip archive | Audit trail | 7 years |
| Timestamp | SQLite `waivers` table + gzip archive | ESIGN requirement | 7 years |
| Terms version + hash | SQLite `waivers` table + gzip archive | Prove which terms were signed | 7 years |
| Email (auth users) | SQLite `users` table | Account identification | Indefinite |
| Name (auth users) | SQLite `users` table | Display name | Indefinite |
| Password hash (bcrypt) | SQLite `users` table | Authentication | Indefinite |
| Session cookie | Browser (not server-stored) | Stateless auth via HMAC | 7 days (cookie expiry) |

## PII Classification

| Field | PII? | Sensitive? |
|-------|------|-----------|
| Full name | Yes | No |
| Email | Yes | No |
| IP address | Yes (per GDPR) | No |
| User-Agent | No (not PII alone) | No |
| Password hash | No (one-way) | Yes (must protect) |
| Typed signature | Yes | Yes |

## Storage Location

All data stored **on-premise** in the deployment host:
- SQLite database: `{data_dir}/waivers.sqlite`
- Gzip archives: `{data_dir}/waivers/*.gz`
- No cloud database. No external data store.
- No data leaves the server except: waiver confirmation emails (sent via Gmail API or Resend API)

## Data Minimization

- No cookies beyond session auth (no tracking cookies)
- No third-party analytics data collection (GA4 is optional, configured via env var)
- No user behavior tracking beyond standard HTTP logs
- Waiver form collects only legally required fields

## GDPR Applicability

Applicable if serving EU residents. Current mitigations:
- Data stored on-premise, not in cloud services
- Minimal data collection
- 7-year retention aligned with legal requirement (veterinary liability)
- **Gap**: No automated data deletion request endpoint. Manual process required.
- **Gap**: No cookie consent banner (only session cookie, not tracking)

## CCPA Applicability

Applicable if serving California residents. Current mitigations:
- No sale of personal information
- No sharing with third parties (except email delivery)
- **Gap**: No automated "Do Not Sell" or "Delete My Data" endpoint

## Data Flow

```
User → HTTPS → oakilydokily server → SQLite (local disk)
                                    → gzip archive (local disk)
                                    → Gmail API or Resend API (email only)
```

No data replication. No backup to cloud. No analytics pipeline.

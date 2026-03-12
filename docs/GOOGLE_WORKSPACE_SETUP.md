# Google Workspace Integration — oakilydokily.com

What oakilydokily needs for Google and where it lives.

---

## What Happened

The Google integration was **never wired into the deployed env**. The code exists (OAuth sign-in, Gmail for waiver emails), but the required env vars were never added to `approuter/.env` on gd.

- **oakilydokily** on gd loads `EnvironmentFile=/home/mcochran/cochranblock/approuter/.env`
- **approuter/.env** has Cloudflare vars only — no `GOOGLE_*` or `GMAIL_*`
- **approuter setup commands** (`setup-google-workspace`, `setup-google-sa`) are stubs — they return "not implemented"

Result: Sign in with Google is hidden (no button), and waiver emails fall back to Resend or are skipped.

---

## Required Env Vars

Add these to **approuter/.env** (or ensure they’re present on gd). oakilydokily reads them via `EnvironmentFile`.

### 1. Google OAuth (Sign in with Google)

| Var | Purpose |
|-----|---------|
| `GOOGLE_CLIENT_ID` | OAuth 2.0 Client ID from Google Cloud Console |
| `GOOGLE_CLIENT_SECRET` | OAuth 2.0 Client secret |
| `OD_BASE_URL` | `https://oakilydokily.com` (used for redirect URIs) |

Without these: "Sign in with Google" is hidden; `/auth/google` redirects to `/`.

### 2. Gmail API (waiver confirmation emails)

| Var | Purpose |
|-----|---------|
| `GOOGLE_APPLICATION_CREDENTIALS` | Path to service account JSON key file |
| `GMAIL_IMPERSONATE_USER` | Workspace user to impersonate (e.g. `byrdkaylie34@gmail.com`) |
| `GMAIL_FROM` | (Optional) Send-as alias, e.g. `OakilyDokily <noreply@oakilydokily.com>` |

Without these: falls back to Resend. If `RESEND_API_KEY` is also unset, waiver emails are skipped.

### 3. Resend fallback (if not using Gmail)

| Var | Purpose |
|-----|---------|
| `RESEND_API_KEY` | Resend API key |
| `RESEND_FROM` | `OakilyDokily <noreply@oakilydokily.com>` |

---

## Google Cloud Console Setup

### OAuth (Sign in)

1. [Google Cloud Console](https://console.cloud.google.com/) → APIs & Services → Credentials
2. Create **OAuth 2.0 Client ID** (Web application)
3. **Authorized redirect URIs** — add:
   - `https://oakilydokily.com/auth/google/callback`
   - `https://www.oakilydokily.com/auth/google/callback`
4. Copy Client ID and Client Secret into `GOOGLE_CLIENT_ID` and `GOOGLE_CLIENT_SECRET`

### Gmail (waiver emails)

1. Create a **service account** in the same project
2. Enable **domain-wide delegation** for the service account
3. In **Google Workspace Admin** → Security → API Controls → Domain-wide delegation:
   - Add the service account client ID
   - Scope: `https://www.googleapis.com/auth/gmail.send`
4. Download the JSON key, put it on gd (e.g. `/home/mcochran/.config/gcloud/oakilydokily-sa.json`)
5. Set `GOOGLE_APPLICATION_CREDENTIALS` to that path
6. Set `GMAIL_IMPERSONATE_USER` to the Workspace user (e.g. `byrdkaylie34@gmail.com`)

---

## Deploying the Vars

1. Add the vars to `approuter/.env` locally
2. Run `deploy-to-gd.sh` (copies approuter/.env to gd), **or**
3. Manually add them on gd: `ssh gd` and edit `/home/mcochran/cochranblock/approuter/.env`
4. Restart oakilydokily: `ssh gd "systemctl --user restart oakilydokily"`

---

## Verify

- **Sign in**: Visit https://oakilydokily.com/auth/login — "Sign in with Google" should appear
- **Waiver email**: Submit a waiver; check logs: `ssh gd "journalctl --user -u oakilydokily -n 50"` for "Waiver confirmation email sent via Gmail" or "via Resend"

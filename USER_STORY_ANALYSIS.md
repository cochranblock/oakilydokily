# OakilyDokily -- Full User Story Analysis

> Analyst perspective: real user, not developer. Brutal and honest.
> Date: 2026-03-27

---

## 1. DISCOVERY (README first impression)

**Can you tell what this product does in 10 seconds?** Barely.

The first 13 lines are a CochranBlock banner about the parent organization, zero-cloud architecture, and a tech intake form. A veterinary clinic owner scanning the README would see infrastructure bragging before learning what the product does. The actual one-liner -- "Veterinary professional services site with interactive mural, multi-auth, ESIGN-compliant waivers, and pixel forge sprite generation" -- is buried at line 22 and reads like a developer feature list, not a value proposition.

**First impression:** This is a developer portfolio project, not a product README aimed at a clinic owner or operator. The Mermaid architecture diagram is the second thing you see. No screenshots. No "here's what the site looks like" section. A paying client would close this tab.

**What's missing:**
- A screenshot or GIF of the live site
- A plain-English description ("OakilyDokily is a website for Kaylie Cochran's veterinary services business, including online waiver signing")
- Any indication of the target audience

---

## 2. INSTALLATION

**Build:** Passes cleanly. `cargo build --release -p oakilydokily --features approuter` completes in under a second (cached). Good.

**--help flag:** Does not exist. The binary supports exactly two undocumented subcommands (`hash-password` and `hash-email`) discovered only by reading source. Running the binary with no args starts the server. There is no usage message, no `--version`, no `--help`.

**.env.example:** Does not exist. The PRODUCTION.md doc lists env vars, but there is no `.env.example` file to copy. A new developer has to hunt through docs to figure out what to set.

**Required env vars to start:** Technically none -- the server starts with zero configuration. It will bind to `0.0.0.0:3000` and use a local `data/` directory. However, for any real functionality:
- `SESSION_SECRET` -- required for auth to work securely (falls back to empty HMAC key otherwise, which is a security concern)
- `OD_BASE_URL` -- needed for OAuth redirect URIs
- `GOOGLE_CLIENT_ID` / `GOOGLE_CLIENT_SECRET` -- for Google sign-in
- `RESEND_API_KEY` or `GOOGLE_APPLICATION_CREDENTIALS` -- for waiver confirmation emails

Without these, the site renders but auth and email silently fail. No warnings at startup about missing critical config.

---

## 3. FIRST USE (Happy Path)

### User visits homepage (GET /)
The page shows:
- An interactive WASM canvas mural (pixel art pets wandering around)
- "Serving Maryland -- Flexible: contract, temp, part-time"
- "OakilyDokily" heading
- Tagline about veterinary professional services
- CTAs: "Get in Touch" (mailto link), "Book a Call", "Services & Experience"
- Navigation: Home, About, Contact, Waiver, Sign in

**Impression:** Clean enough. The mural is a nice differentiator. The tagline is clear. But the mailto CTA goes to `byrdkaylie34@gmail.com` -- a personal Gmail, not a business email. That looks unprofessional for a paying client.

### User visits About (GET /about)
- Resume of Kaylie Cochran with 7+ years experience
- Work history from 2017-2025
- BLS certification
- Print Resume button

**Impression:** This is a resume page, not a services page. A potential client looking for "what services do you offer and what do they cost?" gets a job history instead. There are no service packages, no pricing, no testimonials, no photos of the team or facilities.

### User visits Contact (GET /contact)
- Simple page with "Email Us" button and "Book a Call" link
- "No form, no friction -- just email"
- "We typically respond within 24-48 hours"

**Impression:** Intentionally minimal. No contact form means no lead capture or data. The "no form, no friction" copy is developer thinking, not client thinking. Clients expect a form. The 24-48 hour response time is honest but slow for someone whose dog needs care.

### User visits Waiver (GET /waiver)
- Requires authentication first (redirects to /auth/login)
- User must sign in via Google, Facebook, Apple, or create an account with email/password
- After auth: three-step flow (Review > Sign > Complete)
- Shows full legal waiver text in a scrollable pre-formatted block
- Form fields: full legal name, email, consent checkboxes, typed signature
- Optional Cloudflare Turnstile captcha
- On success: redirects to confirmation page with reference ID
- Confirmation email sent via Gmail API or Resend

**Impression:** The waiver flow is solid and well-built. The step indicator is a nice touch. The legal text is real and covers liability properly. The ESIGN compliance (consent checkbox, typed signature, IP/UA logging, terms hash versioning) is genuine. This is the strongest part of the product.

---

## 4. EDGE CASES (Validation Analysis)

### f77 (validate_waiver_input) in src/waiver.rs:
- **Empty name:** Returns `Err("name empty")` -> HTTP 400 "Missing required fields". Good.
- **Name > 200 chars:** Returns `Err("name too long")` -> HTTP 400 "Invalid input length". Good.
- **Invalid email (no @):** Returns `Err("email invalid")`. The check is `!e.contains('@') || e.starts_with('@') || e.ends_with('@')`. Basic but functional. Does NOT validate format beyond requiring @ not at start/end. `a@b` passes. Acceptable for a waiver form.
- **Email > 254 chars:** Returns `Err("email too long")`. Good (RFC 5321 limit).

### f75 (post_waiver) in src/web/waiver.rs:
- **Missing checkboxes:** Returns HTTP 400 "You must consent to electronic signing and agree to the terms". Good.
- **Missing signature:** Returns HTTP 400 "Signature is required". Good.
- **Signature > 200 chars:** Returns HTTP 400 "Signature too long". Good.
- **Direct POST without auth:** Redirects to `/auth/login?redirect=/waiver`. Good.
- **Turnstile failure (when configured):** Returns HTTP 400 "Security check failed." Good.

### What's NOT validated:
- **Name vs signature mismatch:** The form asks you to "type your full legal name exactly" as signature, but there is no server-side check that signature matches full_name. You could sign as "Mickey Mouse" with name "John Smith".
- **Duplicate waivers:** No check for the same email signing multiple times. Could be intentional (re-signing on each visit) but there is no UI feedback about existing waivers.
- **Email deliverability:** No MX lookup or verification. Email confirmation is fire-and-forget -- if it fails, the waiver is still recorded but the user may not know.
- **Rate limiting:** No rate limiting on POST /waiver. Could be abused to fill the SQLite database.
- **CSRF protection:** No CSRF token on the form. Turnstile is optional and off by default. A malicious page could submit the form for an authenticated user.

**Error UX:** All error responses are plain text strings (not HTML pages). A user who fails validation sees a raw text "Missing required fields" with no page layout, no navigation, no way back except the browser back button. This is a poor experience.

---

## 5. FEATURE GAP (What a vet clinic client expects)

### Can they see their signed waivers?
**No.** After signing, the user gets a reference ID and is told "Contact us if you need a copy." There is no user portal, no waiver history, no way to view or re-download what they signed.

### Can they search past waivers?
**No.** There is no admin interface at all. The only way to find a waiver is to query the SQLite database directly or read the gzipped archive files on disk.

### Can they export/download waivers?
**No.** No PDF generation, no print-friendly waiver view, no download endpoint.

### Is there a dashboard?
**No.** No admin panel, no analytics beyond GA4, no way to see how many waivers have been signed, no staff management.

### Appointment scheduling?
**No.** The "Book a Call" button links to either an env-configured Calendly URL or a mailto fallback. There is no built-in scheduling. For a veterinary services business, this is a critical gap.

### Photo gallery of facilities?
**No.** The mural is art, not photos. There are no images of actual facilities, staff, or animals being cared for. The About page is a text resume with no photos.

### What else is missing:
- **Services and pricing page:** No list of what services cost
- **Testimonials or reviews:** No social proof
- **Service area map:** "Serving Maryland" but no specifics
- **Client portal:** No login-to-view-my-stuff for returning clients
- **Mobile app or SMS:** No text notifications
- **Multi-pet support on waivers:** The waiver does not capture pet name, species, or details
- **Insurance/licensing info:** No professional credentials displayed
- **Emergency contact info:** No after-hours number
- **FAQ section:** Common questions unanswered on the site

---

## 6. DOCUMENTATION

### .md files reviewed:
| File | Assessment |
|------|-----------|
| README.md | Developer-focused, missing user perspective, no screenshots |
| PRODUCTION.md | Good production checklist with env vars and OAuth setup |
| GOOGLE_WORKSPACE_SETUP.md | Detailed Gmail API setup (not reviewed in full) |
| compression_map.md | Internal dev reference, complete |
| PROOF_OF_ARTIFACTS.md | CochranBlock portfolio artifact (meta) |
| TIMELINE_OF_INVENTION.md | Development history (meta) |
| CONTRIBUTORS.md | Contributor list |
| CLAUDE.md | Build instructions (minimal) |
| docs/TEST_SIMULATIONS.md, TRIPLE_SIMS*.md, QUALITY_CHECKS.md | Test infrastructure docs |

### Unanswered questions:
1. How do I set up a development environment from scratch? (No dev setup guide)
2. Where is `.env.example`? (Does not exist)
3. What does the site look like? (No screenshots anywhere)
4. How do I back up the waiver database? (No backup docs)
5. How do I restore from the gzip archive files? (`archive_read` exists in code but no docs)
6. What happens when the SQLite file gets large? (No capacity planning)
7. How do I add a second staff member / admin? (No multi-user admin docs)
8. What are the legal requirements for ESIGN compliance in Maryland? (Waiver mentions 7-year retention but no legal review citation)
9. How do I run tests? (The `tests/` directory is empty; test binary exists but no docs on running it standalone)
10. What port does the WASM mural dev server use? (mural-wasm has its own README but build integration is unclear)

---

## 7. COMPETITOR CHECK

| Feature | OakilyDokily | PetDesk | Jane App | Timely | Basic Vet Website |
|---------|-------------|---------|----------|--------|-------------------|
| Online booking | No (mailto only) | Yes | Yes | Yes | Sometimes (widget) |
| Digital waivers | Yes (strong) | Yes | Yes | No | Rarely |
| Client portal | No | Yes | Yes | Yes | No |
| Pet profiles | No | Yes | Partial | No | No |
| Reminders/notifications | No | Yes (SMS + email) | Yes | Yes | No |
| Payment processing | No | Yes | Yes | Yes | No |
| Photo gallery | No (WASM mural) | No | No | No | Usually yes |
| Reviews/testimonials | No | Integrated | No | No | Usually yes |
| SEO / sitemap | Yes | N/A (SaaS) | N/A | N/A | Varies |
| Mobile responsive | Assumed yes (viewport meta) | Yes (app) | Yes | Yes | Varies |
| Multi-location | No | Yes | Yes | Yes | No |
| Pricing on site | No | N/A | N/A | N/A | Sometimes |

**OakilyDokily's unique advantage:** ESIGN-compliant waiver system with gzip archival, multi-auth (Google/Facebook/Apple/manual), and the pixel art mural. The tech stack is impressive (single binary, zero cloud).

**OakilyDokily's gap:** It is a brochure site with a waiver form. Competitors are full practice management platforms. For the target market (freelance vet tech offering services to clinics), the waiver system is relevant, but the rest of the site does not help close deals.

---

## 8. SCORING

| Category | Score | Notes |
|----------|-------|-------|
| **Usability** | 5/10 | Happy path works. Error states return raw text. No client portal. No way to retrieve signed waivers. |
| **Completeness** | 4/10 | Waiver flow is complete. Everything else is a brochure. No scheduling, no client management, no admin panel. |
| **Error Handling** | 6/10 | Server-side validation is thorough. But errors render as plain text without navigation. No CSRF token. No rate limiting. |
| **Documentation** | 5/10 | PRODUCTION.md is good. No .env.example. No screenshots. No dev setup guide. Empty tests/ directory. |
| **Would-You-Pay-For-This** | 3/10 | As a vet clinic owner: the waiver system has value, but I would need scheduling, a client portal, and an admin dashboard before paying. As a freelance vet tech: it is a personal website with a waiver form -- I would need it to actually book me work. |

**Overall: 4.6/10**

---

## 9. TOP 3 FIXES (Biggest Impact for a Paying Client)

### 1. Add an admin dashboard and waiver viewer
**Why:** Right now, signed waivers go into SQLite and gzip files with no way to access them except the command line. A clinic operator needs to look up a client's waiver by name or email, verify it was signed, and optionally download/print a PDF copy. This is table stakes for any waiver system. Without it, the waiver feature is half-built.

**What to build:**
- GET /admin/waivers -- paginated list of signed waivers (name, email, date, ref ID)
- GET /admin/waivers/:id -- single waiver detail view
- Search by name or email
- PDF or print-friendly export
- Protect behind admin auth (env-configured admin email list)

### 2. Return proper HTML error pages instead of raw text
**Why:** Every validation failure in the waiver flow (`f75`) returns a bare string like "Missing required fields" with no page layout. The user loses all navigation and has to hit the browser back button. This makes the product feel broken. Every competitor returns styled error pages.

**What to build:**
- Wrap all error responses in the same HTML shell (head, nav, footer) used by other pages
- Show the error inline on the waiver form (re-render the form with error messages above the fields, preserving entered data)
- This is a 1-2 hour fix that dramatically improves perceived quality

### 3. Add a services and pricing page with photos
**Why:** The About page is a resume. A client visiting the site wants to know: what do you do, what does it cost, and can I see your work? Right now there are zero photos of actual animals, facilities, or the person providing services. The site has a pixel art mural but no real-world images. Adding a services page with packages/pricing and a gallery of real photos would convert the site from a developer portfolio into something that could actually generate business.

**What to build:**
- GET /services -- list of service offerings (overnight care, kennel ops, surgical support, etc.) with descriptions and starting prices
- Photo gallery section (even 6-8 real photos of animals/facilities)
- Testimonials or client references section
- Consider replacing the About page resume with a more client-facing "Our Team" page

---

## Files Referenced

- `/Users/mcochran/oakilydokily/README.md` -- Product README
- `/Users/mcochran/oakilydokily/src/main.rs` -- Entry point, env var handling
- `/Users/mcochran/oakilydokily/src/lib.rs` -- AppState definition
- `/Users/mcochran/oakilydokily/src/waiver.rs` -- Waiver DB, validation (f77), archive
- `/Users/mcochran/oakilydokily/src/web/pages.rs` -- Home, about, contact pages
- `/Users/mcochran/oakilydokily/src/web/waiver.rs` -- Waiver form GET/POST (f74, f75)
- `/Users/mcochran/oakilydokily/src/web/auth.rs` -- Multi-provider auth
- `/Users/mcochran/oakilydokily/src/web/email.rs` -- Gmail API + Resend email
- `/Users/mcochran/oakilydokily/src/web/head.rs` -- Nav, GA4, shared HTML helpers
- `/Users/mcochran/oakilydokily/src/web/router.rs` -- Route definitions
- `/Users/mcochran/oakilydokily/content/resume.html` -- Resume content
- `/Users/mcochran/oakilydokily/content/waiver_terms.txt` -- Legal waiver text
- `/Users/mcochran/oakilydokily/docs/PRODUCTION.md` -- Production deployment checklist
- `/Users/mcochran/oakilydokily/docs/compression_map.md` -- Token mapping reference

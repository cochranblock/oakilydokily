// Unlicense — cochranblock.org
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

//! /govdocs — serve federal compliance docs at runtime. The binary IS the compliance artifact.

use axum::response::Html;

const SBOM: &str = include_str!("../../govdocs/SBOM.md");
const SECURITY: &str = include_str!("../../govdocs/SECURITY.md");
const SSDF: &str = include_str!("../../govdocs/SSDF.md");
const SUPPLY_CHAIN: &str = include_str!("../../govdocs/SUPPLY_CHAIN.md");
const PRIVACY: &str = include_str!("../../govdocs/PRIVACY.md");
const FIPS: &str = include_str!("../../govdocs/FIPS.md");
const FEDRAMP: &str = include_str!("../../govdocs/FedRAMP_NOTES.md");
const CMMC: &str = include_str!("../../govdocs/CMMC.md");
const ITAR_EAR: &str = include_str!("../../govdocs/ITAR_EAR.md");
const ACCESSIBILITY: &str = include_str!("../../govdocs/ACCESSIBILITY.md");
const FEDERAL_USE: &str = include_str!("../../govdocs/FEDERAL_USE_CASES.md");

fn render(title: &str, md: &str) -> Html<String> {
    let escaped = md
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;");
    Html(format!(
        r#"<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>{} | OakilyDokily</title><style>body{{font-family:system-ui,sans-serif;background:#0f1419;color:#e6edf3;max-width:800px;margin:0 auto;padding:2rem 1rem;line-height:1.6}}pre{{white-space:pre-wrap;word-wrap:break-word;background:#161b22;padding:1.5rem;border-radius:8px;border:1px solid rgba(168,85,247,0.25);overflow-x:auto}}a{{color:#a855f7}}h1{{color:#e6edf3;border-bottom:1px solid rgba(168,85,247,0.25);padding-bottom:0.5rem}}h2{{color:#a855f7}}h3{{color:#ec4899}}.nav{{margin-bottom:2rem;font-size:0.9rem}}.nav a{{margin-right:1rem}}</style></head><body><div class="nav"><a href="/govdocs">Index</a> <a href="/">Home</a></div><pre>{}</pre></body></html>"#,
        title, escaped
    ))
}

/// GET /govdocs — index of all compliance documents
pub async fn index() -> Html<String> {
    Html(format!(
        r#"<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Federal Compliance | OakilyDokily</title><style>body{{font-family:system-ui,sans-serif;background:#0f1419;color:#e6edf3;max-width:800px;margin:0 auto;padding:2rem 1rem;line-height:1.6}}a{{color:#a855f7;text-decoration:none}}a:hover{{text-decoration:underline}}h1{{border-bottom:1px solid rgba(168,85,247,0.25);padding-bottom:0.5rem}}li{{margin:0.5rem 0}}.nav{{margin-bottom:2rem;font-size:0.9rem}}.nav a{{margin-right:1rem}}</style></head><body><div class="nav"><a href="/">Home</a></div><h1>Federal Compliance Documents</h1><p>The binary IS the compliance artifact. These documents are embedded at compile time.</p><ul><li><a href="/govdocs/sbom">SBOM</a> — Software Bill of Materials (EO 14028)</li><li><a href="/govdocs/security">Security Posture</a></li><li><a href="/govdocs/ssdf">SSDF</a> — NIST SP 800-218</li><li><a href="/govdocs/supply-chain">Supply Chain Integrity</a></li><li><a href="/govdocs/privacy">Privacy Impact Assessment</a></li><li><a href="/govdocs/fips">FIPS 140-2/3 Status</a></li><li><a href="/govdocs/fedramp">FedRAMP Notes</a></li><li><a href="/govdocs/cmmc">CMMC Level 1-2</a></li><li><a href="/govdocs/itar-ear">ITAR/EAR Export Control</a></li><li><a href="/govdocs/accessibility">Section 508 / WCAG</a></li><li><a href="/govdocs/federal-use-cases">Federal Use Cases</a></li></ul><p><small>Binary: oakilydokily v{}</small></p></body></html>"#,
        env!("CARGO_PKG_VERSION")
    ))
}

pub async fn sbom() -> Html<String> { render("SBOM", SBOM) }
pub async fn security() -> Html<String> { render("Security", SECURITY) }
pub async fn ssdf() -> Html<String> { render("SSDF", SSDF) }
pub async fn supply_chain() -> Html<String> { render("Supply Chain", SUPPLY_CHAIN) }
pub async fn privacy() -> Html<String> { render("Privacy", PRIVACY) }
pub async fn fips() -> Html<String> { render("FIPS", FIPS) }
pub async fn fedramp() -> Html<String> { render("FedRAMP", FEDRAMP) }
pub async fn cmmc() -> Html<String> { render("CMMC", CMMC) }
pub async fn itar_ear() -> Html<String> { render("ITAR/EAR", ITAR_EAR) }
pub async fn accessibility() -> Html<String> { render("Accessibility", ACCESSIBILITY) }
pub async fn federal_use_cases() -> Html<String> { render("Federal Use Cases", FEDERAL_USE) }

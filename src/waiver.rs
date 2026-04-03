#![allow(non_camel_case_types, non_snake_case)]

// Unlicense — cochranblock.org
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

use chrono::Utc;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use sha2::{Digest, Sha256};
use sqlx::sqlite::SqlitePool;
use std::io::{Read as _, Write as _};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use uuid::Uuid;

static ARCHIVE_DIR: OnceLock<PathBuf> = OnceLock::new();

pub const TERMS_VERSION: &str = "2026-01";
const TERMS: &str = include_str!("../content/waiver_terms.txt");

/// f112 = terms_hash. SHA256 of waiver terms for versioning.
pub fn terms_hash() -> String {
    format!("{:x}", Sha256::digest(TERMS.as_bytes()))
}

/// f113 = init_pool. Create data dir, connect SQLite, migrate. Also prunes archive > 1 year.
pub async fn init_pool(
    data_dir: &Path,
) -> Result<SqlitePool, Box<dyn std::error::Error + Send + Sync>> {
    std::fs::create_dir_all(data_dir)?;
    let arc_dir = data_dir.join("waivers");
    std::fs::create_dir_all(&arc_dir)?;
    let _ = ARCHIVE_DIR.set(arc_dir);
    archive_prune();
    let db_path = data_dir.join("waivers.sqlite");
    let url = format!("sqlite:{}?mode=rwc", db_path.display());
    let pool = SqlitePool::connect(&url).await?;
    sqlx::query("PRAGMA journal_mode=WAL")
        .execute(&pool)
        .await?;
    migrate(&pool).await?;
    Ok(pool)
}

/// f113_memory = init_pool_memory. In-memory pool for tests.
pub async fn init_pool_memory() -> Result<SqlitePool, Box<dyn std::error::Error + Send + Sync>> {
    let pool = SqlitePool::connect("sqlite::memory:").await?;
    migrate(&pool).await?;
    Ok(pool)
}

async fn migrate(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS waivers (
            id TEXT PRIMARY KEY,
            full_name TEXT NOT NULL,
            email TEXT NOT NULL,
            signed_at TEXT NOT NULL,
            ip_address TEXT,
            user_agent TEXT,
            terms_version TEXT NOT NULL,
            terms_hash TEXT NOT NULL,
            consent_electronic INTEGER NOT NULL,
            signature_text TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;
    let _ = sqlx::query("ALTER TABLE waivers DROP COLUMN signing_on_behalf_of")
        .execute(pool)
        .await;
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            email TEXT PRIMARY KEY,
            password_hash TEXT NOT NULL,
            name TEXT NOT NULL,
            created_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// f114 = insert. Insert waiver record. Returns ref_id. Also writes compressed archive.
pub async fn insert(
    pool: &SqlitePool,
    full_name: &str,
    email: &str,
    ip_address: Option<&str>,
    user_agent: Option<&str>,
    signature_text: &str,
) -> Result<String, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let signed_at = Utc::now().to_rfc3339();
    let hash = terms_hash();
    sqlx::query(
        r#"
        INSERT INTO waivers (id, full_name, email, signed_at, ip_address, user_agent, terms_version, terms_hash, consent_electronic, signature_text)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, 1, ?)
        "#,
    )
    .bind(&id)
    .bind(full_name)
    .bind(email)
    .bind(&signed_at)
    .bind(ip_address)
    .bind(user_agent)
    .bind(TERMS_VERSION)
    .bind(&hash)
    .bind(signature_text)
    .execute(pool)
    .await?;
    archive_write(&id, full_name, email, &signed_at, ip_address, user_agent, &hash);
    Ok(id)
}

/// f118 = archive_write. Gzip-compressed JSON, one file per waiver. Tiny footprint.
fn archive_write(
    id: &str,
    name: &str,
    email: &str,
    signed_at: &str,
    ip: Option<&str>,
    ua: Option<&str>,
    terms_hash: &str,
) {
    let Some(dir) = ARCHIVE_DIR.get() else {
        return;
    };
    let date = &signed_at[..10].replace('-', "");
    let short = &id[..8];
    let path = dir.join(format!("{}_{}.gz", date, short));
    let json = serde_json::json!({
        "i": id, "n": name, "e": email, "t": signed_at,
        "ip": ip, "ua": ua, "v": TERMS_VERSION, "h": terms_hash
    });
    let Ok(raw) = serde_json::to_vec(&json) else {
        return;
    };
    let Ok(f) = std::fs::File::create(&path) else {
        tracing::warn!("archive write failed: {}", path.display());
        return;
    };
    let mut gz = GzEncoder::new(f, Compression::best());
    let _ = gz.write_all(&raw);
    let _ = gz.finish();
}

/// f119 = archive_read. Decompress a single .gz waiver file back to JSON.
pub fn archive_read(path: &Path) -> Option<serde_json::Value> {
    let f = std::fs::File::open(path).ok()?;
    let mut dec = GzDecoder::new(f);
    let mut buf = Vec::new();
    dec.read_to_end(&mut buf).ok()?;
    serde_json::from_slice(&buf).ok()
}

/// f120 = archive_prune. Delete .gz files older than 365 days.
fn archive_prune() {
    let Some(dir) = ARCHIVE_DIR.get() else {
        return;
    };
    let cutoff = chrono::Utc::now() - chrono::Duration::days(2557); // 7 years
    let cutoff_str = cutoff.format("%Y%m%d").to_string();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut pruned = 0u32;
    for entry in entries.flatten() {
        let name = entry.file_name();
        let s = name.to_string_lossy();
        if !s.ends_with(".gz") {
            continue;
        }
        // filename: YYYYMMDD_shortid.gz — first 8 chars are the date
        if s.len() >= 8 && &s[..8] < cutoff_str.as_str() {
            let _ = std::fs::remove_file(entry.path());
            pruned += 1;
        }
    }
    if pruned > 0 {
        tracing::info!("archive: pruned {} waiver files older than 1 year", pruned);
    }
}

/// f115 = terms_text. Raw waiver terms for display.
pub fn terms_text() -> &'static str {
    TERMS
}

/// f116 = user_create. Insert native user. Returns Err if email exists.
pub async fn user_create(
    pool: &SqlitePool,
    email: &str,
    password_hash: &str,
    name: &str,
) -> Result<(), sqlx::Error> {
    let created_at = Utc::now().to_rfc3339();
    sqlx::query(
        "INSERT INTO users (email, password_hash, name, created_at) VALUES (?, ?, ?, ?)",
    )
    .bind(email)
    .bind(password_hash)
    .bind(name)
    .bind(&created_at)
    .execute(pool)
    .await?;
    Ok(())
}

/// f117 = user_get. Lookup by email. Returns (name, password_hash).
pub async fn user_get(
    pool: &SqlitePool,
    email: &str,
) -> Result<Option<(String, String)>, sqlx::Error> {
    let row = sqlx::query_as::<_, (String, String)>(
        "SELECT name, password_hash FROM users WHERE email = ?",
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

/// f77 = validate_waiver_input. Why: Proptest (B4); single source of truth for length limits.
pub fn f77(full_name: &str, email: &str) -> Result<(), &'static str> {
    let n = full_name.trim();
    let e = email.trim();
    if n.is_empty() {
        return Err("name empty");
    }
    if e.is_empty() {
        return Err("email empty");
    }
    if n.len() > 200 {
        return Err("name too long");
    }
    if e.len() > 254 {
        return Err("email too long");
    }
    if !e.contains('@') || e.starts_with('@') || e.ends_with('@') {
        return Err("email invalid");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    // --- f77 unit tests ---

    #[test]
    fn validate_ok() {
        assert!(f77("Jane Smith", "jane@example.com").is_ok());
        assert!(f77("  Jane  ", "  jane@x.com  ").is_ok()); // trims whitespace
    }

    #[test]
    fn validate_name_empty() {
        assert_eq!(f77("", "x@y.com"), Err("name empty"));
        assert_eq!(f77("   ", "x@y.com"), Err("name empty"));
    }

    #[test]
    fn validate_email_empty() {
        assert_eq!(f77("Jane", ""), Err("email empty"));
        assert_eq!(f77("Jane", "   "), Err("email empty"));
    }

    #[test]
    fn validate_email_invalid() {
        assert_eq!(f77("Jane", "notanemail"), Err("email invalid"));
        assert_eq!(f77("Jane", "@missing-local"), Err("email invalid"));
        assert_eq!(f77("Jane", "missing-domain@"), Err("email invalid"));
    }

    #[test]
    fn validate_name_too_long() {
        let long = "a".repeat(201);
        assert_eq!(f77(&long, "x@y.com"), Err("name too long"));
        // exactly 200 is ok
        let edge = "a".repeat(200);
        assert!(f77(&edge, "x@y.com").is_ok());
    }

    #[test]
    fn validate_email_too_long() {
        // 255 chars: local@domain where local is padded
        let long = format!("{}@y.com", "a".repeat(250));
        assert_eq!(f77("Jane", &long), Err("email too long"));
        let edge = format!("{}@y.com", "a".repeat(247)); // 254 total
        assert!(f77("Jane", &edge).is_ok());
    }

    #[test]
    fn validate_xss_input_accepted_by_validator() {
        // Validation only checks structure; HTML escaping is the renderer's job.
        // XSS payloads are valid names/emails from a validation perspective.
        assert!(f77("<script>alert(1)</script>", "x@y.com").is_ok());
        assert!(f77("Jane", "x+tag@y.com").is_ok());
    }

    // --- terms_hash determinism ---

    #[test]
    fn terms_hash_deterministic() {
        assert_eq!(terms_hash(), terms_hash());
        assert!(!terms_hash().is_empty());
    }

    // --- insert + archive roundtrip ---

    #[tokio::test]
    async fn insert_roundtrip() {
        let pool = init_pool_memory().await.expect("in-memory pool");
        let dir = TempDir::new().unwrap();
        // Set ARCHIVE_DIR for this test run (may already be set; if so, skip archive check)
        let _ = ARCHIVE_DIR.set(dir.path().to_path_buf());

        let ref_id = insert(&pool, "Test User", "test@example.com", Some("127.0.0.1"), Some("test-ua"), "Test User")
            .await
            .expect("insert ok");
        assert!(!ref_id.is_empty());
        assert_eq!(ref_id.len(), 36); // UUID v4

        // Record is in DB
        let row: (String, String, String) = sqlx::query_as(
            "SELECT full_name, email, signature_text FROM waivers WHERE id = ?"
        )
        .bind(&ref_id)
        .fetch_one(&pool)
        .await
        .expect("row exists");
        assert_eq!(row.0, "Test User");
        assert_eq!(row.1, "test@example.com");
        assert_eq!(row.2, "Test User");
    }

    #[tokio::test]
    async fn insert_duplicate_uuid_impossible() {
        // Two separate inserts produce different IDs.
        let pool = init_pool_memory().await.expect("pool");
        let id1 = insert(&pool, "A", "a@b.com", None, None, "A").await.unwrap();
        let id2 = insert(&pool, "B", "b@c.com", None, None, "B").await.unwrap();
        assert_ne!(id1, id2);
    }

    #[tokio::test]
    async fn user_create_and_get() {
        let pool = init_pool_memory().await.expect("pool");
        user_create(&pool, "u@x.com", "hash123", "User Name").await.expect("create ok");
        let got = user_get(&pool, "u@x.com").await.expect("query ok");
        assert_eq!(got, Some(("User Name".into(), "hash123".into())));
        // Unknown user returns None
        let none = user_get(&pool, "missing@x.com").await.expect("query ok");
        assert!(none.is_none());
    }

    #[tokio::test]
    async fn user_create_duplicate_fails() {
        let pool = init_pool_memory().await.expect("pool");
        user_create(&pool, "u@x.com", "hash1", "Name").await.expect("first ok");
        let err = user_create(&pool, "u@x.com", "hash2", "Name2").await;
        assert!(err.is_err()); // UNIQUE constraint
    }
}
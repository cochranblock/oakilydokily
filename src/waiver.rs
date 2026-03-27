#![allow(non_camel_case_types, non_snake_case, dead_code)]

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
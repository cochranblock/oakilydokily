#![allow(non_camel_case_types, non_snake_case)]

// Unlicense — cochranblock.org
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

#[cfg(feature = "approuter")]
use approuter_client::{f116, RegisterConfig};
use oakilydokily::web::router;
use oakilydokily::{waiver, AppState};
use std::path::{Path, PathBuf};

/// PID lockfile path: ~/.local/share/oakilydokily/pid
fn pid_path() -> PathBuf {
    dirs::data_dir()
        .map(|p| p.join("oakilydokily").join("pid"))
        .unwrap_or_else(|| PathBuf::from("/tmp/oakilydokily.pid"))
}

/// Read old PID from lockfile. Returns None if file missing or unreadable.
fn read_old_pid(path: &Path) -> Option<u32> {
    std::fs::read_to_string(path)
        .ok()
        .and_then(|s| s.trim().parse().ok())
}

/// Write our PID to lockfile.
fn write_pid(path: &Path) {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(path, format!("{}", std::process::id()));
}

/// SIGTERM the old process, wait up to 5s, SIGKILL if needed.
fn kill_old(pid: u32) {
    use nix::sys::signal::{kill, Signal};
    use nix::unistd::Pid;

    let nix_pid = Pid::from_raw(pid as i32);

    // Check if process is alive
    if kill(nix_pid, None).is_err() {
        tracing::info!("hot-reload: old pid {} not running", pid);
        return;
    }

    // Don't kill ourselves
    if pid == std::process::id() {
        return;
    }

    tracing::info!("hot-reload: SIGTERM → pid {}", pid);
    let _ = kill(nix_pid, Signal::SIGTERM);

    // Wait up to 5 seconds for graceful shutdown
    for _ in 0..50 {
        std::thread::sleep(std::time::Duration::from_millis(100));
        if kill(nix_pid, None).is_err() {
            tracing::info!("hot-reload: pid {} exited gracefully", pid);
            return;
        }
    }

    // Force kill
    tracing::warn!("hot-reload: SIGKILL → pid {} (did not exit in 5s)", pid);
    let _ = kill(nix_pid, Signal::SIGKILL);
    std::thread::sleep(std::time::Duration::from_millis(200));
}

/// Create a SO_REUSEPORT TCP listener via socket2, then convert to tokio.
fn reuseport_listener(addr: &str) -> std::io::Result<std::net::TcpListener> {
    let sock_addr: std::net::SocketAddr = addr.parse().map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, e)
    })?;
    let domain = if sock_addr.is_ipv4() {
        socket2::Domain::IPV4
    } else {
        socket2::Domain::IPV6
    };
    let socket = socket2::Socket::new(domain, socket2::Type::STREAM, Some(socket2::Protocol::TCP))?;
    socket.set_reuse_address(true)?;
    socket.set_reuse_port(true)?;
    socket.set_nonblocking(true)?;
    socket.bind(&sock_addr.into())?;
    socket.listen(1024)?;
    Ok(std::net::TcpListener::from(socket))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if std::env::args().nth(1).as_deref() == Some("hash-password") {
        let pw = std::env::args().nth(2).unwrap_or_else(|| {
            eprintln!("Usage: oakilydokily hash-password PASSWORD");
            std::process::exit(1);
        });
        let hash = bcrypt::hash(&pw, bcrypt::DEFAULT_COST).expect("bcrypt hash");
        println!("{}", hash);
        return Ok(());
    }
    if std::env::args().nth(1).as_deref() == Some("hash-email") {
        let email = std::env::args().nth(2).unwrap_or_else(|| {
            eprintln!("Usage: oakilydokily hash-email EMAIL");
            std::process::exit(1);
        });
        let h = oakilydokily::web::auth::hash_email(&email);
        println!("{}", h);
        return Ok(());
    }
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    // Fail fast: SESSION_SECRET must be 32+ chars if any auth provider is configured
    let has_auth = ["GOOGLE_CLIENT_ID", "FB_APP_ID", "APPLE_CLIENT_ID", "OD_MANUAL_USERS"]
        .iter()
        .any(|k| std::env::var(k).map(|v| !v.trim().is_empty()).unwrap_or(false));
    if has_auth {
        match std::env::var("SESSION_SECRET") {
            Ok(s) if s.len() >= 32 => {}
            Ok(s) => {
                tracing::error!("SESSION_SECRET is {} chars, need 32+. Auth will not work.", s.len());
                std::process::exit(1);
            }
            Err(_) => {
                tracing::error!("SESSION_SECRET not set. Required when auth providers are configured.");
                std::process::exit(1);
            }
        }
    }

    let data_dir = std::env::var("OAKILYDOKILY_DATA_DIR").unwrap_or_else(|_| {
        std::env::var("COCHRANBLOCK_DATA_ROOT")
            .ok()
            .map(|r| format!("{}/oakilydokily", r.trim_end_matches('/')))
            .or_else(|| {
                dirs::data_dir()
                    .map(|p| p.join("cochranblock").join("oakilydokily"))
                    .and_then(|p| p.to_str().map(String::from))
            })
            .unwrap_or_else(|| "data".into())
    });
    let pool = waiver::init_pool(Path::new(&data_dir)).await?;
    tracing::info!("waiver DB initialized at {}/waivers.sqlite", data_dir);

    let d1 = oakilydokily::d1_auth::f80_from_env();
    if d1.is_some() {
        tracing::info!("auth: D1 sharded storage enabled ({} shards)", d1.as_ref().map(|c| c.shard_count()).unwrap_or(0));
    }

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);
    let bind = std::env::var("BIND").unwrap_or_else(|_| "0.0.0.0".into());
    let addr = format!("{}:{}", bind, port);

    // --- Hot reload: bind with SO_REUSEPORT BEFORE killing old instance ---
    let std_listener = reuseport_listener(&addr)?;
    tracing::info!("hot-reload: bound {} with SO_REUSEPORT", addr);

    // Register with approuter (new instance takes traffic)
    let rate_limiter = std::sync::Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::new()));
    // Prune expired rate limit entries every 60s
    {
        let rl = rate_limiter.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(60)).await;
                let now = std::time::Instant::now();
                let mut map = rl.lock().await;
                map.retain(|_ip: &std::net::IpAddr, entries: &mut Vec<std::time::Instant>| {
                    entries.retain(|t| now.duration_since(*t).as_secs() < 60);
                    !entries.is_empty()
                });
            }
        });
    }
    let app = router::router(AppState {
        s0: pool,
        s1: d1,
        s2: oakilydokily::web::forge::new_cache(),
        s3: rate_limiter,
    });
    #[cfg(feature = "approuter")]
    f116(RegisterConfig {
        app_id: "oakilydokily",
        hostnames: std::env::var("OD_HOSTNAMES")
            .unwrap_or_else(|_| {
                "oakilydokily.com,www.oakilydokily.com".into()
            })
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect(),
        backend_url: std::env::var("OD_BACKEND_URL")
            .unwrap_or_else(|_| format!("http://127.0.0.1:{}", port)),
    })
    .await;

    // --- Kill old instance AFTER we're bound and registered ---
    let pid_file = pid_path();
    if let Some(old_pid) = read_old_pid(&pid_file) {
        kill_old(old_pid);
    }
    write_pid(&pid_file);
    tracing::info!("hot-reload: pid {} written to {}", std::process::id(), pid_file.display());

    // Convert to tokio listener and serve
    let listener = tokio::net::TcpListener::from_std(std_listener)?;
    tracing::info!("oakilydokily listening on http://{}", addr);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await?;
    Ok(())
}

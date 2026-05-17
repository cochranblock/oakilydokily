// Unlicense — public domain
//! Daily-rolling log appender. Logs land at:
//!   ~/.local/share/{project}/logs/current.YYYY-MM-DD.log
//!
//! Retention: 30 rolled files. Drop-in init returns a guard that must be
//! kept alive for the lifetime of the process; dropping it flushes the
//! async writer.

use std::path::PathBuf;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub struct Guard {
    _g: WorkerGuard,
}

pub fn logs_dir(project: &str) -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(project)
        .join("logs")
}

/// Initialise tracing with daily-rolling file + stderr writer. Default filter
/// "info"; override with RUST_LOG.
pub fn init(project: &str) -> std::io::Result<Guard> {
    let dir = logs_dir(project);
    std::fs::create_dir_all(&dir)?;

    let file = rolling::Builder::new()
        .rotation(rolling::Rotation::DAILY)
        .filename_prefix("current")
        .filename_suffix("log")
        .max_log_files(30)
        .build(&dir)
        .map_err(|e| std::io::Error::other(e.to_string()))?;

    let (nb_file, guard) = tracing_appender::non_blocking(file);

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_writer(std::io::stderr).with_target(true))
        .with(
            fmt::layer()
                .with_writer(nb_file)
                .with_ansi(false)
                .with_target(true),
        )
        .init();

    Ok(Guard { _g: guard })
}

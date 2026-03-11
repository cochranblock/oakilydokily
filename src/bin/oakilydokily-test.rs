// Unlicense — cochranblock.org
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
//! f70=oakilydokily_test. TRIPLE SIMS via exopack::triple_sims::f60. f30=run_tests. f53=screenshot. f62=console check.

use std::process::Command;
use std::time::Duration;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let port = 3000u16;
    let exe = std::env::current_exe().unwrap_or_else(|_| std::path::PathBuf::from("oakilydokily-test"));
    let oakily_bin = exe.parent().unwrap().join("oakilydokily");
    let data_dir = std::env::temp_dir().join("oakilydokily-test");
    let _ = std::fs::create_dir_all(&data_dir);

    let mut child = if oakily_bin.exists() {
        Some(
            Command::new(&oakily_bin)
                .env("PORT", port.to_string())
                .env("BIND", "127.0.0.1")
                .env("OAKILYDOKILY_DATA_DIR", data_dir)
                .env("BASE", format!("http://127.0.0.1:{}", port))
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
                .expect("spawn oakilydokily"),
        )
    } else {
        eprintln!("oakilydokily binary not found at {} (build with: cargo build -p oakilydokily --features approuter)", oakily_bin.display());
        std::process::exit(1);
    };

    tokio::time::sleep(Duration::from_secs(3)).await;

    std::env::set_var("BASE", format!("http://127.0.0.1:{}", port));
    let _ = oakilydokily::screenshot::f53().await;
    let console_errors = oakilydokily::screenshot::f62().await;
    if !console_errors.is_empty() {
        for e in &console_errors {
            eprintln!("console: {}", e);
        }
        std::process::exit(1);
    }
    let ok = exopack::triple_sims::f60(|| async {
        oakilydokily::tests::f30().await == 0
    })
    .await;

    if let Some(ref mut c) = child {
        let _ = c.kill();
        let _ = c.wait();
    }
    std::process::exit(if ok { 0 } else { 1 });
}

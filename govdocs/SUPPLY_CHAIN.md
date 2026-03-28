<!-- Unlicense — cochranblock.org -->

# Supply Chain Integrity

> Last updated: 2026-03-27

## Dependency Sources

All dependencies sourced from **crates.io** (Rust package registry) except:
- `approuter-client`: GitHub (cochranblock/approuter) — same organization
- `exopack`: GitHub (cochranblock/exopack) — same organization, test-only

## Verification

- **Cargo.lock**: Pinned in version control. Every dependency version and checksum is deterministic.
- **crates.io checksums**: Cargo verifies SHA-256 checksums on download.
- **No vendored binaries**: All code compiled from source. SQLite compiled from C source via libsqlite3-sys bundled feature.

## Build Reproducibility

- Rust edition 2021, stable toolchain
- `Cargo.lock` committed — identical inputs produce identical dependency resolution
- Release profile: `lto = true`, `codegen-units = 1` — deterministic optimizations
- No build scripts that fetch from network (except initial crate download)

## No Pre-Built Binaries

- WASM mural binary (`mural-wasm.wasm`) is built from source in `mural-wasm/` crate
- Static assets embedded via `rust-embed` at compile time from `assets/` directory
- No npm, no node_modules, no JavaScript build pipeline

## Source Availability

- Application source: https://github.com/cochranblock/oakilydokily (public, Unlicense)
- All first-party dependencies: public GitHub repositories under cochranblock org
- All third-party dependencies: public crates.io packages

## Audit Trail

```bash
# Verify dependency tree
cargo tree -p oakilydokily --features approuter

# Check for known vulnerabilities
cargo audit

# Verify Cargo.lock integrity
cargo verify-project
```

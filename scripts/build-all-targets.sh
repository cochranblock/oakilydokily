#!/bin/bash
# Unlicense — cochranblock.org
# Build oakilydokily for every supported target.
# Output: release/oakilydokily-<target>
#
# Native builds (on this Mac):
#   aarch64-apple-darwin    macOS ARM64
#   x86_64-apple-darwin     macOS Intel
#   aarch64-apple-ios       iOS (staticlib)
#
# Remote builds (SSH to worker nodes):
#   x86_64-unknown-linux-gnu    Linux x86_64 (on st/gd/lf)
#
# Cross builds (via cross crate):
#   aarch64-unknown-linux-gnu        Linux ARM64 (RPi 4/5, Graviton)
#   armv7-unknown-linux-gnueabihf    Linux ARM 32-bit (older RPi, IoT)
#   x86_64-pc-windows-gnu            Windows x86_64 (MinGW)
#   riscv64gc-unknown-linux-gnu      RISC-V 64-bit
#   x86_64-unknown-freebsd           FreeBSD x86_64
#   powerpc64le-unknown-linux-gnu    IBM POWER
#
# Android (via cargo-ndk):
#   aarch64-linux-android    Android ARM64

set -euo pipefail
cd "$(dirname "$0")/.."

source ~/.cargo/env 2>/dev/null || true

OUT="release"
mkdir -p "$OUT"
PKG="oakilydokily"
FEATURES="--features approuter"

ok=0
fail=0

build_native() {
  local target=$1
  echo "--- $target (native) ---"
  if cargo build --release -p "$PKG" $FEATURES --target "$target" 2>&1; then
    cp "target/$target/release/$PKG" "$OUT/$PKG-$target"
    echo "OK: $OUT/$PKG-$target ($(ls -lh "$OUT/$PKG-$target" | awk '{print $5}'))"
    ((ok++))
  else
    echo "FAIL: $target"
    ((fail++))
  fi
}

build_remote() {
  local target=$1
  local host=$2
  echo "--- $target (remote: $host) ---"
  rsync -az --delete \
    --exclude='target/' --exclude='mural-wasm/target/' \
    --exclude='assets-archive/' --exclude='data/' --exclude='release/' \
    ./ "$host:~/oakilydokily-build/" 2>&1
  if ssh "$host" "cd ~/oakilydokily-build && \
    echo '[workspace]' >> Cargo.toml && \
    source ~/.cargo/env && \
    cargo build --release -p $PKG $FEATURES 2>&1 | tail -3 && \
    head -n -1 Cargo.toml > Cargo.toml.tmp && mv Cargo.toml.tmp Cargo.toml" 2>&1; then
    scp "$host:~/oakilydokily-build/target/release/$PKG" "$OUT/$PKG-$target"
    echo "OK: $OUT/$PKG-$target ($(ls -lh "$OUT/$PKG-$target" | awk '{print $5}'))"
    ((ok++))
  else
    echo "FAIL: $target"
    ((fail++))
  fi
}

build_cross() {
  local target=$1
  echo "--- $target (cross) ---"
  if command -v cross >/dev/null 2>&1; then
    if cross build --release -p "$PKG" $FEATURES --target "$target" 2>&1; then
      cp "target/$target/release/$PKG" "$OUT/$PKG-$target" 2>/dev/null || \
      cp "target/$target/release/$PKG.exe" "$OUT/$PKG-$target.exe" 2>/dev/null
      echo "OK: $OUT/$PKG-$target"
      ((ok++))
    else
      echo "FAIL: $target (cross build failed)"
      ((fail++))
    fi
  else
    echo "SKIP: $target (cross not installed — cargo install cross)"
    ((fail++))
  fi
}

build_android() {
  echo "--- aarch64-linux-android (cargo-ndk) ---"
  if command -v cargo-ndk >/dev/null 2>&1 && [ -n "${ANDROID_NDK_HOME:-}" ]; then
    if cargo ndk --target aarch64-linux-android --platform 35 build --release --lib 2>&1; then
      cp "target/aarch64-linux-android/release/lib${PKG}.so" "$OUT/lib${PKG}-aarch64-linux-android.so" 2>/dev/null
      echo "OK: $OUT/lib${PKG}-aarch64-linux-android.so"
      ((ok++))
    else
      echo "FAIL: android"
      ((fail++))
    fi
  else
    echo "SKIP: android (cargo-ndk or ANDROID_NDK_HOME not set)"
    ((fail++))
  fi
}

echo "=== oakilydokily multi-arch build ==="
echo ""

# Native targets (Mac)
build_native aarch64-apple-darwin
rustup target list --installed | grep -q x86_64-apple-darwin && build_native x86_64-apple-darwin || echo "SKIP: x86_64-apple-darwin (target not installed)"

# Remote Linux
build_remote x86_64-unknown-linux-gnu st

# Cross targets (if cross is installed)
for t in aarch64-unknown-linux-gnu armv7-unknown-linux-gnueabihf x86_64-pc-windows-gnu riscv64gc-unknown-linux-gnu x86_64-unknown-freebsd powerpc64le-unknown-linux-gnu; do
  build_cross "$t"
done

# Android
build_android

echo ""
echo "=== Results: $ok succeeded, $fail failed/skipped ==="
echo ""
ls -lhS "$OUT/$PKG-"* 2>/dev/null

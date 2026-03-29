<!-- Unlicense — cochranblock.org -->

# OakilyDokily Pocket Server (Android)

Your website runs on your phone. No hosting bill. No cloud.

## Architecture

```
┌─────────────────────────────┐
│ Android App                  │
│  ├─ ServerService (foreground)│
│  │   └─ liboakilydokily.so  │
│  │       └─ Rust axum server │
│  │           └─ :3000        │
│  └─ MainActivity (WebView)   │
│      └─ http://127.0.0.1:3000│
└─────────────────────────────┘
```

The Rust server binary runs as a native library inside a foreground service.
A WebView points to localhost. Zero JavaScript. Server-rendered HTML.

## Prerequisites

```bash
cargo install cargo-ndk
rustup target add aarch64-linux-android
export ANDROID_NDK_HOME=~/Library/Android/sdk/ndk/27.0.12077973
export ANDROID_HOME=~/Library/Android/sdk
```

## Build

```bash
./android/build-apk.sh
```

Output: `android/build/oakilydokily.apk`

## Install

```bash
adb install android/build/oakilydokily.apk
```

## Known Build Requirements

- `libsqlite3-sys` bundled: needs NDK C compiler (set by cargo-ndk)
- `ring`: needs NDK assembler for aarch64 crypto
- API level 26+ (Android 8.0) for foreground service
- Target API 35 for Play Store compliance

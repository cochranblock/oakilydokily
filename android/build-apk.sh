#!/bin/bash
# Unlicense — cochranblock.org
# Build oakilydokily Android APK (Pocket Server)
#
# Prerequisites:
#   cargo install cargo-ndk
#   rustup target add aarch64-linux-android
#   ANDROID_NDK_HOME set (e.g. ~/Library/Android/sdk/ndk/27.0.12077973)
#   ANDROID_HOME set (e.g. ~/Library/Android/sdk)
#
# Usage: ./build-apk.sh

set -euo pipefail
cd "$(dirname "$0")/.."

echo "=== oakilydokily Pocket Server APK build ==="

# Check prerequisites
command -v cargo-ndk >/dev/null 2>&1 || { echo "cargo-ndk not found. Install: cargo install cargo-ndk"; exit 1; }
[ -n "${ANDROID_NDK_HOME:-}" ] || { echo "ANDROID_NDK_HOME not set"; exit 1; }
[ -n "${ANDROID_HOME:-}" ] || { echo "ANDROID_HOME not set"; exit 1; }

TARGET=aarch64-linux-android
API=35
LIB_NAME=liboakilydokily.so

echo "--- Step 1: Build Rust shared library for $TARGET (API $API) ---"
cargo ndk \
  --target $TARGET \
  --platform $API \
  build --release --lib \
  2>&1

RUST_LIB="target/$TARGET/release/$LIB_NAME"
if [ ! -f "$RUST_LIB" ]; then
  echo "ERROR: $RUST_LIB not found"
  exit 1
fi
echo "Rust lib: $(ls -lh "$RUST_LIB" | awk '{print $5}') ($RUST_LIB)"

echo "--- Step 2: Package APK ---"
APK_DIR="android/build"
rm -rf "$APK_DIR"
mkdir -p "$APK_DIR/lib/arm64-v8a"

# Copy native lib
cp "$RUST_LIB" "$APK_DIR/lib/arm64-v8a/$LIB_NAME"

# Compile Java
PLATFORM_JAR="$ANDROID_HOME/platforms/android-$API/android.jar"
[ -f "$PLATFORM_JAR" ] || { echo "Android platform $API not installed"; exit 1; }

JAVA_SRC="android/app/src/main/java/org/oakilydokily"
CLASSES_DIR="$APK_DIR/classes"
mkdir -p "$CLASSES_DIR"

javac -source 11 -target 11 \
  -classpath "$PLATFORM_JAR" \
  -d "$CLASSES_DIR" \
  "$JAVA_SRC/MainActivity.java" \
  "$JAVA_SRC/ServerService.java" \
  2>&1

# Convert to DEX
D8="$ANDROID_HOME/build-tools/35.0.0/d8"
[ -f "$D8" ] || D8=$(find "$ANDROID_HOME/build-tools" -name d8 | sort -V | tail -1)
"$D8" --output "$APK_DIR" "$CLASSES_DIR/org/oakilydokily/"*.class 2>&1

# Build APK with aapt2
AAPT2="$ANDROID_HOME/build-tools/35.0.0/aapt2"
[ -f "$AAPT2" ] || AAPT2=$(find "$ANDROID_HOME/build-tools" -name aapt2 | sort -V | tail -1)

# Compile resources
"$AAPT2" compile --dir android/app/src/main/res -o "$APK_DIR/res.zip" 2>&1

# Link
"$AAPT2" link \
  -I "$PLATFORM_JAR" \
  --manifest android/app/src/main/AndroidManifest.xml \
  -o "$APK_DIR/oakilydokily-unsigned.apk" \
  "$APK_DIR/res.zip" \
  --auto-add-overlay \
  2>&1

# Add DEX and native lib to APK
cd "$APK_DIR"
zip -r oakilydokily-unsigned.apk classes.dex lib/ 2>&1
cd -

# Align
ZIPALIGN="$ANDROID_HOME/build-tools/35.0.0/zipalign"
[ -f "$ZIPALIGN" ] || ZIPALIGN=$(find "$ANDROID_HOME/build-tools" -name zipalign | sort -V | tail -1)
"$ZIPALIGN" -f 4 "$APK_DIR/oakilydokily-unsigned.apk" "$APK_DIR/oakilydokily-aligned.apk"

# Sign (debug key for now)
APKSIGNER="$ANDROID_HOME/build-tools/35.0.0/apksigner"
[ -f "$APKSIGNER" ] || APKSIGNER=$(find "$ANDROID_HOME/build-tools" -name apksigner | sort -V | tail -1)

DEBUG_KEYSTORE="$HOME/.android/debug.keystore"
if [ ! -f "$DEBUG_KEYSTORE" ]; then
  keytool -genkey -v -keystore "$DEBUG_KEYSTORE" -storepass android \
    -alias androiddebugkey -keypass android -keyalg RSA -keysize 2048 \
    -validity 10000 -dname "CN=Debug,O=Android,C=US"
fi

"$APKSIGNER" sign \
  --ks "$DEBUG_KEYSTORE" \
  --ks-pass pass:android \
  --key-pass pass:android \
  --out "$APK_DIR/oakilydokily.apk" \
  "$APK_DIR/oakilydokily-aligned.apk" \
  2>&1

FINAL="$APK_DIR/oakilydokily.apk"
echo ""
echo "=== BUILD COMPLETE ==="
echo "APK: $FINAL"
echo "Size: $(ls -lh "$FINAL" | awk '{print $5}')"
echo ""
echo "Install: adb install $FINAL"

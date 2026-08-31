#!/usr/bin/env bash
# Generates a local, self-signed keystore for signing release-mode Android
# builds during development -- gen/android/app/build.gradle.kts's `release`
# buildType has no signingConfig otherwise, so `assembleRelease`/
# `tauri android build` produces an *unsigned* APK, which real devices
# refuse to install ("App not installed"). Debug builds don't need this --
# the Android Gradle Plugin auto-signs those with its own debug keystore.
#
# This is a throwaway test-signing key, not a Play Store upload key: don't
# use it for a real release. Re-running this script after deleting the
# keystore (or with --force) invalidates any existing install signed with
# the old one -- Android refuses to install an update whose signature
# doesn't match what's already on the device, so uninstall the app first
# after regenerating.
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")"

android_dir="src-tauri/gen/android"
keystore="$android_dir/app/release-keystore.jks"
props="$android_dir/keystore.properties"

if ! command -v keytool >/dev/null 2>&1; then
    echo "error: keytool not found -- install a JDK (e.g. \`brew install openjdk\`)" >&2
    exit 1
fi

force=0
if [[ "${1:-}" == "--force" ]]; then
    force=1
fi

if [[ -f "$keystore" || -f "$props" ]]; then
    if [[ "$force" -ne 1 ]]; then
        echo "error: $keystore or $props already exists -- pass --force to regenerate" >&2
        echo "       (this changes the app's signature; uninstall any existing install first)" >&2
        exit 1
    fi
    rm -f "$keystore" "$props"
fi

mkdir -p "$(dirname "$keystore")"

# PKCS12 (keytool's default keystore type since JDK 9) only supports one
# password for both the store and every key in it -- passing a separate
# -keypass just gets silently ignored, so there's only one password here.
storepass=$(openssl rand -base64 33 | tr -dc 'A-Za-z0-9' | head -c 32)

keytool -genkeypair -v \
    -keystore "$keystore" \
    -alias rumad2-release \
    -keyalg RSA -keysize 2048 -validity 10000 \
    -storepass "$storepass" \
    -dname "CN=RUMAD 2 Dev, OU=Dev, O=RUMAD 2, L=Mayaguez, S=PR, C=US"

cat >"$props" <<EOF
storeFile=app/release-keystore.jks
storePassword=$storepass
keyAlias=rumad2-release
keyPassword=$storepass
EOF

echo "Wrote $keystore and $props (gitignored -- see gen/android/.gitignore and app/.gitignore)."

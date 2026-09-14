#!/usr/bin/env bash
# Generates the keypair that signs updater artifacts (see
# `bundle.createUpdaterArtifacts` in src-tauri/tauri.conf.json). Run this
# once; the private key never leaves your machine except as GitHub secrets.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

key_path="updater.key"

if [[ -f "$key_path" && "${1:-}" != "--force" ]]; then
  echo "error: $key_path already exists. Pass --force to overwrite (this invalidates updates signed with the old key)." >&2
  exit 1
fi

pnpm tauri signer generate ${1:-} -w "$key_path"

cat <<EOF

Generated $key_path (private, gitignored) and $key_path.pub (public).

Next steps:
  1. Add these as GitHub Actions repo secrets (Settings > Secrets and
     variables > Actions), so the build workflow can sign updater artifacts:
       TAURI_SIGNING_PRIVATE_KEY          = contents of $key_path
       TAURI_SIGNING_PRIVATE_KEY_PASSWORD = the password you just entered
  2. When the updater plugin is wired into the app, put the contents of
     $key_path.pub into tauri.conf.json's plugins.updater.pubkey.
  3. Keep a backup of $key_path somewhere safe -- losing it means you can
     never ship a signed update again under this pubkey.
EOF

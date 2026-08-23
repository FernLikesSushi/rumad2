#!/usr/bin/env bash
set -euo pipefail

# Same Linux dependency list as .github/workflows/build.yml's Tauri build
# job -- these are what webkit2gtk/appindicator-backed Tauri needs to
# compile on Linux, not editor tooling.
sudo apt-get update
sudo apt-get install -y \
  libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev

# Picks up the pnpm version pinned in package.json's "packageManager" field.
corepack enable

pnpm install

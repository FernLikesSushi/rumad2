#!/bin/zsh
# Xcode Cloud post-clone hook. The base image has neither Rust nor Node,
# both of which the generated Xcode project needs before it can build:
# the "Build Rust Code" phase shells out to `pnpm tauri ios xcode-script`,
# and the app's `assets` folder reference is a symlink to the repo's
# `dist/`, which only exists after a frontend build.
# Unverified against a real Xcode Cloud run -- no Mac available to test
# this against locally.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")" && git rev-parse --show-toplevel)"

# ~/.cargo/bin isn't on the PATH Xcode's own Run Script build phases use,
# so symlink into /usr/local/bin (which is) instead of relying on shell
# profile sourcing.
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
export PATH="$HOME/.cargo/bin:$PATH"
rustup target add aarch64-apple-ios aarch64-apple-ios-sim
ln -sf "$HOME/.cargo/bin/cargo" /usr/local/bin/cargo
ln -sf "$HOME/.cargo/bin/rustc" /usr/local/bin/rustc

# Homebrew's prefix is already on Xcode Cloud's PATH, so anything
# installed through it (unlike rustup above) needs no symlink trick.
brew install node
PNPM_VERSION="$(node -e "console.log(require('$REPO_ROOT/package.json').packageManager.split('@')[1])")"
corepack enable
corepack prepare "pnpm@${PNPM_VERSION}" --activate

cd "$REPO_ROOT"
pnpm install --frozen-lockfile
pnpm build

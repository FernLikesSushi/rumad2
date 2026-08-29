#!/bin/sh
# `-e`: without it, a failed `docker build` still left `docker create`/
# `docker cp` running against whatever image was tagged
# `tauri-windows-builder` from a *previous* successful run -- silently
# copying stale artifacts out and reporting success on a build that
# actually failed (confirmed live: a NASM-missing build failure still
# produced a "Successfully copied" `dist`).
set -e

docker build -f Dockerfile.win32 -t tauri-windows-builder .
docker create --name tauri-build tauri-windows-builder
docker cp tauri-build:/nsis ./dist
docker rm tauri-build

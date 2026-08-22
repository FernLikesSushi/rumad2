docker build -f Dockerfile.win32 -t tauri-windows-builder .
docker create --name tauri-build tauri-windows-builder
docker cp tauri-build:/app/src-tauri/target/x86_64-pc-windows-gnu/release/bundle/nsis ./dist
docker rm tauri-build
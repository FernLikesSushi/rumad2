// swift-tools-version:5.3
// A Tauri iOS plugin is packaged as a Swift Package (SPM), not a plain
// source file the Xcode project just happens to compile -- this manifest
// is what makes `Sources/` below a real library target Xcode can depend
// on. `cargo tauri ios init`/`dev`/`build` is what actually adds this
// package as a local dependency of the generated `gen/apple` Xcode
// project (see that project's `project.yml`) once this plugin is a path
// dependency in the main crate's `Cargo.toml` -- nothing in this file
// does that wiring itself, matching how `android/build.gradle.kts`
// doesn't add itself to `gen/android/settings.gradle.kts` either.
import PackageDescription

let package = Package(
    name: "tauri-plugin-native-map",
    platforms: [
        .iOS(.v13)
    ],
    products: [
        .library(
            name: "tauri-plugin-native-map",
            type: .static,
            targets: ["tauri-plugin-native-map"])
    ],
    dependencies: [
        // `Tauri` here is the iOS runtime's own Swift package -- every
        // Tauri iOS plugin depends on it the same way every Rust plugin
        // crate here depends on the `tauri` crate. This relative path is
        // the one part of this file most likely to need adjusting: it's
        // generated/managed by the Tauri CLI relative to wherever it
        // places this plugin inside `gen/apple` once `cargo tauri ios
        // init` actually runs -- if the build can't resolve "Tauri",
        // check what path the CLI generated and fix this to match (same
        // caveat as `android/build.gradle.kts`'s `project(":tauri-
        // android")` needing `cargo tauri android init` re-run first).
        .package(name: "Tauri", path: "../.tauri/tauri-api")
    ],
    targets: [
        .target(
            name: "tauri-plugin-native-map",
            dependencies: [
                .byName(name: "Tauri")
            ],
            path: "Sources")
    ]
)

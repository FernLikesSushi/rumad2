plugins {
    id("com.android.library")
    id("org.jetbrains.kotlin.android")
}

android {
    namespace = "me.fern.rumad2.nativemap"
    // Keep in sync with `gen/android/app/build.gradle.kts` (the CLI
    // doesn't do this for you across separate Gradle modules).
    compileSdk = 36

    defaultConfig {
        minSdk = 24
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }

    kotlinOptions {
        jvmTarget = "1.8"
    }
}

dependencies {
    implementation("androidx.appcompat:appcompat:1.7.1")
    implementation("androidx.webkit:webkit:1.14.0")
    // Pin as needed -- check https://developers.google.com/android/guides/setup
    // for the current release; this was the latest known-good at write time.
    implementation("com.google.android.gms:play-services-maps:19.1.0")
    // Provided by the generated Android project once `cargo tauri android
    // init`/`dev`/`build` re-scans Cargo.toml and wires this module into
    // `settings.gradle.kts` -- not resolvable until that's re-run.
    implementation(project(":tauri-android"))
}

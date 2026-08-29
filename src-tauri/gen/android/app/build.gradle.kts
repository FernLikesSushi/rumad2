import java.util.Properties

plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("rust")
}

val tauriProperties = Properties().apply {
    val propFile = file("tauri.properties")
    if (propFile.exists()) {
        propFile.inputStream().use { load(it) }
    }
}

// Reuses the same key as the web embed (`VITE_GOOGLE_MAPS_API_KEY` in the
// repo root `.env`, gitignored) rather than a separate Android-only key --
// simpler to manage, at the cost of the key's "Application restriction"
// having to stay off (or cover only one of the two use cases): a single
// key can't be *both* HTTP-referrer-restricted (for the iframe embed) and
// Android-app-restricted (for this native SDK) at once. `API restriction`
// (scoping the key to just Maps Embed + Maps SDK for Android) plus a
// Cloud Console budget alert is the real protection here instead.
val dotenv = Properties().apply {
    // repo root: gen/android -> gen -> src-tauri -> root
    val envFile = rootProject.file("../../../.env")
    if (envFile.exists()) {
        envFile.inputStream().use { load(it) }
    }
}
val mapsApiKey: String = dotenv.getProperty("VITE_GOOGLE_MAPS_API_KEY")
    ?: System.getenv("VITE_GOOGLE_MAPS_API_KEY")
    ?: ""

android {
    compileSdk = 36
    namespace = "me.fern.rumad2"
    defaultConfig {
        manifestPlaceholders["usesCleartextTraffic"] = "false"
        manifestPlaceholders["MAPS_API_KEY"] = mapsApiKey
        applicationId = "me.fern.rumad2"
        minSdk = 24
        targetSdk = 36
        versionCode = tauriProperties.getProperty("tauri.android.versionCode", "1").toInt()
        versionName = tauriProperties.getProperty("tauri.android.versionName", "1.0")
    }
    buildTypes {
        getByName("debug") {
            manifestPlaceholders["usesCleartextTraffic"] = "true"
            isDebuggable = true
            isJniDebuggable = true
            isMinifyEnabled = false
            packaging {                jniLibs.keepDebugSymbols.add("*/arm64-v8a/*.so")
                jniLibs.keepDebugSymbols.add("*/armeabi-v7a/*.so")
                jniLibs.keepDebugSymbols.add("*/x86/*.so")
                jniLibs.keepDebugSymbols.add("*/x86_64/*.so")
            }
        }
        getByName("release") {
            isMinifyEnabled = true
            proguardFiles(
                *fileTree(".") { include("**/*.pro") }
                    .plus(getDefaultProguardFile("proguard-android-optimize.txt"))
                    .toList().toTypedArray()
            )
        }
    }
    kotlinOptions {
        jvmTarget = "1.8"
    }
    buildFeatures {
        buildConfig = true
    }
}

rust {
    rootDirRel = "../../../"
}

dependencies {
    implementation("androidx.webkit:webkit:1.14.0")
    implementation("androidx.appcompat:appcompat:1.7.1")
    implementation("androidx.activity:activity-ktx:1.10.1")
    implementation("com.google.android.material:material:1.12.0")
    implementation("androidx.lifecycle:lifecycle-process:2.10.0")
    testImplementation("junit:junit:4.13.2")
    androidTestImplementation("androidx.test.ext:junit:1.1.4")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.5.0")
}

apply(from = "tauri.build.gradle.kts")
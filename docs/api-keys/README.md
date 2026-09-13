# API keys & signing credentials

Reference for every third-party key or signing credential the build can use,
how to obtain it, and which CI secret it maps to. The build workflow
currently runs without most of these -- builds are unsigned/debug and the
map embed's key is blank -- so nothing here is required to get a green
build. Add a credential (and wire the matching secret into the workflow)
only once that platform's build actually needs to be signed/fully
functional.

| Credential | Used for | Guide |
| --- | --- | --- |
| Google Maps Platform API key | `VITE_GOOGLE_MAPS_API_KEY` -- the desktop map iframe embed and the Android native Maps SDK view | [google-maps.md](google-maps.md) |
| Android release keystore | Signing `tauri android build --apk`/`--aab` so devices/Play Console accept the artifact | [android-signing.md](android-signing.md) |
| Apple Developer certificate + team | Signing `tauri ios build` so a device/TestFlight/App Store accepts the artifact | [apple-signing.md](apple-signing.md) |
| Windows code signing certificate | Replacing the workflow's throwaway self-signed MSIX cert so Windows installs it without a manual "trust this publisher" step | [windows-signing.md](windows-signing.md) |

Each guide ends with how to store that credential as a Codeberg Actions
secret: repo -> **Settings -> Actions -> Secrets** -> **Add Secret**, pasting
the value in directly.

# Android release keystore

Signs `pnpm tauri android build --apk`/`--aab` release output. Without it,
`gen/android/app/build.gradle.kts`'s `release` build type has no
`signingConfig`, producing an unsigned APK real devices refuse to install
("App not installed") and that the Play Console won't accept either.
Debug builds (what `build.yml`'s `build-mobile` job currently runs, via
`--debug`) don't need this -- the Android Gradle Plugin auto-signs those
with its own throwaway debug keystore.

## Obtaining it

There's no external account/portal for this one -- you generate the
keystore yourself. The repo already has a script for it:

```sh
./android_keystore.sh
```

This creates `src-tauri/gen/android/app/release-keystore.jks` and
`src-tauri/gen/android/keystore.properties` (both gitignored -- see
`gen/android/.gitignore`/`app/.gitignore`), with a random 32-character
store/key password. Read the script's own header comment before running it
against a device you care about: re-running with `--force` invalidates any
existing install signed with the old key (Android refuses to install an
update whose signature doesn't match what's already on the device).

This is a self-signed test key, not a Play Store upload key. If/when this
app is actually published to the Play Store, use Play App Signing instead
(Play Console generates and holds the real signing key; this local keystore
just needs to be consistent for your own upload key in the interim) --
Google's own
[app signing docs](https://developer.android.com/studio/publish/app-signing)
cover that flow.

## Storing it

`build-mobile`'s existing job doesn't consume any of this yet, it only runs
`--debug` builds. To sign a release build in CI, base64-encode the
keystore --

```sh
base64 -w0 src-tauri/gen/android/app/release-keystore.jks
```

-- and store four Codeberg Actions secrets (**Settings -> Actions ->
Secrets -> Add Secret**, one per name below):

- `ANDROID_KEYSTORE_BASE64` -- that base64 output.
- `ANDROID_KEYSTORE_PASSWORD` -- the `storePassword` from
  `keystore.properties`.
- `ANDROID_KEY_ALIAS` -- `rumad2-release`.
- `ANDROID_KEY_PASSWORD` -- the `keyPassword` from `keystore.properties`
  (identical to `storePassword` here, since PKCS12 keystores only support
  one password for both).

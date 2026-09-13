# Google Maps Platform API key

Powers two things off the same key (see `src-tauri/gen/android/app/build.gradle.kts`'s
comment for why it's one key, not two):

- `src/components/geo/GoogleMapWeb.tsx`'s `<iframe>` embed (desktop/web) --
  the **Maps Embed API**.
- `src-tauri/plugins/tauri-plugin-native-map/`'s Android view -- the
  **Maps SDK for Android**.

## Obtaining it

1. Go to the [Google Cloud Console](https://console.cloud.google.com/) and
   create a project (or reuse an existing one) for this app.
2. **APIs & Services -> Library**: enable both:
   - `Maps Embed API`
   - `Maps SDK for Android`
3. **APIs & Services -> Credentials -> Create credentials -> API key.**
4. Restrict the key (**Edit API key**):
   - **API restrictions**: select the two APIs enabled above, nothing else.
   - **Application restrictions**: the web embed needs an *HTTP referrer*
     restriction and the Android SDK needs an *Android app* restriction
     (package name `me.fern.rumad2` + the SHA-1 of whichever keystore
     signs that build -- see [android-signing.md](android-signing.md)), but
     a single key can only carry one application-restriction type. Sharing
     one key across both (this project's current setup) means leaving
     **Application restrictions** off and relying on the **API
     restriction** above plus a Cloud Console budget/quota alert instead.
     Two separate keys (one per restriction type) is the alternative if
     that tradeoff isn't acceptable.
5. Copy the generated key.

## Storing it

Locally: put it in the repo-root `.env` (gitignored, see `.env.example`) as
`VITE_GOOGLE_MAPS_API_KEY`.

For CI: add it as a Codeberg Actions secret named `VITE_GOOGLE_MAPS_API_KEY`
(**Settings -> Actions -> Secrets -> Add Secret**), value the key itself.
The workflow doesn't currently read this secret at all (there's no `.env`
in CI, so the embed/native map just render with a blank key).
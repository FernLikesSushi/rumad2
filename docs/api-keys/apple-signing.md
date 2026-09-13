# Apple Developer certificate & team

Signs `pnpm tauri ios build` for a real device, TestFlight, or App Store
target. `build.yml`'s `build-mobile` job currently runs
`tauri ios build --no-sign` instead (`continue-on-error: true`, since it's
unverified against a real macOS/Xcode run) -- good enough to prove the app
still builds, not something installable outside the simulator.

## Obtaining it

1. Enroll in the [Apple Developer Program](https://developer.apple.com/programs/)
   (paid, individual or organization account).
2. **Find your Team ID**: [developer.apple.com/account](https://developer.apple.com/account) ->
   Membership details -> Team ID (a 10-character alphanumeric string). This
   is what `TAURI_APPLE_DEVELOPMENT_TEAM` needs.
3. **Create a signing certificate**: Xcode -> Settings -> Accounts -> your
   team -> Manage Certificates -> + -> "Apple Distribution" (App
   Store/TestFlight) or "Apple Development" (device testing only). Xcode
   generates the private key and certificate together.
4. **Export it as a `.p12`**: Keychain Access -> My Certificates -> find
   the certificate Xcode just created -> right-click -> Export -> save as
   `.p12` with a password. This bundles the private key, which is why it
   needs a password and must never be committed.
5. **Provisioning profile**: usually not needed as a separate secret --
   Xcode's "Automatically manage signing" (which `tauri ios build` relies
   on by default) fetches/creates one from the account the certificate
   came from, as long as the CI keychain has that certificate installed.

## Storing it

Base64-encode the `.p12` --

```sh
base64 -w0 DistributionCert.p12
```

-- and store three Codeberg Actions secrets (**Settings -> Actions ->
Secrets -> Add Secret**):

- `APPLE_CERTIFICATE_BASE64` -- that base64 output.
- `APPLE_CERTIFICATE_PASSWORD` -- the `.p12` export password.
- `APPLE_TEAM_ID` -- the 10-character Team ID.

`TAURI_APPLE_DEVELOPMENT_TEAM` (the env var the Tauri CLI itself reads) is
just `APPLE_TEAM_ID`'s value under a different name -- keep both in mind if
a workflow step ends up naming it either way.

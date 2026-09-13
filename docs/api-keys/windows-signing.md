# Windows code signing certificate

Signs the MSIX package `build.yml`'s "Package MSIX" step hand-assembles.
Right now it
generates a throwaway self-signed certificate on every run purely so
`makeappx`/`signtool` have something to sign with -- Windows refuses to
install an *unsigned* MSIX outright, but it still only installs a
*self-signed* one for someone who's explicitly imported and trusted
`rumad-2-ci.cer` first (also uploaded as a workflow artifact/release
asset). The plain `.msi`/`.exe` from `tauri-action` don't need this at all;
they install fine unsigned, just with an unsigned-publisher SmartScreen
warning.

## Obtaining a real certificate

Two options, in order of how this project is likely to actually use them:

1. **A commercial code signing certificate** (OV or EV) from a CA the
   Microsoft trust store recognizes -- e.g.
   [DigiCert](https://www.digicert.com/signing/code-signing-certificates),
   [Sectigo](https://sectigo.com/ssl-certificates-tls/code-signing), or
   similar. Requires an organization/individual identity-verification
   process with the CA (can take a few days) and is a recurring paid cost.
   EV certificates additionally ship on a hardware token (USB HSM) rather
   than an exportable file, which complicates using them from a hosted CI
   runner -- OV is the more CI-friendly of the two.
2. **[Azure Trusted Signing](https://learn.microsoft.com/en-us/azure/trusted-signing/)**
   -- Microsoft's newer signing-as-a-service offering. No local private
   key/token to manage (signing happens via an Azure API call), billed
   per subscription rather than per certificate, and has an
   [official GitHub Action](https://github.com/Azure/trusted-signing-action)
   already built for CI use. Currently requires a verified Azure account
   tied to a registered business (not available for pure hobby/individual
   use as of this writing) -- worth rechecking eligibility if that's a
   blocker.

## Storing it

**Commercial certificate (OV, exportable `.pfx`)**: base64-encode it --

```sh
base64 -w0 CodeSigningCert.pfx
```

-- and store two Codeberg Actions secrets (**Settings -> Actions ->
Secrets -> Add Secret**):

- `WINDOWS_CERTIFICATE_BASE64` -- that base64 output.
- `WINDOWS_CERTIFICATE_PASSWORD` -- the `.pfx` export password.

**Azure Trusted Signing**: no certificate file/secret at all -- store the
Azure credentials instead: `AZURE_TENANT_ID`, `AZURE_CLIENT_ID`,
`AZURE_CLIENT_SECRET`, plus the Trusted Signing account/endpoint/
certificate-profile names, each as its own secret.

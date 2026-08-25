#!/usr/bin/env bash
# Builds the iOS app and installs it on every connected, paired physical
# device. Devices are scanned via `xcrun devicectl list devices` each run
# rather than a hardcoded UDID, so it works on whatever's plugged in.
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")"

pnpm tauri ios build

ipa=$(find src-tauri/gen/apple/build -name '*.ipa' -print -quit)
if [[ -z "$ipa" ]]; then
    echo "error: no .ipa found under src-tauri/gen/apple/build" >&2
    exit 1
fi
echo "Built: $ipa"

devices_json=$(mktemp)
trap 'rm -f "$devices_json"' EXIT
xcrun devicectl list devices --json-output "$devices_json" >/dev/null

udids=()
while IFS= read -r udid; do
    [[ -n "$udid" ]] && udids+=("$udid")
done < <(jq -r '
  .result.devices[]
  | select(.hardwareProperties.platform == "iOS")
  | select(.connectionProperties.pairingState == "paired")
  | select(.connectionProperties.tunnelState == "connected")
  | .hardwareProperties.udid
' "$devices_json")

if [[ ${#udids[@]} -eq 0 ]]; then
    echo "error: no connected, paired iOS devices found" >&2
    exit 1
fi

for udid in "${udids[@]}"; do
    name=$(jq -r --arg udid "$udid" '
      .result.devices[] | select(.hardwareProperties.udid == $udid) | .deviceProperties.name
    ' "$devices_json")
    echo "Installing on $name ($udid)..."
    xcrun devicectl device install app --device "$udid" "$ipa"
done

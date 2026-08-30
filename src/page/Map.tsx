import { useParams } from "@solidjs/router";
import { Header } from "../components/Header";
import { createMemo, Show } from "solid-js";
import { RoomCodes } from "../data/roomMap";
import { GoogleMap } from "../components/geo/GoogleMapWeb";
import { MapViewNative } from "../components/geo/MapNative";
import { isMobile } from "../data/platform";
import { openUrl } from "@tauri-apps/plugin-opener";
import { t } from "../i18n";

// Universal links -- each opens the platform's own app if it's installed
// (that's what `openUrl` hands off to on mobile), falling back to that
// app's own web view otherwise. Not custom URL schemes (`comgooglemaps:`/
// `waze:`) since those 404 outright when the app isn't installed instead
// of falling back to anything.
//
// Default to walking directions -- this is campus navigation between
// buildings, not a drive -- via each app's *directions* endpoint rather
// than just dropping a pin. Waze has no walking mode at all (it's a
// driving-only navigation app), so there's nothing to set there.
function externalMapsUrls(lat: number, lng: number, label: string) {
  const query = encodeURIComponent(label);
  return {
    google: `https://www.google.com/maps/dir/?api=1&destination=${lat},${lng}&travelmode=walking`,
    apple: `https://maps.apple.com/?daddr=${lat},${lng}&dirflg=w&q=${query}`,
    waze: `https://waze.com/ul?ll=${lat},${lng}&navigate=yes`,
  };
}

export function Map() {
  const params = useParams<{ roomCode?: string }>();
  // parse the first few letters e.g S200 -> S
  const roomType = createMemo(() => params.roomCode?.match(/^[A-Za-z]+/)?.[0]);

  const roomInfo = createMemo(() => (roomType() ? RoomCodes[roomType()!] : undefined));
  const center = createMemo(() => roomInfo()?.location);

  return (
    <>
      <Header />
      <div class="flex flex-col items-center gap-4 flex-1 min-h-0 w-full">
        <h2>Map</h2>
        {params.roomCode ? <p>{params.roomCode}</p> : null}
        <Show when={roomInfo()}>
          {roomInfo => <p>
            {roomInfo().name}
          </p>}
        </Show>
        <Show when={center()} fallback={<p>No location available</p>}>
          {center => <>
            <div class="w-full flex-1 min-h-0 p-4">
              {isMobile()
                ? <MapViewNative lat={center().lat} lng={center().lng} zoom={19.5} title={roomInfo()?.name} />
                : <GoogleMap lat={center().lat} lng={center().lng} zoom={19.5} />}
            </div>
            <Show when={isMobile()}>
              {(() => {
                const urls = () => externalMapsUrls(center().lat, center().lng, roomInfo()?.name ?? params.roomCode ?? "");
                return (
                  <div class="flex gap-2 flex-wrap justify-center pb-4">
                    <button class="btn btn-sm" onClick={() => openUrl(urls().google)}>
                      {t().openInMaps.google}
                    </button>
                    <button class="btn btn-sm" onClick={() => openUrl(urls().apple)}>
                      {t().openInMaps.apple}
                    </button>
                    <button class="btn btn-sm" onClick={() => openUrl(urls().waze)}>
                      {t().openInMaps.waze}
                    </button>
                  </div>
                );
              })()}
            </Show>
          </>
          }
        </Show>
      </div>
    </>
  );
}

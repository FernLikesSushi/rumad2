import { useNavigate, useParams } from "@solidjs/router";
import { Header } from "../components/Header";
import { createMemo, createSignal, For, Show } from "solid-js";
import { RoomCodes } from "../data/roomMap";
import { GoogleMap } from "../components/geo/GoogleMapWeb";
import { MapViewNative } from "../components/geo/MapNative";
import { isMobile } from "../data/platform";
import { openUrl } from "@tauri-apps/plugin-opener";
import { t } from "../i18n";
import { List, Search } from "lucide-solid";

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
  const navigate = useNavigate();
  // parse the first few letters e.g S200 -> S
  const roomType = createMemo(() => params.roomCode?.match(/^[A-Za-z]+/)?.[0]);

  const roomInfo = createMemo(() => (roomType() ? RoomCodes[roomType()!] : undefined));
  const center = createMemo(() => roomInfo()?.location);

  let roomCodeInput: HTMLInputElement | undefined;
  const [roomListOpen, setRoomListOpen] = createSignal(false);

  return (
    <>
      <Header />
      <div class="flex flex-col items-center gap-4 flex-1 min-h-0 w-full">
        <h2>Map</h2>
        {/* Room code input form */}
        <form
          class="flex gap-2"
          onSubmit={e => {
            e.preventDefault();
            const value = roomCodeInput!.value.trim();
            if (value) navigate(`/map/${value}`);
          }}
        >
          <input
            ref={roomCodeInput}
            type="text"
            class="input input-bordered input-sm"
            placeholder={t().roomCodePlaceholder}
            value={params.roomCode ?? ""}
          />
          <button
            type="submit"
            class="btn btn-sm btn-square tooltip"
            aria-label={t().roomCodePlaceholder}
            data-tip={t().roomCodePlaceholder}
          >
            <Search size={16} />
          </button>
          <button
            type="button"
            class="btn btn-sm btn-square tooltip"
            aria-label={t().roomCodeListButton}
            data-tip={t().roomCodeListButton}
            onClick={() => setRoomListOpen(true)}
          >
            <List size={16} />
          </button>
        </form>

        <RoomCodeTableModal
          open={roomListOpen()}
          currentCode={roomType()}
          onClose={() => setRoomListOpen(false)}
          onSelect={code => {
            setRoomListOpen(false);
            navigate(`/map/${code}`);
          }}
        />

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


// The full `RoomCodes` list as a modal table -- each row links to
// `/map/{code}`, an alternative to typing into the input above for
// browsing rather than searching by a known code.
function RoomCodeTableModal(props: {
  open: boolean;
  currentCode?: string;
  onClose: () => void;
  onSelect: (code: string) => void;
}) {
  return (
    <Show when={props.open}>
      <div class="modal modal-open" onClick={props.onClose}>
        <div
          class="modal-box max-w-full sm:max-w-2xl lg:max-w-4xl p-4 sm:p-8"
          role="dialog"
          aria-modal="true"
          onClick={e => e.stopPropagation()}
        >
          <h3 class="font-bold text-lg">{t().roomCodeListTitle}</h3>
          {/* `grid`, not CSS multi-column (`columns-*`) -- multi-column
              reflows overflow *sideways* once its height is capped (it
              adds more columns rather than scrolling), which fought the
              `max-h` below into scrolling horizontally instead of
              vertically. A plain grid just adds more rows downward as
              content overflows, so a normal vertical scrollbar is what
              you get. `max-h` caps it well short of the viewport on
              mobile (single column, so otherwise the full ~50-room list
              pushes the modal to nearly fullscreen height). */}
          <div class="overflow-y-auto max-h-[70vh] sm:max-h-96 mt-2 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-x-6">
            <For each={Object.entries(RoomCodes)}>
              {([code, room]) => (
                <a
                  href={`/map/${code}`}
                  class="flex gap-2 items-baseline rounded px-2 py-1 hover:bg-base-200"
                  classList={{
                    "bg-base-300": code === props.currentCode,
                    "text-primary": code === props.currentCode,
                  }}
                  onClick={e => {
                    e.preventDefault();
                    props.onSelect(code);
                  }}
                >
                  <span class="link link-hover font-mono font-semibold shrink-0">{code}</span>
                  <span class="text-sm opacity-70 truncate">{room.name}</span>
                </a>
              )}
            </For>
          </div>
          <div class="modal-action">
            <button class="btn" onClick={props.onClose}>
              {t().close}
            </button>
          </div>
        </div>
      </div>
    </Show>
  );
}
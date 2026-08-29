import { useParams } from "@solidjs/router";
import { Header } from "../components/Header";
import { createMemo, Show } from "solid-js";
import { RoomCodes } from "../data/roomMap";
import { GoogleMap } from "../components/geo/GoogleMapWeb";
import { GoogleMapNative } from "../components/geo/GoogleMapNative";
import { isAndroid } from "../data/platform";

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
          {center => <div class="w-full flex-1 min-h-0 p-4">
            {isAndroid()
              ? <GoogleMapNative lat={center().lat} lng={center().lng} zoom={19.5} title={roomInfo()?.name} />
              : <GoogleMap lat={center().lat} lng={center().lng} zoom={19.5} />}
          </div>
          }
        </Show>
      </div>
    </>
  );
}

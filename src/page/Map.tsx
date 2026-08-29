import { useParams } from "@solidjs/router";
import { Header } from "../components/Header";
import { createMemo, Show } from "solid-js";
import { RoomCodes } from "../data/roomMap";

export function Map() {
  const params = useParams<{ roomCode?: string }>();
  // parse the first few letters e.g S200 -> S
  const roomType = createMemo(() => params.roomCode?.match(/^[A-Za-z]+/)?.[0]);

  const roomInfo = createMemo(() => (roomType() ? RoomCodes[roomType()!] : undefined));

  return (
    <>
      <Header />
      <div class="flex flex-col items-center justify-center gap-4">
        <h2>Map</h2>
        {params.roomCode ? <p>{params.roomCode}</p> : null}
        <Show when={roomInfo()}>
          {roomInfo => <p>
            {roomInfo().name}
          </p>}
        </Show>
        <p>Map screen is under construction.</p>
      </div>
    </>
  );
}

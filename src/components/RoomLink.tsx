import { createMemo } from "solid-js";

export function RoomLink(props: { room: string; text?: string; class?: string }) {
    // TODO: Handle roomless courses (e.g. online courses)

    function sanitizeRoomCode(room: string) {
        // Remove any whitespace and convert to uppercase
        return room.replace(/\s+/g, "").toUpperCase();
    }

    const link = createMemo(() => `/map/${sanitizeRoomCode(props.room)}`);
    const classes = createMemo(() => props.class ?? "link link-secondary");

    return (
        <a href={link()} class={classes()}>
            {props.text ?? props.room}
        </a>
    );
}
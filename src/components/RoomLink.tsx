import { createMemo } from "solid-js";

export function RoomLink(props: { room: string; text?: string; class?: string }) {
    // TODO: Handle roomless courses (e.g. online courses)

    const link = createMemo(() => `/map/${props.room}`);
    const classes = createMemo(() => props.class ?? "link link-secondary");

    return (
        <a href={link()} class={classes()}>
            {props.text ?? props.room}
        </a>
    );
}
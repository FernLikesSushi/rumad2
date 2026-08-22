import { createSignal, onCleanup, onMount } from "solid-js";

const FADE_MS = 1000;
const DISPLAY_MS = 8000;

export function FadingText(props: { texts: string[]; class?: string }) {
  const [index, setIndex] = createSignal(0);
  const [visible, setVisible] = createSignal(true);

  onMount(() => {
    if (props.texts.length <= 1) return;

    let timeoutId: ReturnType<typeof setTimeout>;

    function scheduleFadeOut() {
      timeoutId = setTimeout(() => {
        setVisible(false);
        
        timeoutId = setTimeout(() => {
          setIndex((i) => (i + 1) % props.texts.length);
          setVisible(true);
          scheduleFadeOut();
        }, FADE_MS);
      }, DISPLAY_MS);
    }

    scheduleFadeOut();
    onCleanup(() => clearTimeout(timeoutId));
  });

  return (
    // Tailwind's `duration-*` utilities only work as literal class names --
    // it can't see a dynamically-built `duration-${FADE_MS}` string, so the
    // duration has to be set as an inline style instead to actually apply.
    <div
      class={`transition-opacity ease-in-out ${visible() ? "opacity-100" : "opacity-0"} ${props.class ?? ""}`}
      style={{ "transition-duration": `${FADE_MS}ms` }}
    >
      {props.texts[index()]}
    </div>
  );
}

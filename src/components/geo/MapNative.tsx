import { createEffect, onCleanup, onMount } from "solid-js";
import { invoke } from "@tauri-apps/api/core";

// Android and iOS, via the app-internal `tauri-plugin-native-map` crate
// (`src-tauri/plugins/tauri-plugin-native-map/`) -- a real native map
// view (Android's `com.google.android.gms.maps.MapView`, iOS's
// `MKMapView`) layered on top of the webview, not anything DOM-based.
// There's no way to render inside an iframe/DOM for this, so instead
// this renders an empty placeholder `<div>` and keeps a native view
// positioned over its `getBoundingClientRect()` -- see the plugin
// crate's module doc. `GoogleMapWeb`'s iframe embed is what desktop uses
// instead (see `Map.tsx`'s platform check) -- this component itself
// doesn't need to know or care which native platform it's actually
// talking to, since both sides of the plugin implement the exact same
// command names.
export function MapViewNative(props: { lat: number; lng: number; zoom?: number; title?: string }) {
  let placeholder: HTMLDivElement | undefined;
  let created = false;

  // The native view is a sibling Android/iOS view added on top of the
  // *whole* webview, not a DOM node -- no HTML z-index or DOM order can
  // put anything above it. Every modal in this codebase renders as a
  // `.modal.modal-open` element (`NoticeDialog`, `SettingsButton`,
  // `RoomCodeTableModal`, ...), so instead of threading "is a modal
  // open" through every caller, shrink the native view to nothing
  // whenever one exists anywhere in the document and restore it once
  // none remain -- that reads as the map going invisible under the
  // modal rather than drawing over it.
  function anyModalOpen() {
    return document.querySelector(".modal.modal-open") !== null;
  }

  function frame() {
    if (anyModalOpen()) return { x: 0, y: 0, width: 0, height: 0 };
    const rect = placeholder!.getBoundingClientRect();
    return { x: rect.x, y: rect.y, width: rect.width, height: rect.height };
  }

  async function updateFrame() {
    if (!created) return;
    await invoke("plugin:native-map|update_frame", { payload: frame() });
  }

  onMount(() => {
    invoke("plugin:native-map|create_map", {
      payload: { ...frame(), lat: props.lat, lng: props.lng, zoom: props.zoom ?? 15 },
    }).then(() => {
      created = true;
    });

    const resizeObserver = new ResizeObserver(() => void updateFrame());
    resizeObserver.observe(placeholder!);
    window.addEventListener("scroll", updateFrame, true);
    window.addEventListener("resize", updateFrame);

    const modalObserver = new MutationObserver(() => void updateFrame());
    modalObserver.observe(document.body, { childList: true, subtree: true, attributes: true, attributeFilter: ["class"] });

    onCleanup(() => {
      resizeObserver.disconnect();
      modalObserver.disconnect();
      window.removeEventListener("scroll", updateFrame, true);
      window.removeEventListener("resize", updateFrame);
      created = false;
      void invoke("plugin:native-map|dispose");
    });
  });

  // Skips the initial run -- `create_map` above already set the starting
  // camera/marker; this only reacts to later prop changes (e.g.
  // navigating from one room's `Map` route to another without unmounting).
  createEffect((isFirst: boolean = true) => {
    const lat = props.lat;
    const lng = props.lng;
    const zoom = props.zoom ?? 15;
    const title = props.title;
    if (!isFirst && created) {
      void invoke("plugin:native-map|set_camera", { payload: { lat, lng, zoom } });
      void invoke("plugin:native-map|set_marker", { payload: { lat, lng, title } });
    }
    return false;
  });

  return <div ref={placeholder} class="w-full h-full" />;
}

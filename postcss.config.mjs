import oklabFunction from "@csstools/postcss-oklab-function";

// daisyUI's theme colors are all authored as oklch()/oklab(). Android's
// system WebView doesn't support those below Chromium 111 (confirmed
// Chromium 109 on-emulator), so any un-downleveled color there is an
// invalid custom property and the whole UI falls back to browser
// defaults. This converts every oklch()/oklab() value -- including
// inside custom properties, which is what daisyUI's theme vars are --
// to legacy rgb() at build time instead of hand-maintaining hex
// approximations of daisyUI's palette.
export default {
  plugins: [oklabFunction({ preserve: false })],
};

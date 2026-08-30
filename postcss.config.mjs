import postcssPresetEnv from "postcss-preset-env";

// Tailwind/daisyUI's generated CSS uses modern color syntax throughout --
// not just oklch()/oklab() (daisyUI's theme vars) but also color-mix()
// (Tailwind's own opacity/mix utilities, daisyUI's depth-shadow effects).
// Both need Chromium 111+; this project's declared floor (package.json's
// `browserslist`) is Chromium 109 (confirmed on-emulator), so anything
// left un-downleveled is invalid CSS there and silently drops. Rather
// than hand-picking which color functions to convert, `postcss-preset-env`
// reads that same browserslist and polyfills whatever it says needs it --
// stays correct if the target list or Tailwind/daisyUI's output changes.
export default {
  plugins: [postcssPresetEnv({
    stage: 1,
    features: {
      'nesting-rules': true,
      'custom-properties': true,
      'custom-media-queries': true,
      'color-mix': true
    }
  })],
};

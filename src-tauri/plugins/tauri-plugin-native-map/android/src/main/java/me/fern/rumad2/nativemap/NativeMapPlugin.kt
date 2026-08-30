package me.fern.rumad2.nativemap

import android.app.Activity
import android.view.ViewGroup
import android.widget.FrameLayout
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.Plugin
import com.google.android.gms.maps.CameraUpdateFactory
import com.google.android.gms.maps.GoogleMap
import com.google.android.gms.maps.MapView
import com.google.android.gms.maps.OnMapReadyCallback
import com.google.android.gms.maps.model.LatLng
import com.google.android.gms.maps.model.Marker
import com.google.android.gms.maps.model.MarkerOptions

// This is what `PluginHandle::run_mobile_plugin("createMap", payload)` on
// the Rust side (this plugin crate's `src/mobile.rs`) is actually calling
// *into*: Tauri's Android runtime finds the plugin class named in
// `register_android_plugin`
// (`NativeMapPlugin`, the class below), looks for a method on it
// annotated `@Command` whose *name* matches the string passed to
// `run_mobile_plugin` ("createMap" -> `fun createMap`), and calls it with
// the deserialized JSON payload wrapped in an `Invoke` object. Nothing
// here is triggered by anything Android itself calls automatically
// (aside from the lifecycle overrides near the bottom) -- it's purely a
// response to a Rust-side call.

// `@InvokeArg` classes are how Tauri deserializes a command's JSON
// payload into a real Kotlin object -- roughly like `serde::Deserialize`
// on the Rust side, but reflection-based: it matches JSON keys to `var`
// property names, so these have to be plain mutable fields with default
// values (not a Kotlin data class' constructor params) for the parser to
// populate them after construction. Field names/casing match the
// `#[serde(rename_all = "camelCase")]` structs in the Rust plugin's
// `models.rs` one-for-one; there's no compiler check tying the two
// together, only convention.

@InvokeArg
class FrameArgs {
    var x: Double = 0.0
    var y: Double = 0.0
    var width: Double = 0.0
    var height: Double = 0.0
}

// Not `FrameArgs` plus extra fields via inheritance -- kept as a flat,
// separate class instead, since it's not certain the `@InvokeArg`
// reflection-based parser would pick up inherited properties correctly,
// and the duplication (x/y/width/height repeated) is cheap compared to
// debugging that if it silently didn't.
@InvokeArg
class CreateMapArgs {
    var x: Double = 0.0
    var y: Double = 0.0
    var width: Double = 0.0
    var height: Double = 0.0
    var lat: Double = 0.0
    var lng: Double = 0.0
    var zoom: Float = 15f
}

@InvokeArg
class SetCameraArgs {
    var lat: Double = 0.0
    var lng: Double = 0.0
    var zoom: Float = 15f
}

@InvokeArg
class SetMarkerArgs {
    var lat: Double = 0.0
    var lng: Double = 0.0
    var title: String? = null
}

/**
 * Embeds a native `MapView` as a sibling of the webview's own view,
 * positioned to match a placeholder element the frontend measures via
 * `getBoundingClientRect()` -- there's no way to render a real Google
 * Maps SDK surface *inside* DOM/webview content, so this layers a
 * separate Android `View` on top instead and keeps it lined up with the
 * placeholder from the JS side (`update_frame`, called on resize/scroll).
 *
 * `createMap`/`updateFrame`/`setCamera`/`setMarker`/`dispose` mirror the
 * Rust `NativeMap<R>` handle's methods (`../../src/mobile.rs`) one-for-
 * one -- each just runs the equivalent `GoogleMap`/`MapView` call on the
 * UI thread, since none of the Maps SDK APIs are safe to touch off it.
 *
 * `@TauriPlugin` + extending `Plugin(activity)` is what makes this a
 * plugin Tauri's Android runtime actually knows about, rather than just
 * an ordinary Kotlin class -- `Plugin` is the base class that wires up
 * command dispatch (see `@Command` below) and forwards Activity
 * lifecycle events (`onResume`/`onPause`/`onDestroy`, also below) to
 * whatever this subclass overrides.
 */
@TauriPlugin
class NativeMapPlugin(private val activity: Activity) : Plugin(activity) {
    // The plugin class is created once and lives for the app's lifetime
    // (Tauri owns exactly one instance, same as `mobile.rs`'s
    // `PluginHandle` on the Rust side only ever pointing at this one
    // instance) -- so these fields are where "is there currently a map,
    // and which one" actually lives, not local state some command
    // handler owns temporarily.
    private var mapView: MapView? = null
    private var googleMap: GoogleMap? = null
    private var marker: Marker? = null

    // A `setCamera`/`setMarker` call that raced ahead of `getMapAsync`
    // resolving -- applied once the map actually becomes ready instead of
    // being silently dropped.
    private var pendingCamera: SetCameraArgs? = null
    private var pendingMarker: SetMarkerArgs? = null

    private val density get() = activity.resources.displayMetrics.density

    // CSS px (from the frontend's `getBoundingClientRect()`) -> real
    // device px -- `View.layout`/`LayoutParams` need actual pixels, not
    // the density-independent units CSS reports.
    private fun frameToLayoutParams(x: Double, y: Double, width: Double, height: Double): FrameLayout.LayoutParams {
        val params = FrameLayout.LayoutParams((width * density).toInt(), (height * density).toInt())
        params.leftMargin = (x * density).toInt()
        params.topMargin = (y * density).toInt()
        return params
    }

    // `@Command` is what makes a method callable from the Rust side at
    // all -- Tauri's plugin runtime scans this class for methods with
    // this annotation and dispatches to whichever one's name matches the
    // string `run_mobile_plugin` was called with (see the big comment at
    // the top of this file). The method name itself ("createMap") is
    // that link -- nothing else associates this function with the Rust
    // `create_map` command.
    @Command
    fun createMap(invoke: Invoke) {
        // `Invoke` wraps the raw call: `parseArgs` deserializes its JSON
        // payload into the `@InvokeArg` class given, and later on this
        // same `invoke` is how the result (or an error, via
        // `invoke.reject(...)`, not used here) gets sent back to
        // whichever Rust `run_mobile_plugin` call is waiting on it.
        val args = invoke.parseArgs(CreateMapArgs::class.java)

        // Every Maps SDK / View call in this file happens inside
        // `runOnUiThread` -- Tauri plugin commands can run on a
        // background thread, but Android Views (and the Maps SDK
        // specifically) are only safe to touch from the main/UI thread.
        activity.runOnUiThread {
            disposeInternal()

            val view = MapView(activity)
            mapView = view
            view.onCreate(null)
            view.onResume()

            val root = activity.window.decorView.findViewById<ViewGroup>(android.R.id.content)
            root.addView(view, frameToLayoutParams(args.x, args.y, args.width, args.height))

            // `getMapAsync` -- the `MapView` itself isn't a usable
            // `GoogleMap` yet the instant it's constructed; this callback
            // fires once the SDK's actually ready, which is also the
            // earliest point camera/marker calls are safe to make (see
            // `moveCamera`/`placeMarker`'s own null-check-and-queue logic
            // below for what happens if one arrives before this fires).
            view.getMapAsync(OnMapReadyCallback { map ->
                googleMap = map
                map.uiSettings.isZoomControlsEnabled = true

                val camera = pendingCamera
                val markerArgs = pendingMarker
                pendingCamera = null
                pendingMarker = null

                moveCamera(
                    LatLng(camera?.lat ?: args.lat, camera?.lng ?: args.lng),
                    camera?.zoom ?: args.zoom,
                )
                placeMarker(
                    LatLng(markerArgs?.lat ?: args.lat, markerArgs?.lng ?: args.lng),
                    markerArgs?.title,
                )
            })
        }

        // `invoke.resolve()` is what makes the Rust-side
        // `run_mobile_plugin(...)` call actually return `Ok(())` -- until
        // this is called, that Rust call just blocks waiting. Called
        // here immediately, not inside the `runOnUiThread` block above,
        // so this is fire-and-forget from the JS side's perspective: it
        // resolves once the work is *posted* to the UI thread, not once
        // the map has actually finished (re)rendering -- matches
        // `updateFrame` being driven by best-effort resize/scroll events
        // anyway.
        invoke.resolve()
    }

    @Command
    fun updateFrame(invoke: Invoke) {
        val args = invoke.parseArgs(FrameArgs::class.java)
        activity.runOnUiThread {
            mapView?.layoutParams = frameToLayoutParams(args.x, args.y, args.width, args.height)
        }
        invoke.resolve()
    }

    @Command
    fun setCamera(invoke: Invoke) {
        val args = invoke.parseArgs(SetCameraArgs::class.java)
        activity.runOnUiThread {
            moveCamera(LatLng(args.lat, args.lng), args.zoom)
        }
        invoke.resolve()
    }

    @Command
    fun setMarker(invoke: Invoke) {
        val args = invoke.parseArgs(SetMarkerArgs::class.java)
        activity.runOnUiThread {
            placeMarker(LatLng(args.lat, args.lng), args.title)
        }
        invoke.resolve()
    }

    @Command
    fun dispose(invoke: Invoke) {
        activity.runOnUiThread { disposeInternal() }
        invoke.resolve()
    }

    // Plain private helpers below -- not `@Command`-annotated, so Tauri
    // has no idea these exist; they're only reachable from the methods
    // above (and each other).

    private fun moveCamera(position: LatLng, zoom: Float) {
        val map = googleMap
        if (map == null) {
            pendingCamera = SetCameraArgs().apply {
                lat = position.latitude
                lng = position.longitude
                this.zoom = zoom
            }
            return
        }
        map.moveCamera(CameraUpdateFactory.newLatLngZoom(position, zoom))
    }

    private fun placeMarker(position: LatLng, title: String?) {
        val map = googleMap
        if (map == null) {
            pendingMarker = SetMarkerArgs().apply {
                lat = position.latitude
                lng = position.longitude
                this.title = title
            }
            return
        }
        marker?.remove()
        marker = map.addMarker(MarkerOptions().position(position).title(title))
    }

    private fun disposeInternal() {
        marker?.remove()
        marker = null
        googleMap = null
        pendingCamera = null
        pendingMarker = null
        mapView?.let { view ->
            (view.parent as? ViewGroup)?.removeView(view)
            view.onPause()
            view.onDestroy()
        }
        mapView = null
    }

    // Forwarded from the Activity's own lifecycle -- these three
    // `override fun`s aren't called by anything in this file; Tauri's
    // `Plugin` base class hooks into the Activity's real lifecycle
    // callbacks and calls these automatically (e.g. when the user
    // backgrounds the app). `MapView` needs `onResume`/`onPause`/
    // `onDestroy` forwarded to it to pause/resume its internal
    // `GLSurfaceView` rendering along with the rest of the app, instead
    // of leaking a live GL context or rendering while backgrounded.
    override fun onResume() {
        super.onResume()
        mapView?.onResume()
    }

    override fun onPause() {
        mapView?.onPause()
        super.onPause()
    }

    override fun onDestroy() {
        disposeInternal()
        super.onDestroy()
    }
}

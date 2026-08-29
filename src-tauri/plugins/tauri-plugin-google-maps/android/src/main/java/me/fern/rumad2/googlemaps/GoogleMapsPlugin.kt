package me.fern.rumad2.googlemaps

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

// Field names/casing match the `#[serde(rename_all = "camelCase")]`
// structs in the Rust plugin's `models.rs` one-for-one -- Tauri's Android
// arg parser maps JSON keys onto these by name.

@InvokeArg
class FrameArgs {
    var x: Double = 0.0
    var y: Double = 0.0
    var width: Double = 0.0
    var height: Double = 0.0
}

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
 * Rust `GoogleMaps<R>` handle's methods (`../../src/mobile.rs`) one-for-
 * one -- each just runs the equivalent `GoogleMap`/`MapView` call on the
 * UI thread, since none of the Maps SDK APIs are safe to touch off it.
 */
@TauriPlugin
class GoogleMapsPlugin(private val activity: Activity) : Plugin(activity) {
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

    @Command
    fun createMap(invoke: Invoke) {
        val args = invoke.parseArgs(CreateMapArgs::class.java)

        activity.runOnUiThread {
            disposeInternal()

            val view = MapView(activity)
            mapView = view
            view.onCreate(null)
            view.onResume()

            val root = activity.window.decorView.findViewById<ViewGroup>(android.R.id.content)
            root.addView(view, frameToLayoutParams(args.x, args.y, args.width, args.height))

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

        // Fire-and-forget from the JS side's perspective: this resolves
        // once the work is *posted* to the UI thread, not once the map
        // has actually finished (re)rendering -- matches `updateFrame`
        // being driven by best-effort resize/scroll events anyway.
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

    // Forwarded from the Activity's own lifecycle -- `MapView` needs
    // these to pause/resume its internal `GLSurfaceView` rendering along
    // with the rest of the app instead of leaking a live GL context or
    // rendering while backgrounded.
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

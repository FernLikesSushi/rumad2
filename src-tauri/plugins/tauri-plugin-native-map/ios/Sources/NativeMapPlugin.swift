import SwiftRs
import Tauri
import UIKit
import WebKit
import MapKit

// This is what `PluginHandle::run_mobile_plugin("createMap", payload)`
// on the Rust side (this plugin crate's `src/mobile.rs`) is actually
// calling *into* on iOS -- mirrors `NativeMapPlugin.kt`'s role on
// Android exactly (same command names, same argument shapes), just
// against Apple's MapKit (`MKMapView`) instead of the Google Maps SDK,
// since MapKit is what's actually available on iOS without pulling in
// an extra SDK/API key.

// `Decodable` classes with `var` properties + default values, not
// structs or `let` -- matches the shape Tauri's iOS runtime expects for
// `invoke.parseArgs(_:)` (its JSON-argument parser), analogous to
// `@InvokeArg`-annotated classes in the Kotlin plugin. `var` + defaults
// (rather than `let` with no defaults) is also what gives each class a
// free no-argument `init()` -- needed below in the `.from(...)` helpers,
// since Swift only synthesizes that when every stored property already
// has a default. Field names/casing match the `#[serde(rename_all =
// "camelCase")]` structs in the Rust plugin's `models.rs` one-for-one --
// there's no compiler check tying the two together, only convention
// (same caveat as the Kotlin side).

class FrameArgs: Decodable {
    var x: Double = 0
    var y: Double = 0
    var width: Double = 0
    var height: Double = 0
}

// Not `FrameArgs` plus extra fields via inheritance -- kept as a flat,
// separate class, same reasoning as the Kotlin plugin's `CreateMapArgs`.
class CreateMapArgs: Decodable {
    var x: Double = 0
    var y: Double = 0
    var width: Double = 0
    var height: Double = 0
    var lat: Double = 0
    var lng: Double = 0
    var zoom: Float = 15
}

class SetCameraArgs: Decodable {
    var lat: Double = 0
    var lng: Double = 0
    var zoom: Float = 15
}

class SetMarkerArgs: Decodable {
    var lat: Double = 0
    var lng: Double = 0
    var title: String? = nil
}

/// Embeds a native `MKMapView` as a sibling of the webview's own view,
/// positioned to match a placeholder element the frontend measures via
/// `getBoundingClientRect()` -- same approach as the Android plugin (see
/// `NativeMapPlugin.kt`'s doc comment for why a real native view is
/// layered on top instead of anything DOM-based).
///
/// `createMap`/`updateFrame`/`setCamera`/`setMarker`/`dispose` mirror the
/// Rust `NativeMap<R>` handle's methods (`../../src/mobile.rs`) one-for-
/// one -- each just runs the equivalent `MKMapView` call on the main
/// thread, since UIKit views (like Android's Maps SDK) are only safe to
/// touch there.
class NativeMapPlugin: Plugin {
    private var mapView: MKMapView?
    private var marker: MKPointAnnotation?

    // A `setCamera`/`setMarker` call that raced ahead of the map view
    // actually existing -- there's no async "map ready" callback on
    // MapKit the way `MKMapView.getMapAsync` exists on Android (the view
    // is usable the instant it's constructed), so this only matters for
    // the narrow window between `createMap` posting to the main thread
    // and that block actually running; kept for the same defensive
    // reason the Kotlin side queues, not because it's been hit live.
    private var pendingCamera: SetCameraArgs?
    private var pendingMarker: SetMarkerArgs?

    // `@objc` is what makes a method callable from the Rust side at all
    // -- Tauri's plugin runtime looks up methods by this Objective-C
    // selector name, which is why it has to be `@objc` (Swift-only
    // methods aren't visible to that lookup). The method name itself
    // ("createMap") is the link to the Rust `create_map` command --
    // nothing else associates the two.
    @objc public func createMap(_ invoke: Invoke) throws {
        let args = try invoke.parseArgs(CreateMapArgs.self)

        // Every MapKit/UIKit call in this file happens inside
        // `DispatchQueue.main.async` -- Tauri plugin commands can run on
        // a background thread, but UIKit views (including MKMapView)
        // are only safe to touch from the main thread, same rule as
        // Android's `runOnUiThread` in the Kotlin plugin.
        DispatchQueue.main.async {
            self.disposeInternal()

            // `self.manager.viewController` is Tauri's handle to the
            // app's root view controller -- its `.view` is what the
            // WKWebView itself is layered inside, so adding this map as
            // a sibling subview there puts it in the same view hierarchy
            // as the webview rather than, say, a detached window.
            guard let root = self.manager.viewController?.view else { return }

            let frame = CGRect(x: args.x, y: args.y, width: args.width, height: args.height)
            let view = MKMapView(frame: frame)
            self.mapView = view
            root.addSubview(view)

            let camera = self.pendingCamera
            let markerArgs = self.pendingMarker
            self.pendingCamera = nil
            self.pendingMarker = nil

            self.moveCamera(
                coordinate: CLLocationCoordinate2D(
                    latitude: camera?.lat ?? args.lat,
                    longitude: camera?.lng ?? args.lng
                ),
                zoom: camera?.zoom ?? args.zoom
            )
            self.placeMarker(
                coordinate: CLLocationCoordinate2D(
                    latitude: markerArgs?.lat ?? args.lat,
                    longitude: markerArgs?.lng ?? args.lng
                ),
                title: markerArgs?.title
            )
        }

        // `invoke.resolve()` is what makes the Rust-side
        // `run_mobile_plugin(...)` call actually return `Ok(())` --
        // called here immediately, not inside the `DispatchQueue.main.async`
        // block above, so this is fire-and-forget from the JS side's
        // perspective, same as the Kotlin plugin's `createMap`.
        invoke.resolve()
    }

    @objc public func updateFrame(_ invoke: Invoke) throws {
        let args = try invoke.parseArgs(FrameArgs.self)
        DispatchQueue.main.async {
            self.mapView?.frame = CGRect(x: args.x, y: args.y, width: args.width, height: args.height)
        }
        invoke.resolve()
    }

    @objc public func setCamera(_ invoke: Invoke) throws {
        let args = try invoke.parseArgs(SetCameraArgs.self)
        DispatchQueue.main.async {
            self.moveCamera(coordinate: CLLocationCoordinate2D(latitude: args.lat, longitude: args.lng), zoom: args.zoom)
        }
        invoke.resolve()
    }

    @objc public func setMarker(_ invoke: Invoke) throws {
        let args = try invoke.parseArgs(SetMarkerArgs.self)
        DispatchQueue.main.async {
            self.placeMarker(coordinate: CLLocationCoordinate2D(latitude: args.lat, longitude: args.lng), title: args.title)
        }
        invoke.resolve()
    }

    @objc public func dispose(_ invoke: Invoke) throws {
        DispatchQueue.main.async {
            self.disposeInternal()
        }
        invoke.resolve()
    }

    // Plain private helpers below -- not `@objc`, so Tauri has no idea
    // these exist; only reachable from the methods above.

    private func moveCamera(coordinate: CLLocationCoordinate2D, zoom: Float) {
        guard let map = mapView else {
            pendingCamera = SetCameraArgs.from(coordinate: coordinate, zoom: zoom)
            return
        }
        // MapKit has no "zoom level" integer the way the Google Maps SDK
        // does -- it works in a lat/lng span (how much of the globe is
        // visible) instead. `360 / 2^zoom` is the standard degrees-per-
        // tile approximation for a given web-map zoom level (same
        // formula the Embed API/most map libraries use internally), so
        // this treats the incoming `zoom` (already used as a Google-
        // style zoom level for `GoogleMapWeb`/the Android plugin)
        // as if it meant the same thing here -- close enough for
        // "center on this building," not pixel-exact versus Android.
        let span = 360 / pow(2, Double(zoom))
        let region = MKCoordinateRegion(
            center: coordinate,
            span: MKCoordinateSpan(latitudeDelta: span, longitudeDelta: span)
        )
        map.setRegion(region, animated: false)
    }

    private func placeMarker(coordinate: CLLocationCoordinate2D, title: String?) {
        guard let map = mapView else {
            pendingMarker = SetMarkerArgs.from(coordinate: coordinate, title: title)
            return
        }
        if let existing = marker {
            map.removeAnnotation(existing)
        }
        let point = MKPointAnnotation()
        point.coordinate = coordinate
        point.title = title
        map.addAnnotation(point)
        marker = point
    }

    private func disposeInternal() {
        mapView?.removeFromSuperview()
        mapView = nil
        marker = nil
        pendingCamera = nil
        pendingMarker = nil
    }
}

// Small conveniences so `moveCamera`/`placeMarker` can build a "pending"
// value straight from a `CLLocationCoordinate2D` instead of unpacking
// `.latitude`/`.longitude` by hand at each call site -- `Decodable`
// classes don't get a memberwise initializer for free the way a Swift
// `struct` would, hence these instead of `SetCameraArgs(lat:lng:zoom:)`.
private extension SetCameraArgs {
    static func from(coordinate: CLLocationCoordinate2D, zoom: Float) -> SetCameraArgs {
        let args = SetCameraArgs()
        args.lat = coordinate.latitude
        args.lng = coordinate.longitude
        args.zoom = zoom
        return args
    }
}

private extension SetMarkerArgs {
    static func from(coordinate: CLLocationCoordinate2D, title: String?) -> SetMarkerArgs {
        let args = SetMarkerArgs()
        args.lat = coordinate.latitude
        args.lng = coordinate.longitude
        args.title = title
        return args
    }
}

// The C-exported entry point `mobile.rs`'s `tauri::ios_plugin_binding!
// (init_plugin_native_map)` links against -- the string passed to that
// Rust macro has to match this symbol name exactly (`init_plugin_native_map`
// in both places), that's the entire connection between the two. This is
// the only place this plugin class actually gets constructed.
@_cdecl("init_plugin_native_map")
func initPlugin() -> Plugin {
    return NativeMapPlugin()
}

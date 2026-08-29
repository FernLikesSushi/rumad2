export function GoogleMap(props: { lat: number, lng: number, zoom?: number }) {
    const zoom = props.zoom ?? 15;
    return (
        <iframe
            width="100%"
            height="100%"
            loading="lazy"

            referrerPolicy="no-referrer-when-downgrade"
            src={`https://www.google.com/maps/embed/v1/place?key=${import.meta.env.VITE_GOOGLE_MAPS_API_KEY}&q=${props.lat},${props.lng}&zoom=${zoom}`}
        />
    );
}
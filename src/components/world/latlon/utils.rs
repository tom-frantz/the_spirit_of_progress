pub fn wrap_latitude(latitude: f32) -> f32 {
    let mut lat = latitude;
    while lat <= -180.0 {
        lat += 360.0;
    }
    lat
}
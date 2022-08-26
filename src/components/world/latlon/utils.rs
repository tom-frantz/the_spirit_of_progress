use crate::components::world::latlon::{LatLonPoint, WorldPoint};

pub fn wrap_lon(mut lon: f32) -> f32 {
    loop {
        if lon > 180. {
            lon -= 360.
        } else if lon <= -180. {
            lon += 360.
        } else {
            return lon;
        }
    }
}

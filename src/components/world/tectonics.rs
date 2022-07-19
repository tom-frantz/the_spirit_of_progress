use bevy::prelude::*;

pub trait WorldPoint {
    // Vertical (-180, 180]
    fn latitude(&self) -> f32;
    // Horizontal [-90, 90]
    fn longitude(&self) -> f32;

    fn lat(&self) -> f32 {
        self.latitude()
    }

    fn lon(&self) -> f32 {
        self.longitude()
    }
}

impl WorldPoint for Vec2 {
    fn latitude(&self) -> f32 {
        self.x
    }

    fn longitude(&self) -> f32 {
        self.y
    }
}

pub struct WorldTectonics<T>
where
    T: WorldPoint,
{
    points: Vec<T>,
}

impl<T> WorldTectonics<T>
where
    T: WorldPoint,
{
    fn point_index(lat: f32, lon: f32) -> usize {
        let adjusted_lat = lat + 179;
        let adjusted_lon = (lon + 90) * 360;
        return adjusted_lon + adjusted_lat;
    }

    pub fn point(&self, lat: f32, lon: f32) -> &T {
        return &self.points[Self::point_index(lat, lon)];
    }
}

pub enum ZIndex {
    Map,
    City,
    Connection,
}

impl ZIndex {
    pub fn z_index(&self) -> f32 {
        match self {
            ZIndex::Map => 0.0,
            ZIndex::City => 10.0,
            ZIndex::Connection => 20.0,
        }
    }
}

impl Into<f32> for ZIndex {
    fn into(self) -> f32 {
        self.z_index()
    }
}

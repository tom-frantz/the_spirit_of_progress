use crate::Color;

pub const LIGHT_GREEN: &str = "B6B388";
pub const DARK_GREEN: &str = "7E9579";

pub const LIGHT_ORANGE: &str = "D6A771";
pub const DARK_ORANGE: &str = "CD834E";

pub const CREAM: &str = "F4E8CE";
pub const DARK: &str = "30261C";

pub enum MapColour {
    LightGreen,
    DarkGreen,
    LightOrange,
    DarkOrange,
    Cream,
    Dark,
}

impl MapColour {
    fn hex(&self) -> &'static str {
        match self {
            MapColour::LightGreen => "B6B388",
            MapColour::DarkGreen => "7E9579",
            MapColour::LightOrange => "D6A771",
            MapColour::DarkOrange => "CD834E",
            MapColour::Cream => "F4E8CE",
            MapColour::Dark => "30261C",
        }
    }
}

impl Into<Color> for MapColour {
    fn into(self) -> Color {
        let hex = self.hex();
        Color::hex(hex).unwrap()
    }
}

use crate::{Color, UiColor};

pub enum TerrainColour {
    Step1,
    Step2,
    Step3,
    Step4,
    Step5,
    Step6,
    Step7,
    Step8,
}

impl TerrainColour {
    fn hex(&self) -> &'static str {
        match self {
            TerrainColour::Step1 => "c8d0ab",
            TerrainColour::Step2 => "e5d6b5",
            TerrainColour::Step3 => "d6bba6",
            TerrainColour::Step4 => "cf968f",
            TerrainColour::Step5 => "c27474",
            TerrainColour::Step6 => "ba525b",
            TerrainColour::Step7 => "b94a51",
            TerrainColour::Step8 => "973c47",
        }
    }
}

impl Into<Color> for TerrainColour {
    fn into(self) -> Color {
        let hex = self.hex();
        Color::hex(hex).unwrap()
    }
}

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

pub enum TypographyColour {
    BackgroundBorder,
    Background,
    Blue,
    Black,
    Red,
}

impl TypographyColour {
    fn hex(&self) -> &'static str {
        match self {
            TypographyColour::BackgroundBorder => "dad4c3",
            TypographyColour::Background => "f0e9d7",

            TypographyColour::Blue => "234560",
            TypographyColour::Black => "3b3b33",
            // TODO change red to match book.
            TypographyColour::Red => "612923",
        }
    }
}

impl Into<Color> for TypographyColour {
    fn into(self) -> Color {
        let hex = self.hex();
        Color::hex(hex).unwrap()
    }
}

impl Into<UiColor> for TypographyColour {
    fn into(self) -> UiColor {
        UiColor(self.into())
    }
}

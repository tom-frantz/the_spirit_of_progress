use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::TileColor;
use rand::distributions::Standard;
use rand::prelude::*;

pub const SPACING: f32 = 8.0;

pub trait Colour {
    fn hex(&self) -> String;

    fn tile_color(&self) -> TileColor {
        TileColor(self.color())
    }

    fn color(&self) -> Color {
        Color::hex(self.hex()).unwrap()
    }

    fn ui_color(&self) -> UiColor {
        UiColor(self.color())
    }
}

pub enum MenuColour {
    Background,
    Border,
    BorderBackground, // Also 'neutral' on a map (ie uncoloured parts)
    BackgroundSecondary,

    BlackPen,
    RedPen,
    BluePen,

    Chart,
    BrownPen,
    GreenPen,
}

impl Colour for MenuColour {
    fn hex(&self) -> String {
        String::from(match &self {
            MenuColour::Background => "F4E8D2",
            MenuColour::Border => "C19686",
            MenuColour::BorderBackground => "DBC1A6",
            MenuColour::BackgroundSecondary => "EADBBD",
            MenuColour::BlackPen => "483D3C",
            MenuColour::RedPen => "D96A50",
            MenuColour::BluePen => "6E8283",
            MenuColour::Chart => "B3A990",
            MenuColour::BrownPen => "936E3E",
            MenuColour::GreenPen => "8B9880",
        })
    }
}

pub enum IndustryColour {
    LightPurple,
    Purple,

    LightRed,
    Red,

    LightBlue,
    Blue,

    Green,
    BlueGreen,

    PaleYellow,
    Brown,
}

impl IndustryColour {
    pub fn vec() -> Vec<IndustryColour> {
        vec![
            IndustryColour::LightPurple,
            IndustryColour::Purple,
            IndustryColour::LightRed,
            IndustryColour::Red,
            IndustryColour::LightBlue,
            IndustryColour::Blue,
            IndustryColour::Green,
            IndustryColour::BlueGreen,
            IndustryColour::PaleYellow,
            IndustryColour::Brown,
        ]
    }
}

impl Colour for IndustryColour {
    fn hex(&self) -> String {
        String::from(match self {
            IndustryColour::LightPurple => "D7ABB3",
            IndustryColour::Purple => "8C435A",
            IndustryColour::LightRed => "E0966E",
            IndustryColour::Red => "DA594A",
            IndustryColour::LightBlue => "A5B1AD",
            IndustryColour::Blue => "7C97A1",
            IndustryColour::Green => "7A9577",
            IndustryColour::BlueGreen => "6E8A82",
            IndustryColour::PaleYellow => "E3D89F",
            IndustryColour::Brown => "C2A37D",
        })
    }
}

impl Distribution<IndustryColour> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> IndustryColour {
        match rng.gen_range(0..10) {
            0 => IndustryColour::LightPurple,
            1 => IndustryColour::Purple,
            2 => IndustryColour::LightRed,
            3 => IndustryColour::Red,
            4 => IndustryColour::LightBlue,
            5 => IndustryColour::Blue,
            6 => IndustryColour::Green,
            7 => IndustryColour::BlueGreen,
            8 => IndustryColour::PaleYellow,
            9 => IndustryColour::Brown,
            _ => panic!("Aaaaaa random failed?"),
        }
    }
}

pub enum IndustryColour2 {
    LightRed,
    Red,

    LightBlue,
    Blue,
    DarkBlue,

    Sepia,

    LightPurple,
    Purple,

    LightYellow,
    Yellow,

    Green,
}

impl IndustryColour2 {
    pub fn vec() -> Vec<IndustryColour2> {
        vec![
            IndustryColour2::LightRed,
            IndustryColour2::Red,
            IndustryColour2::LightBlue,
            IndustryColour2::Blue,
            IndustryColour2::DarkBlue,
            IndustryColour2::Sepia,
            IndustryColour2::LightPurple,
            IndustryColour2::Purple,
            IndustryColour2::LightYellow,
            IndustryColour2::Yellow,
            IndustryColour2::Green,
        ]
    }
}

impl Colour for IndustryColour2 {
    fn hex(&self) -> String {
        String::from(match self {
            IndustryColour2::LightRed => "DF8A60",
            IndustryColour2::Red => "B9351E",
            IndustryColour2::LightBlue => "708B94",
            IndustryColour2::Blue => "1E517B",
            IndustryColour2::DarkBlue => "203950",
            IndustryColour2::Sepia => "AD8876",
            IndustryColour2::LightPurple => "EAB2A2",
            IndustryColour2::Purple => "8C5D63",
            IndustryColour2::LightYellow => "F4E49B",
            IndustryColour2::Yellow => "EEC875",
            IndustryColour2::Green => "B0BD75",
        })
    }
}

#[derive(Debug, Clone)]
pub enum Agriculture {
    Pink,
    LightPurple,
    Purple,
    DarkPurple,

    Green1,
    Green2,
    Green3,
    Green4,
    Green5,

    Grey,
    GreenYellow,
    Yellow,
    DarkGreenYellow,
    Brown,

    PaleGreen,
    LightGreen,
}

impl Distribution<Agriculture> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Agriculture {
        match rng.gen_range(0..16) {
            0 => Agriculture::Pink,
            1 => Agriculture::LightPurple,
            2 => Agriculture::Purple,
            3 => Agriculture::DarkPurple,
            4 => Agriculture::Green1,
            5 => Agriculture::Green2,
            6 => Agriculture::Green3,
            7 => Agriculture::Green4,
            8 => Agriculture::Green5,
            9 => Agriculture::Grey,
            10 => Agriculture::GreenYellow,
            11 => Agriculture::Yellow,
            12 => Agriculture::DarkGreenYellow,
            13 => Agriculture::Brown,
            14 => Agriculture::PaleGreen,
            15 => Agriculture::LightGreen,
            _ => panic!("Aaaaaa random failed?"),
        }
    }
}

impl Colour for Agriculture {
    fn hex(&self) -> String {
        String::from(match self {
            Agriculture::Pink => "D3A1A5",
            Agriculture::LightPurple => "B24F75",
            Agriculture::Purple => "833B58",
            Agriculture::DarkPurple => "8D6376",
            Agriculture::Green1 => "96A68C",
            Agriculture::Green2 => "8CA286",
            Agriculture::Green3 => "90A471",
            Agriculture::Green4 => "7B8D75",
            Agriculture::Green5 => "6D9073",
            Agriculture::Grey => "D7BCA6",
            Agriculture::GreenYellow => "D6BD98",
            Agriculture::Yellow => "E5B285",
            Agriculture::DarkGreenYellow => "C4A473",
            Agriculture::Brown => "BE8464",
            Agriculture::PaleGreen => "B3AF85",
            Agriculture::LightGreen => "C8C88F",
        })
    }
}

pub enum Agriculture2 {
    LightYellow,
    Yellow,
    Orange,
    Brown,
    LightGreen,
    BlueGreen,
}

impl Colour for Agriculture2 {
    fn hex(&self) -> String {
        String::from(match self {
            Agriculture2::LightYellow => "F3D488",
            Agriculture2::Yellow => "EBB35E",
            Agriculture2::Orange => "C8733C",
            Agriculture2::Brown => "B79B5F",
            Agriculture2::LightGreen => "B4B264",
            Agriculture2::BlueGreen => "79A282",
        })
    }
}

pub enum Livestock {
    Sparse,
    Step1,
    Step2,
    Step3,
    Step4,
    Dense,
}

impl Colour for Livestock {
    fn hex(&self) -> String {
        String::from(match self {
            Livestock::Sparse => "E0D79A",
            Livestock::Step1 => "A9AF8A",
            Livestock::Step2 => "90A17D",
            Livestock::Step3 => "879E81",
            Livestock::Step4 => "658263",
            Livestock::Dense => "486850",
        })
    }
}

pub enum IncreaseDecrease {
    Increase4,
    Increase3,
    Increase2,
    Increase1,
    Decrease1,
    Decrease2,
    Decrease3,
}

impl Colour for IncreaseDecrease {
    fn hex(&self) -> String {
        String::from(match self {
            IncreaseDecrease::Increase4 => "A74242",
            IncreaseDecrease::Increase3 => "B35A58",
            IncreaseDecrease::Increase2 => "C2877A",
            IncreaseDecrease::Increase1 => "BD9689",
            IncreaseDecrease::Decrease1 => "9DA7A1",
            IncreaseDecrease::Decrease2 => "8C9B9B",
            IncreaseDecrease::Decrease3 => "6A8289",
        })
    }
}

pub enum Population {
    Sparse,
    Step1,
    Step2,
    Step3,
    Step4,
    Dense,
}

impl Colour for Population {
    fn hex(&self) -> String {
        String::from(match self {
            Population::Sparse => "6A8289",
            Population::Step1 => "EBD684",
            Population::Step2 => "F0C18F",
            Population::Step3 => "D89A54",
            Population::Step4 => "C75530",
            Population::Dense => "B7311F",
        })
    }
}

pub enum Terrain {
    Sea6,
    Sea5,
    Sea4,
    Sea3,
    Sea2,
    Sea1,
    SeaLevelWater,
    SeaLevelLand,
    Land1,
    Land2,
    Land3,
    Land4,
    Land5,
    Land6,
    Land7,
    Land8,
    Land9,
}

impl Colour for Terrain {
    fn hex(&self) -> String {
        String::from(match self {
            Terrain::Sea6 => "D6E0D0",
            Terrain::Sea5 => "CCDDCD",
            Terrain::Sea4 => "C2D9CA",
            Terrain::Sea3 => "AAC9C1",
            Terrain::Sea2 => "9FC1BD",
            Terrain::Sea1 => "8CADAC",
            Terrain::SeaLevelWater => "81A4A5",
            Terrain::SeaLevelLand => "B3C6AB",
            Terrain::Land1 => "CAD1B1",
            Terrain::Land2 => "DFDABC",
            Terrain::Land3 => "EEE0BB",
            Terrain::Land4 => "CCA891",
            Terrain::Land5 => "BD907D",
            Terrain::Land6 => "B58676",
            Terrain::Land7 => "AE6C62",
            Terrain::Land8 => "AE6059",
            Terrain::Land9 => "A54B47",
        })
    }
}

use crate::ui::theme::{MapColour, TypographyColour};
use bevy::prelude::*;

pub enum Typography {
    Title,
    Subtitle,
    Body,
    BodyBold,
}

impl Typography {
    const fn font_handle(&self) -> &str {
        match self {
            Typography::Title => "fonts/Montserrat/Montserrat-Light.ttf",
            Typography::Subtitle => "fonts/Montserrat/Montserrat-Light.ttf",
            Typography::Body => "fonts/Montserrat/Montserrat-Light.ttf",
            Typography::BodyBold => "fonts/Montserrat/Montserrat-Light.ttf",
        }
    }

    fn colour(&self) -> Color {
        match self {
            Typography::Title => TypographyColour::Blue.into(),
            Typography::Subtitle => TypographyColour::Blue.into(),
            Typography::Body => TypographyColour::Black.into(),
            Typography::BodyBold => TypographyColour::Black.into(),
        }
    }

    fn font_size(&self) -> f32 {
        match self {
            Typography::Title => 46.0,
            Typography::Subtitle => 34.0,
            Typography::Body => 20.0,
            Typography::BodyBold => 20.0,
        }
    }

    pub fn with_section<S>(&self, section: S, asset_server: &AssetServer) -> Text
    where
        S: Into<String>,
    {
        Text::with_section(
            section,
            TextStyle {
                color: self.colour(),
                font_size: self.font_size(),
                font: asset_server.load(self.font_handle()),
            },
            Default::default(),
        )
    }
}

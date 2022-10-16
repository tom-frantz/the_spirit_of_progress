use crate::ui::theme::{Colour, IndustryColour, MenuColour};
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
            Typography::Title => "fonts/Pixolde/Pixolde-Bold.ttf",
            Typography::Subtitle => "fonts/Pixolde/Pixolde-Bold.ttf",
            Typography::Body => "fonts/Pixolde/Pixolde.ttf",
            Typography::BodyBold => "fonts/Pixolde/Pixolde-Bold.ttf",
        }
    }

    fn colour(&self) -> Color {
        match self {
            Typography::Title => IndustryColour::Purple.color(),
            Typography::Subtitle => IndustryColour::Purple.color(),
            Typography::Body => MenuColour::BlackPen.color(),
            Typography::BodyBold => MenuColour::BlackPen.color(),
        }
    }

    fn font_size(&self) -> f32 {
        match self {
            Typography::Title => 46.0,
            Typography::Subtitle => 34.0,
            Typography::Body => 16.0,
            Typography::BodyBold => 16.0,
        }
    }

    pub fn with_section<S>(&self, section: S, asset_server: &AssetServer) -> Text
    where
        S: Into<String>,
    {
        Text::from_section(
            section,
            TextStyle {
                color: self.colour(),
                font_size: self.font_size(),
                font: asset_server.load(self.font_handle()),
            },
        )
    }
}

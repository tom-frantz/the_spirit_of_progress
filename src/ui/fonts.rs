use crate::utils::colours::{MapColour, TypographyColour};
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
            Typography::Title => "fonts/Pragati_Narrow/PragatiNarrow-Bold.ttf",
            Typography::Subtitle => "fonts/Pragati_Narrow/PragatiNarrow-Regular.ttf",
            Typography::Body => "fonts/News_Cycle/NewsCycle-Regular.ttf",
            Typography::BodyBold => "fonts/News_Cycle/NewsCycle-Bold.ttf",
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

    pub fn with_section<S>(
        &self,
        section: S,
        style: TextStyle,
        alignment: TextAlignment,
        asset_server: &AssetServer,
    ) -> Text
    where
        S: Into<String>,
    {
        let color = if style.color == Color::default() {
            self.colour()
        } else {
            style.color
        };

        Text::with_section(
            section,
            TextStyle {
                color,
                font_size: 28.0,
                font: asset_server.load(self.font_handle()),
                ..style
            },
            alignment,
        )
    }
}

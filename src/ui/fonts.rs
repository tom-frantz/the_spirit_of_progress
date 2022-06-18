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
        Text::with_section(
            section,
            TextStyle {
                font: asset_server.load(self.font_handle()),
                ..style
            },
            alignment,
        )
    }
}

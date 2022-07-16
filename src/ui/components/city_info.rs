use crate::ui::fonts::Typography;
use crate::ui::primitives::sidebar::SIDEBAR_CONTENT_SIZE;
use crate::City;
use bevy::prelude::Val::*;
use bevy::prelude::*;

pub fn render_city_info(
    sidebar_content_node: &mut ChildBuilder,
    asset_server: &AssetServer,
    city: &City,
) {
    sidebar_content_node.spawn_bundle(TextBundle {
        text: Typography::Title.with_section(city.name.clone(), asset_server),
        ..default()
    });
    sidebar_content_node.spawn_bundle(TextBundle {
        style: Style {
            border: Rect::all(Px(5.0)),
            size: Size::new(Px(SIDEBAR_CONTENT_SIZE), Auto),
            ..default()
        },
        text: Typography::Body.with_section(
            "This is evidently a very long string and I'd hate to see what I think might happen.",
            asset_server,
        ),
        ..default()
    });
}

use bevy::prelude::*;

#[derive(Debug, Default, Copy, Clone)]
pub struct StyleBuilder {
    flex_grow: bool,
    flex_direction: FlexDirection,

    align_items: AlignItems,
    align_self: AlignSelf,
    justify_content: JustifyContent,

    size: Size<Val>,
    margin: UiRect<Val>,
    padding: UiRect<Val>,
}

impl StyleBuilder {
    pub fn new() -> StyleBuilder {
        StyleBuilder {
            // Override stupid stretch. It annoys me.
            justify_content: JustifyContent::FlexStart,
            ..default()
        }
    }

    pub fn build(&mut self) -> Style {
        Style {
            flex_grow: if self.flex_grow { 1.0 } else { 0.0 },
            flex_direction: self.flex_direction,

            align_items: self.align_items,
            align_self: self.align_self,

            justify_content: self.justify_content,

            size: self.size,
            margin: self.margin,
            padding: self.padding,
            ..default()
        }
    }

    pub fn build_clear_node_bundle(&mut self) -> NodeBundle {
        NodeBundle {
            color: Color::NONE.into(),
            style: self.build(),
            ..default()
        }
    }

    //
    // FLEX
    //
    pub fn flex_grow(&mut self, grow: bool) -> &mut Self {
        self.flex_grow = grow;
        self
    }
    pub fn flex_direction(&mut self, direction: FlexDirection) -> &mut Self {
        self.flex_direction = direction;
        self
    }
    pub fn column(&mut self) -> &mut Self {
        self.flex_direction = FlexDirection::ColumnReverse;
        self
    }
    pub fn row(&mut self) -> &mut Self {
        self.flex_direction = FlexDirection::Row;
        self
    }

    pub fn align_items(&mut self, align_items: AlignItems) -> &mut Self {
        self.align_items = align_items;
        self
    }
    pub fn align_self(&mut self, align_self: AlignSelf) -> &mut Self {
        self.align_self = align_self;
        self
    }

    pub fn justify_content(&mut self, justify_content: JustifyContent) -> &mut Self {
        self.justify_content = justify_content;
        self
    }

    //
    // SIZE
    //
    pub fn size(&mut self, width: Val, height: Val) -> &mut Self {
        self.size = Size::new(width, height);
        self
    }

    //
    // MARGIN
    //
    pub fn margin(&mut self, val: Val) -> &mut Self {
        self.margin = UiRect::all(val);
        self
    }
    pub fn margin_top(&mut self, val: Val) -> &mut Self {
        self.margin.top = val;
        self
    }
    pub fn margin_bottom(&mut self, val: Val) -> &mut Self {
        self.margin.bottom = val;
        self
    }
    pub fn margin_left(&mut self, val: Val) -> &mut Self {
        self.margin.left = val;
        self
    }
    pub fn margin_right(&mut self, val: Val) -> &mut Self {
        self.margin.right = val;
        self
    }

    //
    // PADDING
    //
    pub fn padding(&mut self, val: Val) -> &mut Self {
        self.padding = UiRect::all(val);
        self
    }
    pub fn padding_top(&mut self, val: Val) -> &mut Self {
        self.padding.top = val;
        self
    }
    pub fn padding_bottom(&mut self, val: Val) -> &mut Self {
        self.padding.bottom = val;
        self
    }
    pub fn padding_left(&mut self, val: Val) -> &mut Self {
        self.padding.left = val;
        self
    }
    pub fn padding_right(&mut self, val: Val) -> &mut Self {
        self.padding.right = val;
        self
    }
}

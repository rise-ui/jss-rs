use types::{PropertyKey, LayoutKey, AppearanceKey};
use hashbrown::HashMap;

lazy_static! {
    static ref LAYOUT_PROPERTIES: HashMap<LayoutKey, &'static str> = {
        let mut map = HashMap::new();

        map.insert(LayoutKey::FlexDirection, "FlexDirection");
        map.insert(LayoutKey::JustifyContent, "Justify");
        map.insert(LayoutKey::Position, "PositionType");
        map.insert(LayoutKey::AlignContent, "Align");
        map.insert(LayoutKey::AlignItems, "Align");
        map.insert(LayoutKey::AlignSelf, "Align");
        map.insert(LayoutKey::FlexWrap, "Wrap");
        map.insert(LayoutKey::Display, "Display");
        map.insert(LayoutKey::Overflow, "Overflow");
        map.insert(LayoutKey::AspectRatio, "AspectRatio");
        map.insert(LayoutKey::FlexShrink, "FlexShrink");
        map.insert(LayoutKey::FlexGrow, "FlexGrow");
        map.insert(LayoutKey::Flex, "FlexFactor");
        map.insert(LayoutKey::Bottom, "SharedUnit");
        map.insert(LayoutKey::End, "SharedUnit");
        map.insert(LayoutKey::FlexBasis, "SharedUnit");
        map.insert(LayoutKey::Height, "SharedUnit");
        map.insert(LayoutKey::Left, "SharedUnit");
        map.insert(LayoutKey::Margin, "SharedUnit");
        map.insert(LayoutKey::MarginBottom, "SharedUnit");
        map.insert(LayoutKey::MarginEnd, "SharedUnit");
        map.insert(LayoutKey::MarginHorizontal, "SharedUnit");
        map.insert(LayoutKey::MarginLeft, "SharedUnit");
        map.insert(LayoutKey::MarginRight, "SharedUnit");
        map.insert(LayoutKey::MarginStart, "SharedUnit");
        map.insert(LayoutKey::MarginTop, "SharedUnit");
        map.insert(LayoutKey::MarginVertical, "SharedUnit");
        map.insert(LayoutKey::MaxHeight, "SharedUnit");
        map.insert(LayoutKey::MaxWidth, "SharedUnit");
        map.insert(LayoutKey::MinHeight, "SharedUnit");
        map.insert(LayoutKey::MinWidth, "SharedUnit");
        map.insert(LayoutKey::Padding, "SharedUnit");
        map.insert(LayoutKey::PaddingBottom, "SharedUnit");
        map.insert(LayoutKey::PaddingEnd, "SharedUnit");
        map.insert(LayoutKey::PaddingHorizontal, "SharedUnit");
        map.insert(LayoutKey::PaddingLeft, "SharedUnit");
        map.insert(LayoutKey::PaddingRight, "SharedUnit");
        map.insert(LayoutKey::PaddingStart, "SharedUnit");
        map.insert(LayoutKey::PaddingTop, "SharedUnit");
        map.insert(LayoutKey::PaddingVertical, "SharedUnit");
        map.insert(LayoutKey::Right, "SharedUnit");
        map.insert(LayoutKey::Start, "SharedUnit");
        map.insert(LayoutKey::Top, "SharedUnit");
        map.insert(LayoutKey::Width, "SharedUnit");
        map.insert(LayoutKey::BorderBottomWidth, "BorderWidth");
        map.insert(LayoutKey::BorderRightWidth, "BorderWidth");
        map.insert(LayoutKey::BorderLeftWidth, "BorderWidth");
        map.insert(LayoutKey::BorderTopWidth, "BorderWidth");

        map
    };

    static ref APPEARANCE_PROPERTIES: HashMap<AppearanceKey, &'static str> = {
        let mut map = HashMap::new();

        map.insert(AppearanceKey::Background, "Background");
        map.insert(AppearanceKey::Transform, "Transforms");
        map.insert(AppearanceKey::Filter, "Filters");

        map.insert(AppearanceKey::BorderTopColor, "BorderColor");
        map.insert(AppearanceKey::BorderRightColor, "BorderColor");
        map.insert(AppearanceKey::BorderLeftColor, "BorderColor");
        map.insert(AppearanceKey::BorderBottomColor, "BorderColor");

        map.insert(AppearanceKey::BorderTopStyle, "BorderStyle");
        map.insert(AppearanceKey::BorderRightStyle, "BorderStyle");
        map.insert(AppearanceKey::BorderLeftStyle, "BorderStyle");
        map.insert(AppearanceKey::BorderBottomStyle, "BorderStyle");

        map.insert(AppearanceKey::BorderTopRightRadius, "BorderRadius");
        map.insert(AppearanceKey::BorderTopLeftRadius, "BorderRadius");
        map.insert(AppearanceKey::BorderBottomRightRadius, "BorderRadius");
        map.insert(AppearanceKey::BorderBottomLeftRadius, "BorderRadius");

        map
    };
}

pub fn get_appearance_property_key(key: &str) -> Option<AppearanceKey> {
    match key {
        "background" => Some(AppearanceKey::Background),
        "transform" => Some(AppearanceKey::Transform),
        "filter" => Some(AppearanceKey::Filter),

        "border_top_color" => Some(AppearanceKey::BorderTopColor),
        "border_right_color" => Some(AppearanceKey::BorderRightColor),
        "border_left_color" => Some(AppearanceKey::BorderLeftColor),
        "border_bottom_color" => Some(AppearanceKey::BorderBottomColor),
        
        "border_top_style" => Some(AppearanceKey::BorderTopStyle),
        "border_right_style" => Some(AppearanceKey::BorderRightStyle),
        "border_left_style" => Some(AppearanceKey::BorderLeftStyle),
        "border_bottom_style" => Some(AppearanceKey::BorderBottomStyle),

        "border_top_right_radius" => Some(AppearanceKey::BorderTopRightRadius),
        "border_top_left_radius" => Some(AppearanceKey::BorderTopLeftRadius),
        "border_bottom_right_radius" => Some(AppearanceKey::BorderBottomRightRadius),
        "border_bottom_left_radius" => Some(AppearanceKey::BorderBottomLeftRadius),
        _ => None,
    }
}

pub fn get_layout_property_key(key: &str) -> Option<LayoutKey> {
    match key {
        "flex_direction" => Some(LayoutKey::FlexDirection),
        "justify_content" => Some(LayoutKey::JustifyContent),
        "position" => Some(LayoutKey::Position),
        "align_content" => Some(LayoutKey::AlignContent),
        "align_items" => Some(LayoutKey::AlignItems),
        "align_self" => Some(LayoutKey::AlignSelf),
        "flex_wrap" => Some(LayoutKey::FlexWrap),
        "display" => Some(LayoutKey::Display),
        "overflow" => Some(LayoutKey::Overflow),
        "aspect_ratio" => Some(LayoutKey::AspectRatio),
        "flex_shrink" => Some(LayoutKey::FlexShrink),
        "flex_grow" => Some(LayoutKey::FlexGrow),
        "flex" => Some(LayoutKey::Flex),
        "bottom" => Some(LayoutKey::Bottom),
        "end" => Some(LayoutKey::End),
        "flex_basis" => Some(LayoutKey::FlexBasis),
        "height" => Some(LayoutKey::Height),
        "left" => Some(LayoutKey::Left),
        
        "margin" => Some(LayoutKey::Margin),
        "margin_bottom" => Some(LayoutKey::MarginBottom),
        "margin_end" => Some(LayoutKey::MarginEnd),
        "margin_horizontal" => Some(LayoutKey::MarginHorizontal),
        "margin_left" => Some(LayoutKey::MarginLeft),
        "margin_right" => Some(LayoutKey::MarginRight),
        "margin_start" => Some(LayoutKey::MarginStart),
        "margin_top" => Some(LayoutKey::MarginTop),
        "margin_vertical" => Some(LayoutKey::MarginVertical),
        
        "max_height" => Some(LayoutKey::MaxHeight),
        "max_width" => Some(LayoutKey::MaxWidth),
        "min_height" => Some(LayoutKey::MinHeight),
        "min_width" => Some(LayoutKey::MinWidth),
        
        "padding" => Some(LayoutKey::Padding),
        "padding_bottom" => Some(LayoutKey::PaddingBottom),
        "padding_end" => Some(LayoutKey::PaddingEnd),

        "padding_horizontal" => Some(LayoutKey::PaddingHorizontal),
        "padding_left" => Some(LayoutKey::PaddingLeft),
        "padding_right" => Some(LayoutKey::PaddingRight),
        "padding_start" => Some(LayoutKey::PaddingStart),
        "padding_top" => Some(LayoutKey::PaddingTop),
        "padding_vertical" => Some(LayoutKey::PaddingVertical),

        "right" => Some(LayoutKey::Right),
        "start" => Some(LayoutKey::Start),
        "top" => Some(LayoutKey::Top),
        "width" => Some(LayoutKey::Width),

        "border_bottom_width" => Some(LayoutKey::BorderBottomWidth),
        "border_right_width" => Some(LayoutKey::BorderRightWidth),
        "border_left_width" => Some(LayoutKey::BorderLeftWidth),
        "border_top_width" => Some(LayoutKey::BorderTopWidth),

        _ => None,
    }
}

pub fn get_reflect_property_type(key: &PropertyKey) -> String {
    match key {
        PropertyKey::Appearance(key) => APPEARANCE_PROPERTIES[&key].to_string(),
        PropertyKey::Layout(key) => LAYOUT_PROPERTIES[&key].to_string(),
    }
}

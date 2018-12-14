use types::{Appearance, PropertyValue, SharedUnit};
use properties::Color;
use yoga::StyleUnit;

impl_union_into_appearance!(BorderStyle);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BorderRadius {
    pub bottom_right: SharedUnit,
    pub bottom_left: SharedUnit,
    pub top_right: SharedUnit,
    pub top_left: SharedUnit,
}

impl Default for BorderRadius {
    fn default() -> BorderRadius {
        BorderRadius {
            bottom_right: SharedUnit::StyleUnit(StyleUnit::Point(0.0.into())),
            bottom_left: SharedUnit::StyleUnit(StyleUnit::Point(0.0.into())),
            top_right: SharedUnit::StyleUnit(StyleUnit::Point(0.0.into())),
            top_left: SharedUnit::StyleUnit(StyleUnit::Point(0.0.into())),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Border {
    pub style: BorderStyle,
    pub color: Color,
    pub width: f32,
}

impl Default for Border {
    fn default() -> Border {
        Border {
            style: BorderStyle::None,
            color: Color::transparent(),
            width: 0.0,
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BorderStyles {
    pub bottom: Border,
    pub right: Border,
    pub left: Border,
    pub top: Border,
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BorderStyle {
    None,
    Solid,
    Double,
    Dotted,
    Dashed,
    Hidden,
    Groove,
    Ridge,
    Inset,
    Outset,
}

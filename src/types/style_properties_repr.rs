use erased_serde::{Deserializer, deserialize};
use types::property_types::*;
use types::SharedUnit;

/// Style properties struct with all css properties,
/// by default allow to raw serialize/deserialize with serde.
/// All elements wrap over Option<T> for support optional field.
/// Also implements derive macro for Add<T> operator for merge properties,
/// and custom macros for relative parse & prepares for styles
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, ImplPropertySetters, ImplStyleParser)]
#[serde(rename_all = "kebab-case")]
pub struct StyleProperties {
    // Layout Styles
    pub flex_direction: Option<FlexDirection>,
    pub justify_content: Option<Justify>,
    pub position: Option<PositionType>,
    pub align_content: Option<Align>,
    pub align_items: Option<Align>,
    pub align_self: Option<Align>,
    pub flex_wrap: Option<Wrap>,
    pub display: Option<Display>,
    pub overflow: Option<Overflow>,

    pub aspect_ratio: Option<AspectRatio>,
    pub flex_shrink: Option<FlexShrink>,
    pub flex_grow: Option<FlexGrow>,
    pub flex: Option<FlexFactor>,

    pub bottom: Option<SharedUnit>,
    pub end: Option<SharedUnit>,
    pub flex_basis: Option<SharedUnit>,
    pub height: Option<SharedUnit>,
    pub left: Option<SharedUnit>,
    pub margin: Option<SharedUnit>,
    pub margin_bottom: Option<SharedUnit>,
    pub margin_end: Option<SharedUnit>,
    pub margin_horizontal: Option<SharedUnit>,
    pub margin_left: Option<SharedUnit>,
    pub margin_right: Option<SharedUnit>,
    pub margin_start: Option<SharedUnit>,
    pub margin_top: Option<SharedUnit>,
    pub margin_vertical: Option<SharedUnit>,
    pub max_height: Option<SharedUnit>,
    pub max_width: Option<SharedUnit>,
    pub min_height: Option<SharedUnit>,
    pub min_width: Option<SharedUnit>,
    pub padding: Option<SharedUnit>,
    pub padding_bottom: Option<SharedUnit>,
    pub padding_end: Option<SharedUnit>,
    pub padding_horizontal: Option<SharedUnit>,
    pub padding_left: Option<SharedUnit>,
    pub padding_right: Option<SharedUnit>,
    pub padding_start: Option<SharedUnit>,
    pub padding_top: Option<SharedUnit>,
    pub padding_vertical: Option<SharedUnit>,
    pub right: Option<SharedUnit>,
    pub start: Option<SharedUnit>,
    pub top: Option<SharedUnit>,
    pub width: Option<SharedUnit>,

    pub border_bottom_width: Option<BorderWidth>,
    pub border_right_width: Option<BorderWidth>,
    pub border_left_width: Option<BorderWidth>,
    pub border_top_width: Option<BorderWidth>,

    // Appearance Styles
    pub background: Option<Background>,
    pub transform: Option<Transforms>,
    pub filter: Option<Filters>,

    // Borders
    pub border_top_color: Option<BorderColor>,
    pub border_right_color: Option<BorderColor>,
    pub border_left_color: Option<BorderColor>,
    pub border_bottom_color: Option<BorderColor>,

    pub border_top_style: Option<BorderStyle>,
    pub border_right_style: Option<BorderStyle>,
    pub border_left_style: Option<BorderStyle>,
    pub border_bottom_style: Option<BorderStyle>,

    pub border_top_right_radius: Option<BorderRadius>,
    pub border_top_left_radius: Option<BorderRadius>,
    pub border_bottom_right_radius: Option<BorderRadius>,
    pub border_bottom_left_radius: Option<BorderRadius>,
}

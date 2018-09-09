use types::property_types::*;
use properties::unit;

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

    #[serde(default)]
    #[serde(with = "unit")]
    pub bottom: Option<StyleUnit>,
    #[serde(default)]
    #[serde(with = "unit")]
    pub end: Option<StyleUnit>,
    #[serde(default)]
    #[serde(with = "unit")]
    pub flex_basis: Option<StyleUnit>,
    #[serde(default)]
    #[serde(with = "unit")]
    pub height: Option<StyleUnit>,
    #[serde(default)]
    #[serde(with = "unit")]
    pub left: Option<StyleUnit>,
    #[serde(default)]
    #[serde(with = "unit")]
    pub margin: Option<StyleUnit>,
    #[serde(default)]
    #[serde(with = "unit")]
    pub margin_bottom: Option<StyleUnit>,
    #[serde(default)]
    #[serde(with = "unit")]
    pub margin_end: Option<StyleUnit>,
    #[serde(default)]
    #[serde(with = "unit")]
    pub margin_horizontal: Option<StyleUnit>,
    #[serde(default)]
    #[serde(with = "unit")]
    pub margin_left: Option<StyleUnit>,
    #[serde(default)]
    #[serde(with = "unit")]
    pub margin_right: Option<StyleUnit>,
    #[serde(default)]
    #[serde(with = "unit")]
    pub margin_start: Option<StyleUnit>,
    #[serde(default)]
    #[serde(with = "unit")]
    pub margin_top: Option<StyleUnit>,
    #[serde(default)]
    #[serde(with = "unit")]
    pub margin_vertical: Option<StyleUnit>,
    #[serde(default)]
    #[serde(with = "unit")]
    pub max_height: Option<StyleUnit>,
    #[serde(default)]
    #[serde(with = "unit")]
    pub max_width: Option<StyleUnit>,
    #[serde(default)]
    #[serde(with = "unit")]
    pub min_height: Option<StyleUnit>,
    #[serde(default)]
    #[serde(with = "unit")]
    pub min_width: Option<StyleUnit>,
    #[serde(default)]
    #[serde(with = "unit")]
    pub padding: Option<StyleUnit>,
    #[serde(default)]
    #[serde(with = "unit")]
    pub padding_bottom: Option<StyleUnit>,
    #[serde(default)]
    #[serde(with = "unit")]
    pub padding_end: Option<StyleUnit>,
    #[serde(default)]
    #[serde(with = "unit")]
    pub padding_horizontal: Option<StyleUnit>,
    #[serde(default)]
    #[serde(with = "unit")]
    pub padding_left: Option<StyleUnit>,
    #[serde(default)]
    #[serde(with = "unit")]
    pub padding_right: Option<StyleUnit>,
    #[serde(default)]
    #[serde(with = "unit")]
    pub padding_start: Option<StyleUnit>,
    #[serde(default)]
    #[serde(with = "unit")]
    pub padding_top: Option<StyleUnit>,
    #[serde(default)]
    #[serde(with = "unit")]
    pub padding_vertical: Option<StyleUnit>,
    #[serde(default)]
    #[serde(with = "unit")]
    pub right: Option<StyleUnit>,
    #[serde(default)]
    #[serde(with = "unit")]
    pub start: Option<StyleUnit>,
    #[serde(default)]
    #[serde(with = "unit")]
    pub top: Option<StyleUnit>,
    #[serde(default)]
    #[serde(with = "unit")]
    pub width: Option<StyleUnit>,

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

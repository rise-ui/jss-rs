use ordered_float::OrderedFloat;
use common::PrepareStyleExt;
use common::unit as parse_unit;

use yoga::{
  Align,
  Display,
  FlexDirection,
  FlexStyle,
  Justify,
  Overflow,
  PositionType,
  StyleUnit,
  Wrap
};

use properties::{
  BorderRadius,
  BorderStyles,
  BorderStyle,
  Background,
  Transforms,
  Apperance,
  Filters,
  Color,
};

pub type BorderWidth = OrderedFloat<f32>;
pub type AspectRatio = OrderedFloat<f32>;
pub type FlexShrink = OrderedFloat<f32>;
pub type FlexFactor = OrderedFloat<f32>;
pub type FlexGrow = OrderedFloat<f32>;

#[derive(Serialize, Deserialize, Debug, Clone, PrepareStyle, MergeStyle)]
#[serde(rename_all = "snake_case")]
pub struct ElementStyle {
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

  pub border_bottom: Option<BorderWidth>,
  pub border_right: Option<BorderWidth>,
  pub border_left: Option<BorderWidth>,
  pub border_top: Option<BorderWidth>,

  pub aspect_ratio: Option<AspectRatio>,
  pub flex_shrink: Option<FlexShrink>,
  pub flex_grow: Option<FlexGrow>,
  pub flex: Option<FlexFactor>,

  #[serde(default)]
  #[serde(with = "parse_unit")]
  pub bottom: Option<StyleUnit>,
  #[serde(default)]
  #[serde(with = "parse_unit")]
  pub end: Option<StyleUnit>,
  #[serde(default)]
  #[serde(with = "parse_unit")]
  pub flex_basis: Option<StyleUnit>,
  #[serde(default)]
  #[serde(with = "parse_unit")]
  pub height: Option<StyleUnit>,
  #[serde(default)]
  #[serde(with = "parse_unit")]
  pub left: Option<StyleUnit>,
  #[serde(default)]
  #[serde(with = "parse_unit")]
  pub margin: Option<StyleUnit>,
  #[serde(default)]
  #[serde(with = "parse_unit")]
  pub margin_bottom: Option<StyleUnit>,
  #[serde(default)]
  #[serde(with = "parse_unit")]
  pub margin_end: Option<StyleUnit>,
  #[serde(default)]
  #[serde(with = "parse_unit")]
  pub margin_horizontal: Option<StyleUnit>,
  #[serde(default)]
  #[serde(with = "parse_unit")]
  pub margin_left: Option<StyleUnit>,
  #[serde(default)]
  #[serde(with = "parse_unit")]
  pub margin_right: Option<StyleUnit>,
  #[serde(default)]
  #[serde(with = "parse_unit")]
  pub margin_start: Option<StyleUnit>,
  #[serde(default)]
  #[serde(with = "parse_unit")]
  pub margin_top: Option<StyleUnit>,
  #[serde(default)]
  #[serde(with = "parse_unit")]
  pub margin_vertical: Option<StyleUnit>,
  #[serde(default)]
  #[serde(with = "parse_unit")]
  pub max_height: Option<StyleUnit>,
  #[serde(default)]
  #[serde(with = "parse_unit")]
  pub max_width: Option<StyleUnit>,
  #[serde(default)]
  #[serde(with = "parse_unit")]
  pub min_height: Option<StyleUnit>,
  #[serde(default)]
  #[serde(with = "parse_unit")]
  pub min_width: Option<StyleUnit>,
  #[serde(default)]
  #[serde(with = "parse_unit")]
  pub padding: Option<StyleUnit>,
  #[serde(default)]
  #[serde(with = "parse_unit")]
  pub padding_bottom: Option<StyleUnit>,
  #[serde(default)]
  #[serde(with = "parse_unit")]
  pub padding_end: Option<StyleUnit>,
  #[serde(default)]
  #[serde(with = "parse_unit")]
  pub padding_horizontal: Option<StyleUnit>,
  #[serde(default)]
  #[serde(with = "parse_unit")]
  pub padding_left: Option<StyleUnit>,
  #[serde(default)]
  #[serde(with = "parse_unit")]
  pub padding_right: Option<StyleUnit>,
  #[serde(default)]
  #[serde(with = "parse_unit")]
  pub padding_start: Option<StyleUnit>,
  #[serde(default)]
  #[serde(with = "parse_unit")]
  pub padding_top: Option<StyleUnit>,
  #[serde(default)]
  #[serde(with = "parse_unit")]
  pub padding_vertical: Option<StyleUnit>,
  #[serde(default)]
  #[serde(with = "parse_unit")]
  pub right: Option<StyleUnit>,
  #[serde(default)]
  #[serde(with = "parse_unit")]
  pub start: Option<StyleUnit>,
  #[serde(default)]
  #[serde(with = "parse_unit")]
  pub top: Option<StyleUnit>,
  #[serde(default)]
  #[serde(with = "parse_unit")]
  pub width: Option<StyleUnit>,

  // Apperance Styles
  pub background: Option<Background>,
  pub transform: Option<Transforms>,
  pub filter: Option<Filters>,

  // Borders
  pub border_top_color: Option<Color>,
  pub border_right_color: Option<Color>,
  pub border_left_color: Option<Color>,
  pub border_bottom_color: Option<Color>,

  pub border_top_style: Option<BorderStyle>,
  pub border_right_style: Option<BorderStyle>,
  pub border_left_style: Option<BorderStyle>,
  pub border_bottom_style: Option<BorderStyle>,

  pub border_top_right_radius: Option<i32>,
  pub border_top_left_radius: Option<i32>,
  pub border_bottom_right_radius: Option<i32>,
  pub border_bottom_left_radius: Option<i32>,
}

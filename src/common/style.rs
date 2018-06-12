use yoga::{Align, Display, FlexDirection, FlexStyle, Justify, Overflow, PositionType, StyleUnit, Wrap};
use properties::{Apperance, Background, BorderRadius, BorderStyle, BorderStyles, Color, Filter};
use ordered_float::OrderedFloat;
use common::PrepareStyleExt;

mod parse_unit {
  use serde::{Deserialize, Deserializer, Serializer};
  use serde_json::Value;
  use yoga::StyleUnit;

  pub fn serialize<S>(date: &Option<StyleUnit>, s: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    if let Some(ref d) = *date {
      match &d {
        StyleUnit::Percent(number) => return s.serialize_str(&*format!("{}%", number.into_inner() as i32)),
        StyleUnit::Point(number) => return s.serialize_str(&*format!("{}px", number.into_inner() as i32)),
        StyleUnit::Auto | StyleUnit::UndefinedValue => return s.serialize_str("auto"),
      }
    }

    s.serialize_none()
  }

  pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<StyleUnit>, D::Error>
  where
    D: Deserializer<'de>,
  {
    let value = Value::deserialize(deserializer)?;
    match value {
      Value::String(unit_value) => Ok(Some(unit_value.into())),
      _ => Ok(None),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PrepareStyle)]
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

  pub aspect_ratio: Option<OrderedFloat<f32>>,
  pub border_bottom: Option<OrderedFloat<f32>>,
  pub border_right: Option<OrderedFloat<f32>>,
  pub border_left: Option<OrderedFloat<f32>>,
  pub border_top: Option<OrderedFloat<f32>>,

  pub border_start: Option<OrderedFloat<f32>>,
  pub border_end: Option<OrderedFloat<f32>>,
  pub border: Option<OrderedFloat<f32>>,

  pub flex_shrink: Option<OrderedFloat<f32>>,
  pub flex_grow: Option<OrderedFloat<f32>>,
  pub flex: Option<OrderedFloat<f32>>,

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

  // Filters
  pub filter: Option<Vec<Filter>>,
}

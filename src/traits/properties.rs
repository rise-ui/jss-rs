use std::fmt::Debug;
use failure::Error;

use types::{Apperance, PropertyValue, Layout};
use types::PropertyError;
use yoga::FlexStyle;

pub trait TStyle: Debug + PartialEq + Clone {
  fn get_apperance_style(&self, &str) -> Option<&Apperance>;
  fn get_layout_style(&self, &str) -> Option<&Layout>;

  fn set_apperance_style(&mut self, &str, Option<Apperance>) -> Result<(), PropertyError>;
  fn set_layout_style(&mut self, &str, Option<Layout>) -> Result<(), PropertyError>;
  fn set_style(&mut self, &str, PropertyValue) -> Result<(), PropertyError>;
}

// trait_properties!({
//   flex_direction: FlexDirection,
//   justify_content: Justify,
//   position: PositionType,
//   align_content: Align,
//   align_items: Align,
//   align_self: Align,
//   flex_wrap: Wrap,
//   display: Display,
//   overflow: Overflow,

//   aspect_ratio: AspectRatio,
//   flex_shrink: FlexShrink,
//   flex_grow: FlexGrow,
//   flex: FlexFactor,

//   bottom: StyleUnit,
//   end: StyleUnit,
//   flex_basis: StyleUnit,
//   height: StyleUnit,
//   left: StyleUnit,
//   margin: StyleUnit,
//   margin_bottom: StyleUnit,
//   margin_end: StyleUnit,
//   margin_horizontal: StyleUnit,
//   margin_left: StyleUnit,
//   margin_right: StyleUnit,
//   margin_start: StyleUnit,
//   margin_top: StyleUnit,
//   margin_vertical: StyleUnit,
//   max_height: StyleUnit,
//   max_width: StyleUnit,
//   min_height: StyleUnit,
//   min_width: StyleUnit,
//   padding: StyleUnit,
//   padding_bottom: StyleUnit,
//   padding_end: StyleUnit,
//   padding_horizontal: StyleUnit,
//   padding_left: StyleUnit,
//   padding_right: StyleUnit,
//   padding_start: StyleUnit,
//   padding_top: StyleUnit,
//   padding_vertical: StyleUnit,
//   right: StyleUnit,
//   start: StyleUnit,
//   top: StyleUnit,
//   width: StyleUnit,

// // Apperance Styles
//   background: Background,
//   transform: Transforms,
//   filter: Filters,

// // Borders
//   border_bottom_width: BorderWidth,
//   border_right_width: BorderWidth,
//   border_left_width: BorderWidth,
//   border_top_width: BorderWidth,

//   border_top_color: BorderColor,
//   border_right_color: BorderColor,
//   border_left_color: BorderColor,
//   border_bottom_color: BorderColor,

//   border_top_style: BorderStyle,
//   border_right_style: BorderStyle,
//   border_left_style: BorderStyle,
//   border_bottom_style: BorderStyle,

//   border_top_right_radius: BorderRadius,
//   border_top_left_radius: BorderRadius,
//   border_bottom_right_radius: BorderRadius,
//   border_bottom_left_radius: BorderRadius
// });

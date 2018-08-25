use std::collections::HashMap;
use types::property_types::*;
use types::PropertyError;
use inflector::Inflector;
use traits::TStyle;

lazy_static! {
  static ref APPERANCE_KEYS: Vec<&'static str> = vec![
    "background",
    "transform",
    "filter",
    "border_top_color",
    "border_right_color",
    "border_left_color",
    "border_bottom_color",
    "border_top_style",
    "border_right_style",
    "border_left_style",
    "border_bottom_style",
    "border_top_right_radius",
    "border_top_left_radius",
    "border_bottom_right_radius",
    "border_bottom_left_radius",
  ];
  static ref LAYOUT_KEYS: Vec<&'static str> = vec![
    "flex_direction",
    "justify_content",
    "position",
    "align_content",
    "align_items",
    "align_self",
    "flex_wrap",
    "display",
    "overflow",
    "aspect_ratio",
    "flex_shrink",
    "flex_grow",
    "flex",
    "bottom",
    "end",
    "flex_basis",
    "height",
    "left",
    "margin",
    "margin_bottom",
    "margin_end",
    "margin_horizontal",
    "margin_left",
    "margin_right",
    "margin_start",
    "margin_top",
    "margin_vertical",
    "max_height",
    "max_width",
    "min_height",
    "min_width",
    "padding",
    "padding_bottom",
    "padding_end",
    "padding_horizontal",
    "padding_left",
    "padding_right",
    "padding_start",
    "padding_top",
    "padding_vertical",
    "right",
    "start",
    "top",
    "width",
    "border_bottom_width",
    "border_right_width",
    "border_left_width",
    "border_top_width",
  ];
}

#[derive(Debug, Clone, PartialEq)]
pub enum Apperance {
  BorderRadius(BorderRadius),
  BorderColor(BorderColor),
  BorderStyle(BorderStyle),
  Background(Background),
  Transforms(Transforms),
  Filters(Filters),
  Auto,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Layout {
  FlexDirection(FlexDirection),
  PositionType(PositionType),
  AspectRatio(AspectRatio),
  BorderWidth(BorderWidth),
  FlexShrink(FlexShrink),
  FlexFactor(FlexFactor),
  StyleUnit(StyleUnit),
  Overflow(Overflow),
  FlexGrow(FlexGrow),
  Display(Display),
  Justify(Justify),
  Align(Align),
  Wrap(Wrap),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
  Apperance(Apperance),
  Layout(Layout),
}

pub type PropertiesApperance = PropertiesStore<Apperance>;
pub type PropertiesLayout = PropertiesStore<Layout>;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PropertiesStore<T>(HashMap<String, T>);

#[derive(Debug, Clone, PartialEq)]
pub struct Properties {
  apperance: PropertiesApperance,
  layout: PropertiesLayout,
}

/* _____________________________________________________________________ */

impl TStyle for Properties {
  fn get_apperance_style(&self, name: &str) -> Option<&Apperance> {
    self.apperance.0.get(name)
  }

  fn get_layout_style(&self, name: &str) -> Option<&Layout> {
    self.layout.0.get(name)
  }

  fn set_style(&mut self, name: &str, property: PropertyValue) -> Result<(), PropertyError> {
    if APPERANCE_KEYS.contains(&name) {
      self.set_apperance_style(name, extract!(PropertyValue::Apperance(_), property))
    } else if LAYOUT_KEYS.contains(&name) {
      self.set_layout_style(name, extract!(PropertyValue::Layout(_), property))
    } else {
      Err(PropertyError::InvalidKey {
        key: name.to_string(),
      })
    }
  }

  fn set_apperance_style(&mut self, name: &str, property: Option<Apperance>) -> Result<(), PropertyError> {
    if property.is_none() {
      if let Some(removed) = self.apperance.0.remove(name) {
        // @TODO: debug log inside this
      }

      return Ok(());
    }

    let property = property.unwrap();
    style_setters!((self, name, property, Apperance, apperance) {
      Background: Background,
      Transform: Transforms,
      Filter: Filters,

      BorderTopColor: BorderColor,
      BorderRightColor: BorderColor,
      BorderLeftColor: BorderColor,
      BorderBottomColor: BorderColor,

      BorderTopStyle: BorderStyle,
      BorderRightStyle: BorderStyle,
      BorderLeftStyle: BorderStyle,
      BorderBottomStyle: BorderStyle,

      BorderTopRightRadius: BorderRadius,
      BorderTopLeftRadius: BorderRadius,
      BorderBottomRightRadius: BorderRadius,
      BorderBottomLeftRadius: BorderRadius
    })
  }

  fn set_layout_style(&mut self, name: &str, property: Option<Layout>) -> Result<(), PropertyError> {
    if property.is_none() {
      if let Some(removed) = self.layout.0.remove(name) {
        // @TODO: debug log inside this
      }

      return Ok(());
    }

    let property = property.unwrap();
    style_setters!((self, name, property, Layout, layout) {
      FlexDirection: FlexDirection,
      JustifyContent: Justify,
      Position: PositionType,
      AlignContent: Align,
      AlignItems: Align,
      AlignSelf: Align,
      FlexWrap: Wrap,
      Display: Display,
      Overflow: Overflow,

      AspectRatio: AspectRatio,
      FlexShrink: FlexShrink,
      FlexGrow: FlexGrow,
      Flex: FlexFactor,

      Bottom: StyleUnit,
      End: StyleUnit,
      FlexBasis: StyleUnit,
      Height: StyleUnit,
      Left: StyleUnit,
      Margin: StyleUnit,
      MarginBottom: StyleUnit,
      MarginEnd: StyleUnit,
      MarginHorizontal: StyleUnit,
      MarginLeft: StyleUnit,
      MarginRight: StyleUnit,
      MarginStart: StyleUnit,
      MarginTop: StyleUnit,
      MarginVertical: StyleUnit,
      MaxHeight: StyleUnit,
      MaxWidth: StyleUnit,
      MinHeight: StyleUnit,
      MinWidth: StyleUnit,
      Padding: StyleUnit,
      PaddingBottom: StyleUnit,
      PaddingEnd: StyleUnit,
      PaddingHorizontal: StyleUnit,
      PaddingLeft: StyleUnit,
      PaddingRight: StyleUnit,
      PaddingStart: StyleUnit,
      PaddingTop: StyleUnit,
      PaddingVertical: StyleUnit,
      Right: StyleUnit,
      Start: StyleUnit,
      Top: StyleUnit,
      Width: StyleUnit,

      BorderBottomWidth: BorderWidth,
      BorderRightWidth: BorderWidth,
      BorderLeftWidth: BorderWidth,
      BorderTopWidth: BorderWidth
    })
  }
}

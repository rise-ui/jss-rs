use std::collections::HashMap;
use types::property_types::*;
use properties;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Appearance {
  Background(Background),
  Transforms(Transforms),
  Filters(Filters),

  BorderRadius(BorderRadius),
  BorderColor(BorderColor),
  BorderStyle(BorderStyle),
  
  Auto,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropertyValue {
  Appearance(Appearance),
  Layout(Layout),
}

pub type PropertiesApperance = PropertiesStore<Appearance>;
pub type PropertiesLayout = PropertiesStore<FlexStyle>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PropertiesStore<T>(pub HashMap<String, T>);

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Properties {
  pub appearance: PropertiesApperance,
  pub layout: PropertiesLayout,
}

/* _______________________Generic macro impl`s______________________ */
impl_union_property_conversion!(Appearance);
impl_union_property_conversion!(Layout);

/* ___________________Impl Traits & Conversions_____________________ */
impl <T>Default for PropertiesStore<T> {
  fn default() -> PropertiesStore<T> {
    PropertiesStore(HashMap::new())
  }
}
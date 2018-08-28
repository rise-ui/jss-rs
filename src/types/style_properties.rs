use std::collections::HashMap;
use types::property_types::*;
use properties;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Apperance {
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
  Apperance(Apperance),
  Layout(Layout),
}

pub type PropertiesApperance = PropertiesStore<Apperance>;
pub type PropertiesLayout = PropertiesStore<FlexStyle>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PropertiesStore<T>(pub HashMap<String, T>);

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Properties {
  pub apperance: PropertiesApperance,
  pub layout: PropertiesLayout,
}

/* _______________________Generic macro impl`s______________________ */
impl_union_property_conversion!(Apperance);
impl_union_property_conversion!(Layout);

/* ___________________Impl Traits & Conversions_____________________ */
impl <T>Default for PropertiesStore<T> {
  fn default() -> PropertiesStore<T> {
    PropertiesStore(HashMap::new())
  }
}
use std::collections::HashMap;
use types::property_types::*;
use types::SharedUnit;
use eval::Expr;

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
    Overflow(Overflow),
    FlexGrow(FlexGrow),
    Display(Display),
    Justify(Justify),
    Align(Align),
    Wrap(Wrap),

    // Shared Unit
    SharedUnit(SharedUnit),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropertyValue {
    Appearance(Appearance),
    Layout(Layout),
}

/// Link type for appearance `PropertiesStore`
pub type PropertiesApperance = PropertiesStore<Appearance>;
/// Link type for layout `PropertiesStore`
pub type PropertiesLayout = PropertiesStore<FlexStyle>;
/// Link type for calc expressions `PropertiesStore`
pub type PropertiesExpressions = PropertiesStore<Expr>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PropertiesStore<T>(pub HashMap<String, T>);

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Properties {
    /// Runtime Layout Calc Expressions (returned value for StyleUnit)
    pub expressions: PropertiesExpressions,

    /// Appearance properties store
    pub appearance: PropertiesApperance,
    /// Layout properties store with yoga `FlexStyle`
    pub layout: PropertiesLayout,
}

/* _______________________Generic macro impl`s______________________ */
impl_union_property_conversion!(Appearance);
impl_union_property_conversion!(Layout);

/* ___________________Impl Traits & Conversions_____________________ */
impl<T> Default for PropertiesStore<T> {
    fn default() -> PropertiesStore<T> {
        PropertiesStore(HashMap::new())
    }
}

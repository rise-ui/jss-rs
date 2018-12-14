use types::{Appearance, PropertyValue};
use properties::Color;
use yoga::StyleUnit;

impl_union_into_appearance!(Background);

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Background {
    Gradient(Gradient),
    Color(Color),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GradientStop {
    // Percentage offset
    pub offset: f32,
    pub color: Color,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Gradient {
    pub stops: Vec<GradientStop>,
    // By percentage
    pub from: (f32, f32),
    pub to: (f32, f32),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BoxShadow {
    pub color: Option<Color>,
    pub horizontal: StyleUnit,
    pub vertical: StyleUnit,
    pub blur: Option<StyleUnit>,
    pub spread: Option<StyleUnit>,
    pub inset: bool,
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum Visibility {
    Hidden,
    Visible,
}

impl Into<bool> for Visibility {
    fn into(self) -> bool {
        match self {
            Visibility::Visible => true,
            Visibility::Hidden => false,
        }
    }
}

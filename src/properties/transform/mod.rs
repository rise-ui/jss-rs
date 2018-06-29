mod deserialize;

use properties::{Length, Angle};
pub use self::deserialize::*;

pub type Transforms = Vec<Transform>;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Transform {
  Translate((Length, Length)),
  Rotate((Angle, Angle)),
  Skew((Angle, Angle)),
  None,
}

impl Transform {
  pub fn is_none(&self) -> bool {
    self.clone() == Transform::None
  }
}


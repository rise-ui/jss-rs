use properties::{Length, Angle};
mod convert;
mod deserialize;

pub use self::deserialize::*;
pub use self::convert::*;

pub type Transforms = Vec<Transform>;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Transform {
  Translate((Length, Length)),
  Skew((Angle, Angle)),
  Rotate(Angle),
  None,
}

impl Transform {
  pub fn is_none(&self) -> bool {
    self.clone() == Transform::None
  }
}

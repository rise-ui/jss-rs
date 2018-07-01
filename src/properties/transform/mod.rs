mod convert;
mod deserialize;

use properties::{Length, Angle};
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

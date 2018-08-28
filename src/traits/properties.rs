use types::{Apperance, PropertyValue, Layout};
use types::PropertyError;
use yoga::FlexStyle;
use std::fmt::Debug;

pub trait TStyle: Debug + PartialEq + Clone {
  fn get_apperance_style(&self, &str) -> Option<&Apperance>;
  fn get_layout_style(&self, &str) -> Option<&FlexStyle>;

  fn set_apperance_style(&mut self, &str, Option<Apperance>) -> Result<(), PropertyError>;
  fn set_layout_style(&mut self, &str, Option<Layout>) -> Result<(), PropertyError>;
  fn set_style(&mut self, &str, PropertyValue) -> Result<(), PropertyError>;
}
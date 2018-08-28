use types::{Apperance, PropertyValue, Layout};
use types::PropertyError;
use yoga::FlexStyle;
use std::fmt::Debug;

pub trait TStyle: Debug + PartialEq + Clone {
  fn get_apperance_style(&self, &str) -> Option<&Apperance>;
  fn get_layout_style(&self, &str) -> Option<&FlexStyle>;

  fn set_apperance_style<T: Into<Option<Apperance>>>(&mut self, &str, T) -> Result<(), PropertyError>;
  fn set_layout_style<T: Into<Option<Layout>>>(&mut self, &str, T) -> Result<(), PropertyError>;
  fn set_style<T: Into<Option<PropertyValue>>>(&mut self, &str, PropertyValue) -> Result<(), PropertyError>;
}
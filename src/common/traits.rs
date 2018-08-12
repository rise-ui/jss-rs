use common::{Style, ParseOptions, PropertyCase, RecursiveType};
use properties::Apperance;
use yoga::FlexStyle;
use failure::Error;

// Trait for get finalizer styles after prepared from raw style properties
pub trait PrepareStyleExt {
  fn get_prepared_styles(&self) -> (Apperance, Vec<FlexStyle>);
}

// Trait for relative parse element style
pub trait ParseStyleExt {
  // Uniform function for parse element
  fn parse_element(source: &str, options: ParseOptions) -> Result<Style, Error>;
  // Parse element on JSON
  fn parse_json_element(source: &str, recursive: RecursiveType, style: PropertyCase) -> Result<Style, Error>;
  // Parse element on YAML
  fn parse_yaml_element(source: &str, recursive: RecursiveType, style: PropertyCase) -> Result<Style, Error>;
}

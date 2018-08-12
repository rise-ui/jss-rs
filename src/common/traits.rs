use common::{Style, ParseOptions, PropertyCase, RecursiveType};
use properties::Apperance;
use yoga::FlexStyle;
use failure::Error;

pub trait PrepareStyleExt {
  fn get_prepared_styles(&self) -> (Apperance, Vec<FlexStyle>);
}

pub trait ParseStyleExt {
  fn parse_element(source: &str, options: ParseOptions) -> Result<Style, Error>;
  fn parse_json_element(source: &str, recursive: RecursiveType, style: PropertyCase) -> Result<Style, Error>;
  fn parse_yaml_element(source: &str, recursive: RecursiveType, style: PropertyCase) -> Result<Style, Error>;
}

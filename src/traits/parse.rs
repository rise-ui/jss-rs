use types::parser::{ParseOptions, PropertyCase, RecursiveType};
use types::{Style, ParseError};

// Trait for relative parse element style
pub trait TParseStyle {
  // Uniform function for parse element
  fn parse_element(source: &str, options: ParseOptions) -> Result<Style, ParseError>;
  // Parse element on JSON
  fn parse_json_element(source: &str, recursive: RecursiveType, style: PropertyCase) -> Result<Style, ParseError>;
  // Parse element on YAML
  fn parse_yaml_element(source: &str, recursive: RecursiveType, style: PropertyCase) -> Result<Style, ParseError>;
}

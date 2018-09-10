use types::{Style, ParseOptions, ParseError, Stylesheet, Case, PropertyKeyInfo};
use erased_serde::Deserializer;
use std::boxed::Box;

/// Trait for relative parse element style
pub trait TParseStyle {
    /// Uniform function for parse element
    fn parse_element(source: &str, options: ParseOptions) -> Result<Style, ParseError>;

    /// Parse element on JSON
    fn parse_json_element(source: &str, style: Case) -> Result<Style, ParseError>;

    /// Parse element on YAML
    fn parse_yaml_element(source: &str, style: Case) -> Result<Style, ParseError>;
}

pub trait TParseMiddleware {
    fn name(&self) -> String;

    fn process_value(
        &mut self,
        info: PropertyKeyInfo,
        value: Box<Deserializer>,
        properties: &mut Style,
    ) -> Result<(), ParseError>;
}

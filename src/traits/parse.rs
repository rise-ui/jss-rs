use types::{Style, ParseOptions, ParseError, Properties, Case, SourceFormat};
use erased_serde::Deserializer;

use std::fmt::{Display, Debug};
use std::error::Error;
use std::boxed::Box;

// Trait for relative parse element style
pub trait TParseStyle {
    // Uniform function for parse element
    fn parse_element(source: &str, options: ParseOptions) -> Result<Style, ParseError>;

    // Parse element on JSON
    fn parse_json_element(source: &str, style: Case) -> Result<Style, ParseError>;

    // Parse element on YAML
    fn parse_yaml_element(source: &str, style: Case) -> Result<Style, ParseError>;
}

pub trait TParseMiddleware {
    fn process_value<E>(key: String, value: Box<Deserializer>, properties: &mut Properties) -> Result<(), E>
    where
        E: Error + Display + Debug;
}

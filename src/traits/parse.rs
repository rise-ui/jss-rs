use erased_serde::Deserializer;
use std::boxed::Box;

use types::{
    StylesheetFieldInfo,
    StylesheetOptions,
    PropertyKeyInfo,
    ParseError,
    Stylesheet,
    Style,
};

/// Parsing middleware an style object
pub trait TParseStyleMiddleware {
    fn name(&self) -> String;

    /// Target method for process style property field
    fn process_value(
        &mut self,
        info: PropertyKeyInfo,
        value: Box<Deserializer>,
        properties: &mut Style,
    ) -> Result<(), ParseError>;
}

/// Parsing middleware an stylesheet object
pub trait TParseStylesheetMiddleware {
    fn name(&self) -> String;

    /// Target method for process style block or expression field
    fn process_value(
        &mut self,
        key: StylesheetFieldInfo,
        value: Box<Deserializer>,
        stylesheet: &mut Stylesheet,
        options: StylesheetOptions, 
    ) -> Result<(), ParseError>;
}
use types::{Style, ParseError, PropertyKeyInfo};
use erased_serde::Deserializer;
use std::boxed::Box;

/// Parsing middleware an style object
pub trait TParseMiddleware {
    fn name(&self) -> String;

    /// Target method for process style property field
    fn process_value(
        &mut self,
        info: PropertyKeyInfo,
        value: Box<Deserializer>,
        properties: &mut Style,
    ) -> Result<(), ParseError>;
}

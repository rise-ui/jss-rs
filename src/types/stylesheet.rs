use erased_serde::{Deserializer, deserialize};
use std::collections::HashMap;

use traits::{
    TParseStylesheetMiddleware,
    TParseStyleMiddleware
};

use types::{
    StylesheetFieldInfo,
    StylesheetOptions,
    StyleBuilder,
    ParseError,
    Variable,
    Style,
};

/// Collection of styles
#[derive(Clone, Debug, Default)]
pub struct Stylesheet {
    // Styles collection
    pub styles: HashMap<String, Style>,
    // Variables for preset before configurations
    pub variables: HashMap<String, Variable>,
}

#[derive(Debug, Clone)]
pub struct ParseStylesheetMiddleware {}

impl TParseStylesheetMiddleware for ParseStylesheetMiddleware {
    fn name(&self) -> String {
        "default".to_string()
    }

    /// Target method for process style block or expression field
    fn process_value(
        &mut self,
        key: StylesheetFieldInfo,
        value: Box<Deserializer>,
        stylesheet: &mut Stylesheet,
        options: StylesheetOptions,
    ) -> Result<(), ParseError> {
        let mut style = StyleBuilder::default()
            .source_type(options.source_type)
            .case(options.case);

        // @todo: need solve problem with usage style parse middlewares from global stylesheet
        let style = style.parse_from_de(value)?;
        stylesheet.styles.insert(key.name, style);

        Ok(())
    }
}

impl Stylesheet {
    pub fn take(&self, name: String) -> Option<Style> {
        self.styles.get(&name).and_then(|style| {
            Some(style.clone())
        })
    }
}
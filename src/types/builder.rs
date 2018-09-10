/// Style Builder
use std::collections::HashMap;
use traits::TParseMiddleware;
use utils::generic_erase;
use std::boxed::Box;

use types::{
    DefaultParseMiddleware,
    PropertyParseType,
    PropertyKeyInfo,
    SourceFormat,
    ParseError,
    Style,
    Case,
};

pub struct StyleBuilder<'a> {
    /// Custom started from "@"
    custom_middlewares: HashMap<String, Box<TParseMiddleware>>,
    //expression_middleware: Box<TParseMiddleware>,
    default_middleware: Box<TParseMiddleware>,

    source_type: SourceFormat,
    source: &'a str,
    case: Case,
}

impl<'a> Default for StyleBuilder<'a> {
    fn default() -> StyleBuilder<'a> {
        StyleBuilder {
            default_middleware: Box::new(DefaultParseMiddleware {}),
            custom_middlewares: HashMap::default(),
            source_type: SourceFormat::Json,
            case: Case::Ignore,
            source: "{}",
        }
    }
}

impl<'a> StyleBuilder<'a> {
    pub fn middleware(mut self, middleware: Box<TParseMiddleware>) -> Self {
        self.custom_middlewares.insert(middleware.name(), middleware);
        self
    }

    pub fn case(mut self, case: Case) -> Self {
        self.case = case;
        self
    }

    pub fn source_type(mut self, source_type: SourceFormat) -> Self {
        self.source_type = source_type;
        self
    }

    pub fn source(mut self, source: &'a str) -> Self {
        self.source = source;
        self
    }

    #[rustfmt::skip]
    pub fn parse(mut self) -> Result<Style, ParseError> {
        let mut style = Style::default();
        make_initial_style_states!(style, [default, active, hover]);
        let erased = generic_erase(self.source, self.source_type)?;

        for (key, value) in erased {
            match key.key_type {
                PropertyParseType::Default => {
                    self.default_middleware.process_value(key, value, &mut style)?;
                }

                PropertyParseType::Expression => {
                    //self.expression_middleware.process_value(key, value, &mut style)?;
                }

                PropertyParseType::Custom => {
                    self.custom_middlewares.get_mut(&key.name)
                        .ok_or(ParseError::MissingMiddleware { name: key.name.clone() })
                        .and_then(|middleware| middleware.process_value(key, value, &mut style))?;
                }
            }
        }

        Ok(style)
    }
}

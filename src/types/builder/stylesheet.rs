/// Stylesheet Builder
use utils::{generic_erase, StylePropertyField};
use erased_serde::{Deserializer, deserialize};
use hashbrown::HashMap;
use std::boxed::Box;

use traits::{
    TParseStylesheetMiddleware,
    TParseStyleMiddleware,
};

use types::{
    ParseStylesheetMiddleware,
    StylesheetFieldInfo,
    FieldParseType,
    SourceFormat,
    Stylesheet,
    ParseError,
    Case,
};

pub struct StylesheetBuilder {
    style_custom_middlewares: HashMap<String, Box<TParseStyleMiddleware>>,
    custom_middlewares: HashMap<String, Box<TParseStylesheetMiddleware>>,
    default_middleware: Box<TParseStylesheetMiddleware>,

    source_type: SourceFormat,
    case: Case,
}

#[derive(Clone, Debug, Copy)]
pub struct StylesheetOptions {
    pub source_type: SourceFormat,
    pub case: Case,
}

impl Default for StylesheetBuilder {
    fn default() -> StylesheetBuilder {
        StylesheetBuilder {
            default_middleware: Box::new(ParseStylesheetMiddleware {}),
            style_custom_middlewares: HashMap::default(),
            custom_middlewares: HashMap::default(),
            source_type: SourceFormat::Json,
            case: Case::Ignore,
        }
    }
}

impl StylesheetBuilder {
    pub fn middleware(mut self, middleware: Box<TParseStylesheetMiddleware>) -> Self {
        self.custom_middlewares.insert(middleware.name(), middleware);
        self
    }

    pub fn style_middleware(mut self, middleware: Box<TParseStyleMiddleware>) -> Self {
        self.style_custom_middlewares.insert(middleware.name(), middleware);
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

    pub fn parse_from_str(&mut self, source: &str) -> Result<Stylesheet, ParseError> {
        let erased = generic_erase(source, self.source_type)?;
        let mut stylesheet = Stylesheet::default();

        let options = StylesheetOptions {
            source_type: self.source_type,
            case: self.case,
        };

        for (key, value) in erased {
            let key = StylesheetFieldInfo::new(key.as_str());

            match key.key_type {
                FieldParseType::Variable => {
                    // @todo: handle this
                }

                FieldParseType::Custom => {
                    self.custom_middlewares
                        .get_mut(&key.name)
                        .ok_or(ParseError::MissingMiddleware {
                            name: key.name.clone(),
                        })
                        .and_then(|middleware| middleware.process_value(key, value, &mut stylesheet, options))?;
                }

                FieldParseType::Default => {
                    self.default_middleware.process_value(key, value, &mut stylesheet, options)?;
                }
            }
        }

        Ok(stylesheet)
    }
}

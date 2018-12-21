/// Style Builder
use traits::{TParseStyleMiddleware, TStyleStates};
use utils::{generic_erase, StylePropertyField};
use erased_serde::{Deserializer, deserialize};
use hashbrown::HashMap;
use serde_json::Value;
use std::boxed::Box;

use types::{
    ParseStyleMiddleware,
    PropertyParseType,
    PropertyKeyInfo,
    PropertyError,
    SourceFormat,
    ParseError,
    Style,
    Case,
};

pub struct StyleBuilder {
    /// Custom started from "@"
    custom_middlewares: HashMap<String, Box<TParseStyleMiddleware>>,

    //expression_middleware: Box<TParseStyleMiddleware>,
    default_middleware: Box<TParseStyleMiddleware>,

    source_type: SourceFormat,
    case: Case,
}

impl Default for StyleBuilder {
    fn default() -> StyleBuilder {
        StyleBuilder {
            default_middleware: Box::new(ParseStyleMiddleware {}),
            custom_middlewares: HashMap::default(),
            source_type: SourceFormat::Json,
            case: Case::Ignore,
        }
    }
}

fn get_property_key(key: String) -> Result<PropertyKeyInfo, ParseError> {
    PropertyKeyInfo::new(key.as_str()).map_err(|error| ParseError::PropertyError {
        error,
    })
}

fn valid_case(info: &PropertyKeyInfo, case: Case) -> Result<(), ParseError> {
    if case == Case::Ignore || case == info.case {
        Ok(())
    } else {
        Err(ParseError::PropertyError {
            error: PropertyError::InvalidKeyCase {
                key: info.name.clone(),
                case,
            },
        })
    }
}

impl StyleBuilder {
    pub fn middleware(mut self, middleware: Box<TParseStyleMiddleware>) -> Self {
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

    #[rustfmt::skip]
    pub fn parse_from_str(&mut self, source: &str) -> Result<Style, ParseError> {
        let erased = generic_erase(source, self.source_type).and_then(|properties| {
            let mut result = vec![];

            for (key, value) in properties {
                let key_info = get_property_key(key)?;
                valid_case(&key_info, self.case)?;
                result.push((key_info, value));
            }

            Ok(result)
        })?;

        self.parse_from_iter(erased.into_iter())
    }

    pub fn parse_from_de(&mut self, de: Box<Deserializer>) -> Result<Style, ParseError> {
        let map: HashMap<String, Value> = deserialize(Box::leak(de)).map_err(|error| ParseError::InvalidSource {
            source_type: self.source_type,
            error: error.into(),
        })?;

        let mut erased: Vec<StylePropertyField> = vec![];
        for (key, value) in map {
            let key_info = get_property_key(key)?;
            valid_case(&key_info, self.case)?;

            let value = Box::new(Deserializer::erase(value));
            erased.push((key_info, value));
        }

        self.parse_from_iter(erased.into_iter())
    }

    #[rustfmt::skip]
    pub fn parse_from_iter<'a, I>(&mut self, iter: I) -> Result<Style, ParseError>
    where
        I: Iterator<Item = StylePropertyField<'a>>,
    {
        let mut style = Style::default();
        make_initial_style_states!(style, [default, active, hover]);
        style.enable_states(vec![ "default".to_string() ]);

        for (key, value) in iter {
            match key.key_type {
                PropertyParseType::Default => {
                    self.default_middleware.process_value(key, value, &mut style)?;
                }

                PropertyParseType::Custom => {
                    self.custom_middlewares.get_mut(&key.name)
                        .ok_or(ParseError::MissingMiddleware { name: key.name.clone() })
                        .and_then(|middleware| middleware.process_value(key, value, &mut style))?;
                }

                _ => {}
            }
        }

        Ok(style)
    }
}

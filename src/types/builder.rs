/// Style Builder
use types::{Style, Properties, ParseError, Case, SourceFormat};
use traits::TParseMiddleware;

use std::collections::HashMap;
use std::any::{Any, TypeId};

use erased_serde::{self, Deserializer};
use serde_json;
use serde_yaml;

// let data: DeStruct = erased_serde::deserialize(Box::leak(formats)).unwrap();

fn generic_erase(source: &str, source_type: SourceFormat) -> Result<Box<Deserializer>, ParseError> {
    use self::SourceFormat::*;
    use self::ParseError::*;

    match source_type {
        Yaml => serde_yaml::from_str::<serde_yaml::Value>(source)
            .map_err(|error| InvalidSource {
                source_type,
                error: error.into(),
            }).and_then(|yaml| {
                let erased: Box<Deserializer> = Box::new(Deserializer::erase(yaml));
                Ok(erased)
            }),
        Json => serde_json::from_str::<serde_json::Value>(source)
            .map_err(|error| InvalidSource {
                source_type,
                error: error.into(),
            }).and_then(|json| {
                let erased: Box<Deserializer> = Box::new(Deserializer::erase(json));
                Ok(erased)
            }),
    }
}

pub struct StyleBuilder<'a, P>
where
    P: TParseMiddleware,
{
    /// Custom started from "@"
    custom_middlewares: HashMap<String, P>,
    default_middleware: P,

    source_type: SourceFormat,
    source: &'a str,
    case: Case,
}

impl<'a, P> StyleBuilder<'a, P>
where
    P: TParseMiddleware,
{
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

    pub fn parse(mut self) -> Result<(), ParseError> {
        let mut properties_default = Properties::default();
        let mut style = Style::default();

        let erased = generic_erase(self.source, self.source_type)?;
        let map: HashMap<String, &[u8]> = erased_serde::deserialize(Box::leak(erased)).unwrap();

        Ok(())
    }
}

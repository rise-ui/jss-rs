use types::{PropertyKeyInfo, ParseError, SourceFormat};
use erased_serde::Deserializer;
use std::collections::HashMap;
use serde_json;
use serde_yaml;

/// Parser that return erased deserialize object, by different source type
pub fn generic_erase(
    source: &str,
    source_type: SourceFormat,
) -> Result<Vec<(PropertyKeyInfo, Box<Deserializer>)>, ParseError> {
    use self::SourceFormat::*;
    use self::ParseError::*;

    match source_type {
        Yaml => serde_yaml::from_str::<HashMap<String, serde_yaml::Value>>(source)
            .map_err(|error| InvalidSource {
                source_type,
                error: error.into(),
            }).and_then(|yaml| {
                let mut erased = vec![];
                for (key, value) in yaml.into_iter() {
                    let value: Box<Deserializer> = Box::new(Deserializer::erase(value));
                    let key_info = PropertyKeyInfo::new(key.as_str()).map_err(|error| PropertyError {
                        error,
                    })?;

                    erased.push((key_info, value));
                }

                Ok(erased)
            }),
        Json => serde_json::from_str::<HashMap<String, serde_json::Value>>(source)
            .map_err(|error| InvalidSource {
                source_type,
                error: error.into(),
            }).and_then(|json| {
                let mut erased = vec![];
                for (key, value) in json.into_iter() {
                    let value: Box<Deserializer> = Box::new(Deserializer::erase(value));
                    let key_info = PropertyKeyInfo::new(key.as_str()).map_err(|error| PropertyError {
                        error,
                    })?;

                    erased.push((key_info, value));
                }

                Ok(erased)
            }),
    }
}

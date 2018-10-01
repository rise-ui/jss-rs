use types::{PropertyKeyInfo, ParseError, SourceFormat};
use erased_serde::Deserializer;
use std::collections::HashMap;
use serde_json;
use serde_yaml;

pub type StylePropertyField<'a> = (PropertyKeyInfo, Box<Deserializer<'a>>);

pub fn map_property_key_info(field: (String, Box<Deserializer>)) -> Result<StylePropertyField, ParseError> {
    use self::ParseError::*;

    let key_info = PropertyKeyInfo::new(field.0.as_str()).map_err(|error| PropertyError {
        error,
    })?;

    Ok((key_info, field.1))
}

/// Parser that return erased deserialize object, by different source type
pub fn generic_erase(source: &str, source_type: SourceFormat) -> Result<Vec<(String, Box<Deserializer>)>, ParseError> {
    use self::SourceFormat::*;
    use self::ParseError::*;

    match source_type {
        Yaml => serde_yaml::from_str::<HashMap<String, serde_yaml::Value>>(source)
            .map_err(|error| InvalidSource {
                source_type,
                error: error.into(),
            })
            .and_then(|yaml| {
                let mut erased = vec![];
                for (key, value) in yaml.into_iter() {
                    let value: Box<Deserializer> = Box::new(Deserializer::erase(value));
                    erased.push((key, value));
                }

                Ok(erased)
            }),
        Json => serde_json::from_str::<HashMap<String, serde_json::Value>>(source)
            .map_err(|error| InvalidSource {
                source_type,
                error: error.into(),
            })
            .and_then(|json| {
                let mut erased = vec![];
                for (key, value) in json.into_iter() {
                    let value: Box<Deserializer> = Box::new(Deserializer::erase(value));
                    erased.push((key, value));
                }

                Ok(erased)
            }),
    }
}

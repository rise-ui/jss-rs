use types::parser::{Case, SourceFormat};

use failure::Error;
use serde_json;
use serde_yaml;

#[derive(Debug, Fail)]
pub enum PropertyError {
    #[fail(display = "invalid property type for {} - expected {}", property, expected)]
    InvalidType {
        property: String,
        expected: String,
    },
    #[fail(display = "invalid property key {}", key)]
    InvalidKey {
        key: String,
    },
}

#[derive(Debug, Fail)]
pub enum ParseError {
    #[fail(display = "invalid property '{}' case, need: {:?}", key, case)]
    InvalidKeyCase {
        case: Case,
        key: String,
    },

    #[fail(display = "invalid {:?}: {:?}", source_type, error)]
    InvalidSource {
        source_type: SourceFormat,
        error: Error,
    },

    #[fail(display = "invalid {:?} value for property '{}': {:?}", source_type, property, error)]
    InvalidValue {
        source_type: SourceFormat,
        property: String,
        error: Error,
    },

    #[fail(display = "error with set property '{}': {:?}", property, error)]
    ErrorPasteProperty {
        error: PropertyError,
        property: String,
    },

    #[fail(display = "error: '{}'", error)]
    CustomError {
        error: String,
    },
}

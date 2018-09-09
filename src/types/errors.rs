use types::parser::PropertyCase;
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
        case: PropertyCase,
        key: String,
    },

    #[fail(display = "invalid JSON: {:?}", error)]
    InvalidJSON {
        error: serde_json::Error,
    },

    #[fail(display = "invalid JSON value for property '{}': {:?}", property, error)]
    InvalidJSONValue {
        error: serde_json::Error,
        property: String,
    },

    #[fail(display = "invalid YAML: {:?}", error)]
    InvalidYAML {
        error: serde_yaml::Error,
    },

    #[fail(display = "invalid YAML value for property '{}': {:?}", property, error)]
    InvalidYAMLValue {
        error: serde_yaml::Error,
        property: String,
    },

    #[fail(display = "error with set property '{}': {:?}", property, error)]
    ErrorPasteProperty {
        error: PropertyError,
        property: String,
    },
}

use types::parser::{Case, SourceFormat};
use failure::Error;
use eval;

#[derive(Debug, Fail, PartialEq)]
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

    #[fail(display = "invalid property '{}' case, need: {:?}", key, case)]
    InvalidKeyCase {
        case: Case,
        key: String,
    },

    #[fail(display = "invalid expression '{}': {:?}", key, error)]
    InvalidExpression {
        error: eval::Error,
        key: String,
    },

    #[fail(display = "unknown '{}' key (or not associated with `FlexStyle` properties)", key)]
    SharedUnitConvert {
        key: String,
    },
}

#[derive(Debug, Fail)]
pub enum ParseError {
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

    #[fail(display = "error with deserialize `{}` into {}: {:?}", key, target, error)]
    DeserializeError {
        target: String,
        error: Error,
        key: String,
    },

    #[fail(display = "missing custom middleware \"{}\"", name)]
    MissingMiddleware {
        name: String,
    },

    #[fail(display = "{:?}", error)]
    PropertyError {
        error: PropertyError,
    },

    #[fail(display = "error: '{}'", error)]
    CustomError {
        error: String,
    },

    #[fail(display = "missing state key `{}` in style", name)]
    StateMissing {
        name: String,
    },
}

#[derive(Debug, Fail)]
pub enum ProcessingError {
    #[fail(display = "failed run expression for `{}` in style: {:#?}", property, error)]
    ExecFailed {
        error: eval::Error,
        property: String,
    },

    #[fail(display = "invalid type `{}` - expected {}", property, expected)]
    InvalidType {
        property: String,
        expected: String,
    },
}

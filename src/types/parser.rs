use types::{Style, ParseError};

/// What format of properties keys to use and check when parsing
/// Current allowed: snake_case, camelCase, kebab-case or ignore case
/// Default: camelCase
#[derive(Debug, Clone, Copy)]
pub enum Case {
    Ignore,
    Snake,
    Kebab,
    Camel,
}

impl Default for Case {
    fn default() -> Self {
        Case::Camel
    }
}

/// From what data format to parse, currently available: JSON, YAML
/// Default: JSON
#[derive(Debug, Clone, Copy)]
pub enum SourceFormat {
    Json,
    Yaml,
}

impl Default for SourceFormat {
    fn default() -> Self {
        SourceFormat::Json
    }
}

/// Options for parse style from uniform functions
#[derive(Default, Debug, Clone, Copy)]
pub struct ParseOptions {
    pub from: SourceFormat,
    pub style: Case,
}

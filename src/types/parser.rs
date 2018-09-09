use types::{Style, ParseError};

/// Result with styles and parse warnings
#[derive(Debug)]
pub struct ParseResult {
    pub warnings: Vec<ParseError>,
    pub style: Style,
}

/// What format of properties keys to use and check when parsing
/// Current allowed: snake_case, camelCase, kebab-case or ignore case
/// Default: camelCase
#[derive(Debug, Clone, Copy)]
pub enum PropertyCase {
    Ignore,
    Snake,
    Kebab,
    Camel,
}

impl Default for PropertyCase {
    fn default() -> Self {
        PropertyCase::Camel
    }
}

/// From what data format to parse, currently available: JSON, YAML
/// Default: JSON
#[derive(Debug, Clone, Copy)]
pub enum ParseTarget {
    Json,
    Yaml,
}

impl Default for ParseTarget {
    fn default() -> Self {
        ParseTarget::Json
    }
}

/// One level recursion inside with: @media queries, status presudo classes: :hover, :active.
/// Default: Basic
#[derive(Debug, Clone, Copy)]
pub enum RecursiveType {
    Basic,
    Never,
}

impl Default for RecursiveType {
    fn default() -> Self {
        RecursiveType::Basic
    }
}

/// Options for parse style from uniform functions
#[derive(Default, Debug, Clone, Copy)]
pub struct ParseOptions {
    pub recursive: RecursiveType,
    pub style: PropertyCase,
    pub from: ParseTarget,
}

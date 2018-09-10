use inflector::Inflector;

/// What format of properties keys to use and check when parsing
/// Current allowed: snake_case, camelCase, kebab-case or ignore case
/// Default: camelCase
#[derive(Debug, Clone, Copy, PartialEq)]
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

impl Case {
    pub fn is_valid(&self, key: &String) -> bool {
        match &self {
            Case::Snake => key.is_snake_case(),
            Case::Kebab => key.is_kebab_case(),
            Case::Camel => key.is_camel_case(),
            Case::Ignore => true,
        }
    }

    pub fn new(source: &str) -> Case {
        if source.is_snake_case() {
            Case::Snake
        } else if source.is_camel_case() {
            Case::Camel
        } else if source.is_kebab_case() {
            Case::Kebab
        } else {
            Case::Ignore
        }
    }
}

/// From what data format to parse, currently available: JSON, YAML
/// Default: JSON
#[derive(Debug, Clone, Copy, PartialEq)]
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
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct ParseOptions {
    pub from: SourceFormat,
    pub style: Case,
}

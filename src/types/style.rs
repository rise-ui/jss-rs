use types::parser::{ParseOptions, PropertyCase, RecursiveType};
use types::{Properties, ParseError, PropertyValue};
use traits::{TParseStyle, TStyle};
use std::collections::HashMap;
use inflector::Inflector;
use yoga::Layout;

/// Context with other needed info - for parse and prepares,
/// aka dimensions screen, element measures, variables, and other.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Context {
    // Variables for preset before configurations
    variables: HashMap<String, String>,
    // Layout props this container
    layout: Option<Layout>,
}

/// Style element, with all element status, and context`s,
/// with implementations of traits for parse unions of one element
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Style {
    // States of properties as :hover, :active, etc..
    pub states: HashMap<String, Properties>,

    // Context
    pub context: Context,
}

/* _______________________________________________________________________ */

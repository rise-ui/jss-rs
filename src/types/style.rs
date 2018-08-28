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

// Trait for relative parse element style
impl TParseStyle for Style {
  // Uniform function for parse element
  fn parse_element(source: &str, options: ParseOptions) -> Result<Style, ParseError> {
    unimplemented!()
  }
  
  // Parse element on JSON
  fn parse_json_element(source: &str, recursive: RecursiveType, case_style: PropertyCase) -> Result<Style, ParseError> {
    use serde_json::{Value, from_value, from_str};
    use self::ParseError::*;
    
    let properties: HashMap<String, Value> = from_str(source).map_err(|error| InvalidJSON { error })?;
    let mut properties_default = Properties::default();
    let mut style = Style::default();

    for (key, value) in properties {
      let action_prefix = &key[..1];
      
      match action_prefix {
        // States like :hover, :active
        ":" => {}
        // Nested insides
        "$" => {},
        // actions handlers
        "@" => {},
        // Otherwise standard properties
        _ => {
          let is_valid_case = match case_style {
            PropertyCase::Snake => key.is_snake_case(),
            PropertyCase::Kebab => key.is_kebab_case(),
            PropertyCase::Camel => key.is_camel_case(),
            PropertyCase::Ignore => true
          };

          if is_valid_case {
            let property_key = {
              let mut src = key.to_camel_case();
              let f = &src[0..1].to_uppercase();

              src.replace_range(..1, f);
              src
            };
            
            let property: PropertyValue = from_value(value)
              .map_err(|error| InvalidJSONValue { error, property: key.clone() })?;


            println!("'{}': {:#?}", property_key, property);

            // @TODO: add support for uniform styles like: border-radius, border-style, etc..
            properties_default.set_style(property_key.as_str(), property)
              .map_err(|error| ErrorPasteProperty { error, property: key.clone() })?;
          }          
        }
      }
    }

    // Insert default properties to default state
    style.states.insert("default".to_string(), properties_default);

    Ok(style)
  }

  // Parse element on YAML
  fn parse_yaml_element(source: &str, recursive: RecursiveType, style: PropertyCase) -> Result<Style, ParseError> {
    unimplemented!()
  }
}

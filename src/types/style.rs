use yoga::{FlexStyle, StyleUnit, Layout as Dimensions};
use ordered_float::OrderedFloat;
use std::collections::HashMap;
use serde_json::Value;

use types::{
    PropertiesExpressions,
    PropertiesLayout,
    ProcessingError,
    Properties,
    Variable,
    pair_to_flex
};

use traits::{
    TStyleContext,
    TStyleCollect,
    TStyleStates,
};

/// Context with other needed info - for parse and prepares,
/// aka dimensions screen, element measures, variables, and other.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Context {
    // Variables for preset before configurations
    pub variables: HashMap<String, Variable>,
    // Layout props this container
    pub dimensions: Option<Dimensions>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CollectedStyle {
    layout: Vec<FlexStyle>,
}

/// Style element, with all element status, and context`s,
/// with implementations of traits for parse unions of one element
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Style {
    // States of properties as :hover, :active, etc..
    pub states: HashMap<String, Properties>,

    // Context
    pub context: Context,

    // Collected styles, after compilation by enables states
    pub collected: CollectedStyle,

    // Enabled states of current style
    pub enabled_states: Vec<String>,
}

/* _______________________________________________________________________ */
impl TStyleContext for Context {
    fn set_dimensions(&mut self, dimensions: Option<Dimensions>) {
        self.dimensions = dimensions;
    }

    fn set_variable(&mut self, name: String, value: Variable) {
        self.variables.insert(name, value).is_some();
    }

    fn set_variables<T>(&mut self, variables: T)
    where
        T: IntoIterator<Item = (String, Variable)>,
    {
        for (name, value) in variables {
            self.variables.insert(name, value).is_some();
        }
    }
}

impl TStyleStates for Style {
    // @todo: adding check for state exists
    fn set_states(&mut self, states: Vec<String>) {
        self.enabled_states = states;
    }
}

impl TStyleCollect for Style {
    fn collect_layout_style(&self) -> (Vec<FlexStyle>, Vec<ProcessingError>) {
        use self::ProcessingError::*;

        let mut layout_styles = vec![];
        let mut eval_errors = vec![];

        let mut expressions = PropertiesExpressions::default();
        let mut layout = PropertiesLayout::default();

        for state in self.enabled_states.iter() {
            let state_expressions = self.states[state].expressions.0.clone();
            let state_layout = self.states[state].layout.0.clone();

            expressions.0.extend(state_expressions);
            layout.0.extend(state_layout);
        }

        // Run expressions
        for (property, expr) in expressions.0 {
            let mut expression = expr.clone();

            // Set variables into runtime expression
            for (name, value) in self.context.variables.iter() {
                match value {
                    Variable::Number(num) => {
                        expression = expression.value(name.clone(), num);
                    }
                    Variable::Map(map) => {
                        expression = expression.value(name.clone(), map);
                    }
                }
            }

            match expression.exec() {
                Err(error) => eval_errors.push(ExecFailed {
                    property,
                    error,
                }),

                Ok(value) => {
                    let number_msg = "expected float or integer";

                    let make_type_error = |msg: &str| InvalidType {
                        expected: msg.to_string(),
                        property: property.clone(),
                    };

                    extract!(Value::Number(_), value)
                        .ok_or(make_type_error(number_msg))
                        .and_then(|n| n.as_f64().ok_or(make_type_error(number_msg)))
                        .and_then(|number| {
                            let number: OrderedFloat<f32> = (number as f32).into();

                            pair_to_flex(property.clone(), StyleUnit::Point(number))
                                .map_err(|_| make_type_error("valid unit by key"))
                        }).and_then(|flex_style| {
                            layout_styles.push(flex_style);
                            Ok(())
                        }).is_ok();
                }
            }
        }

        // Setter static property
        for (_property, value) in layout.0 {
            layout_styles.push(value);
        }

        (layout_styles, eval_errors)
    }
}

use yoga::{FlexStyle, StyleUnit, Layout as Dimension};
use types::PropertiesAppearance;
use ordered_float::OrderedFloat;
use std::collections::HashMap;
use convert::WebrenderStyles;
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

/// Style dimensions context
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DimensionsContext {
    pub current: Option<Dimension>,
    pub parent: Option<Dimension>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DimensionType {
    Current,
    Parent,
}

/// Context with other needed info - for parse and prepares,
/// aka dimensions screen, element measures, variables, and other.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Context {
    // Variables for preset before configurations
    pub variables: HashMap<String, Variable>,
    // Layout props this container
    pub dimensions: DimensionsContext,
}

/// Store for save calculated cached styles with converted version
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Computed {
    pub appearance: Option<PropertiesAppearance>,
    pub layout: Option<Vec<FlexStyle>>,
}

/// Style element, with all element status, and context`s,
/// with implementations of traits for parse unions of one element
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Style {
    // Finalize compuited values
    pub computed: Computed,

    // States of properties as :hover, :active, etc..
    pub states: HashMap<String, Properties>,

    // Context
    pub context: Context,

    // Enabled states of current style
    pub enabled_states: Vec<String>,
}

/// Default context representation for Parsing Trait
/// Trait `TParseStyleMiddleware` implemented in jss_derive as proc-macro
#[derive(Debug, Clone)]
pub struct ParseStyleMiddleware {}

/* _______________________________________________________________________ */
fn set_dimension_variable(context: &mut Context, name: String, dimension: &Option<Dimension>) {
    extract!(Some(_), dimension)
        .and_then(|dimension| {
            let self_variable = hashmap! {
                "bottom" => dimension.bottom(),
                "right" => dimension.right(),
                "left" => dimension.left(),
                "top" => dimension.top(),
                "height" => dimension.height(),
                "width" => dimension.width(),
            }
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect::<HashMap<String, f32>>();

            context.set_variable(name, Variable::Map(self_variable));
            Some(())
        })
        .is_some();
}

impl TStyleContext for Context {
    fn set_dimension(&mut self, entry_type: DimensionType, dimension: Option<Dimension>) {
        match entry_type {
            DimensionType::Current => {
                set_dimension_variable(self, "$self".to_string(), &dimension);
                self.dimensions.current = dimension;
            }

            DimensionType::Parent => {
                set_dimension_variable(self, "$parent".to_string(), &dimension);
                self.dimensions.parent = dimension;
            }
        }
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
    fn enable_states(&mut self, states: Vec<String>) {
        self.enabled_states = states;
    }
}

impl TStyleCollect for Style {
    fn calculate_layout(&mut self) -> Vec<ProcessingError> {
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
                        })
                        .and_then(|flex_style| {
                            layout_styles.push(flex_style);
                            Ok(())
                        })
                        .is_ok();
                }
            }
        }

        // Setter static property
        for (_property, value) in layout.0 {
            layout_styles.push(value);
        }

        self.computed.layout = Some(layout_styles);
        eval_errors
    }

    fn calculate_appearance(&mut self) -> Vec<ProcessingError> {
        // use self::ProcessingError::*;

        let mut properties = PropertiesAppearance::default();
        let eval_errors = vec![];

        // @TODO: Adding support variables & expr for appearance styles
        for state in self.enabled_states.iter() {
            let appearance = self.states[state].appearance.0.clone();
            properties.0.extend(appearance);
        }

        self.computed.appearance = Some(properties);
        eval_errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use types::{Properties, Style, DimensionType};
    use types::values::{CalcExpr, Dimensions};
    use properties::{Background, Color};
    use traits::*;

    use test::Bencher;
    use eval::Expr;

    #[bench]
    fn bench_style_with_calc(b: &mut Bencher) {
        b.iter(|| {
            let mut properties = Properties::default();
            let mut style = Style::default();

            let current = Dimensions::new(10., 10., 10., 10., 480., 480.);
            let parent = Dimensions::new(0., 0., 0., 0., 500., 500.);

            style.context.set_dimension(DimensionType::Current, Some(current));
            style.context.set_dimension(DimensionType::Parent, Some(parent));

            properties.set_style("background", Background::Color(Color::transparent())).is_ok();
            properties.set_style("height", CalcExpr(Expr::new("$parent.width + 10"))).is_ok();

            style.states.insert("default".to_string(), properties);
            style.enable_states(vec!["default".to_string()]);

            style.calculate_layout();
            style.calculate_appearance();
        });
    }
}
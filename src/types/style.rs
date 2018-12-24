use yoga::{FlexStyle, StyleUnit, Layout as Dimension};
use types::PropertiesAppearance;
use ordered_float::OrderedFloat;
use hashbrown::HashMap;
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
    pub appearance: PropertiesAppearance,
    pub layout: Vec<FlexStyle>,
}

/// All default states for style
#[derive(Debug, Ord, PartialOrd, Clone, Eq, Hash, PartialEq)]
pub enum StateKey {
    Default = 0,
    Active = 1,
    Hover = 2,
}

/// Style element, with all element status, and context`s,
/// with implementations of traits for parse unions of one element
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Style {
    // Finalize compuited values
    pub computed: Computed,

    // States of properties as :hover, :active, etc..
    pub states: HashMap<StateKey, Properties>,

    // Context
    pub context: Context,

    // Enabled states of current style
    pub enabled_states: Vec<StateKey>,
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
    fn enable_state(&mut self, state: StateKey) {
        if let Err(index) = self.enabled_states.binary_search(&state) {
            self.enabled_states.insert(index, state);
        }
    }
}

fn make_type_error(key: String, msg: &str) -> ProcessingError {
    ProcessingError::InvalidType {
        expected: msg.to_string(),
        property: key,
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
        for (property, mut expression) in expressions.0 {
            let formatted_key = format!("{:?}", property);

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
                    property: formatted_key,
                    error,
                }),

                Ok(value) => {
                    extract!(Value::Number(_), value)
                        .ok_or(make_type_error(formatted_key.clone(), "expected float or integer"))
                        .and_then(|n| n.as_f64().ok_or(make_type_error(formatted_key.clone(), "expected float or integer")))
                        .and_then(|number| {
                            let number: OrderedFloat<f32> = (number as f32).into();
                            Ok(pair_to_flex(property.clone(), StyleUnit::Point(number)))
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

        self.computed.layout = layout_styles;
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

        self.computed.appearance = properties;
        eval_errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use types::{Properties, PropertyKey, LayoutKey, AppearanceKey, Style, DimensionType, StateKey};
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

            properties.set_style(
                PropertyKey::Appearance(AppearanceKey::Background),
                Background::Color(Color::transparent())
            ).is_ok();

            properties.set_style(
                PropertyKey::Layout(LayoutKey::Height),
                CalcExpr(Expr::new("$parent.width + 10"))
            ).is_ok();
            
            style.states.insert(StateKey::Default, properties);
            style.enable_state(StateKey::Default);

            style.calculate_layout();
            style.calculate_appearance();
        });
    }
}
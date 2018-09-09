use utils::extract_args_by_type;

use properties::parse::{
  TransformFunction,
  GradientStopRepr,
  GradientFunction,
  FilterFunction,
  LengthRepr,
  AngleRepr,
  UnitRepr,
};

use properties::{
  SharedUnit,
  Transform,
  Filter,
};

/* Into main types */
impl<'a> From<FilterFunction<'a>> for Filter {
    fn from(value: FilterFunction) -> Filter {
        match value.name {
            "brightness" => Filter::Brightness(value.value),
            "grayscale" => Filter::Grayscale(value.value),
            "hueRotate" => Filter::HueRotate(value.value),
            "saturate" => Filter::Saturate(value.value),
            "contrast" => Filter::Contrast(value.value),
            "invert" => Filter::Invert(value.value),
            "sepia" => Filter::Sepia(value.value),
            "blur" => Filter::Blur(value.value),
            _ => Filter::None,
        }
    }
}

impl<'a, 'b, 'c> From<TransformFunction<'a, 'b, 'c>> for Transform {
    fn from(value: TransformFunction) -> Transform {
        let units: Vec<SharedUnit> = value.args.iter().cloned().map(Into::into).collect();
        extract_args_by_type(value.name, &units)
    }
}

/* Into serealized string */

impl<'a, 'b, 'c> From<TransformFunction<'a, 'b, 'c>> for String {
    fn from(value: TransformFunction) -> String {
        let args = value.args.into_iter().map(|s| String::from(s)).collect::<Vec<_>>().join(",");
        format!("{}({})", value.name, args)
    }
}

impl From<GradientStopRepr> for String {
    fn from(value: GradientStopRepr) -> String {
        format!("{} {}%", value.color.to_string(), value.offset as u32)
    }
}

impl<'a, 'b> From<GradientFunction<'a, 'b>> for String {
    fn from(value: GradientFunction) -> String {
        let stops = value.stops.into_iter().map(|s| String::from(s)).collect::<Vec<_>>().join(",");
        format!("{}, {}", String::from(value.angle), stops)
    }
}

impl<'a> From<FilterFunction<'a>> for String {
    fn from(value: FilterFunction) -> String {
        format!("{}({})", value.name, value.value as u32)
    }
}

impl<'a, 'b> From<LengthRepr<'a, 'b>> for String {
    fn from(value: LengthRepr) -> String {
        format!("{}{}", value.value, value.unit)
    }
}

impl<'a, 'b> From<AngleRepr<'a, 'b>> for String {
    fn from(value: AngleRepr) -> String {
        format!("{}{}", value.value, value.angle)
    }
}

impl<'a, 'b> From<UnitRepr<'a, 'b>> for String {
    fn from(value: UnitRepr) -> String {
        match value {
            UnitRepr::Angle(v) => v.into(),
            UnitRepr::Length(v) => v.into(),
        }
    }
}

use yoga::StyleUnit;
use std::f32;

pub fn extract_unit_value(unit: &StyleUnit) -> f32 {
    use self::StyleUnit::*;

    match unit {
        Percent(value) => value.into_inner(),
        Point(value) => value.into_inner(),
        _ => 0.0,
    }
}

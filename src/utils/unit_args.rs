use properties::{Length, Angle, Transform, SharedUnit};
use std::mem::discriminant as mem_entity;

pub fn is_valid_transform(name: &str, units: &Vec<SharedUnit>) -> bool {
    match name {
        "translate" => valid_args_scheme(vec!["length", "length"], &units),
        "skew" => valid_args_scheme(vec!["angle", "angle"], &units),
        "rotate" => valid_args_scheme(vec!["angle"], &units),
        _ => false,
    }
}

pub fn valid_args_scheme(scheme: Vec<&str>, source: &Vec<SharedUnit>) -> bool {
    if scheme.len() == source.len() {
        let mut matches: Vec<bool> = vec![];

        for (index, unit_type) in scheme.iter().enumerate() {
            let entity = mem_entity(&source[index]);

            match unit_type.clone() {
                "length" => matches.push(mem_entity(&SharedUnit::Length(Length::Percent(1.0))) == entity),
                "angle" => matches.push(mem_entity(&SharedUnit::Angle(Angle::Degrees(1.0))) == entity),
                _ => matches.push(false),
            }
        }

        let position_unmatched = matches.iter().position(|matched| !matched);
        position_unmatched.is_none()
    } else {
        false
    }
}

pub fn extract_args_by_type(name: &str, source: &Vec<SharedUnit>) -> Transform {
    match name {
        "translate" => {
            let x = extract!(SharedUnit::Length(_), source[0]).unwrap_or(Length::Point(0.));
            let y = extract!(SharedUnit::Length(_), source[1]).unwrap_or(Length::Point(0.));
            Transform::Translate((x, y))
        }
        "rotate" => {
            let angle = extract!(SharedUnit::Angle(_), source[0]).unwrap_or(Angle::Degrees(0.));
            Transform::Rotate(angle)
        }
        "skew" => {
            let x = extract!(SharedUnit::Angle(_), source[0]).unwrap_or(Angle::Degrees(0.));
            let y = extract!(SharedUnit::Angle(_), source[1]).unwrap_or(Angle::Degrees(0.));
            Transform::Skew((x, y))
        }
        _ => Transform::None,
    }
}

// Tests of parse expressions
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_args_scheme() {
        let units = vec![SharedUnit::Length(Length::Point(1.)), SharedUnit::Length(Length::Point(1.))];
        assert!(valid_args_scheme(vec!["length", "length"], &units), true);

        let units = vec![SharedUnit::Length(Length::Point(1.)), SharedUnit::Angle(Angle::Degrees(1.))];
        assert_eq!(valid_args_scheme(vec!["angle", "length"], &units), false);
        assert!(valid_args_scheme(vec!["length", "angle"], &units), true);
    }
}

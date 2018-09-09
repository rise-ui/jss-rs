use properties::parse::{self, UnitRepr, AngleRepr, LengthRepr};
use euclid;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Angle {
    Degrees(f32),
    Radians(f32),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Length {
    Percent(f32),
    Point(f32),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SharedUnit {
    Length(Length),
    Angle(Angle),
    None,
}

impl From<Length> for String {
    fn from(unit: Length) -> String {
        match unit {
            Length::Percent(value) => format!("{}%", value),
            Length::Point(value) => format!("{}px", value),
        }
    }
}

impl<'a, 'b> From<LengthRepr<'a, 'b>> for SharedUnit {
    fn from(u: LengthRepr) -> SharedUnit {
        let value = u.value.parse::<f32>().unwrap_or(0.0);
        match u.unit {
            "percent" => SharedUnit::Length(Length::Percent(value)),
            "point" => SharedUnit::Length(Length::Point(value)),
            _ => SharedUnit::None,
        }
    }
}

impl From<Angle> for String {
    fn from(unit: Angle) -> String {
        use self::Angle::*;

        match unit {
            Degrees(v) => format!("{}deg", v),
            Radians(v) => format!("{}rad", v),
        }
    }
}

impl From<Angle> for euclid::Angle<f32> {
    fn from(angle: Angle) -> euclid::Angle<f32> {
        match angle {
            Angle::Radians(v) => euclid::Angle::radians(v),
            Angle::Degrees(v) => euclid::Angle::degrees(v),
        }
    }
}

impl<'a, 'b> From<AngleRepr<'a, 'b>> for SharedUnit {
    fn from(u: AngleRepr) -> SharedUnit {
        let value = u.value.parse::<f32>().unwrap_or(0.0);
        match u.angle {
            "radians" => SharedUnit::Angle(Angle::Radians(value)),
            "degrees" => SharedUnit::Angle(Angle::Degrees(value)),
            _ => SharedUnit::None,
        }
    }
}

impl<'a, 'b> From<UnitRepr<'a, 'b>> for SharedUnit {
    fn from(u: UnitRepr) -> SharedUnit {
        match u {
            UnitRepr::Length(length) => length.into(),
            UnitRepr::Angle(angle) => angle.into(),
        }
    }
}

impl<'a> From<&'a str> for SharedUnit {
    fn from(s: &str) -> SharedUnit {
        if let Ok(result) = parse::unit(s.as_bytes()) {
            result.1.into()
        } else {
            SharedUnit::None
        }
    }
}

impl From<String> for SharedUnit {
    fn from(s: String) -> SharedUnit {
        let slice = &*s;
        slice.into()
    }
}

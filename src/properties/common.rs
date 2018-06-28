use properties::parse::{self, UnitRepr};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Angle {
  Degrees(f32),
  Radians(f32)
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Length {
  Percent(f32),
  Point(f32)
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

// #[derive(Debug, PartialEq, Copy, Clone)]
// pub enum SharedUnit {
//   Percent(f32),
//   Angle(Angle),
//   Point(f32),
//   None,
// }

// impl From<Length> for String {
//   fn from(unit: Length) -> String {
//     match unit {
//       Length::Percent(value) => format!("{}%", value),
//       Length::Point(value) => format!("{}px", value)
//     }
//   }
// }

// impl<'a, 'b> From<UnitRepr<'a, 'b>> for SharedUnit {
//   fn from(u: UnitRepr) -> SharedUnit {
//     let value = u.value.parse::<f32>().unwrap_or(0.0);
//     match u.unit {
//       "radians" => SharedUnit::Angle(Angle::Radians(value)),
//       "degrees" => SharedUnit::Angle(Angle::Degrees(value)),
//       "percent" => SharedUnit::Percent(value),
//       "point" => SharedUnit::Point(value),
//       _ => SharedUnit::None,
//     }
//   }
// }

// impl<'a> From<&'a str> for SharedUnit {
//   fn from(s: &str) -> SharedUnit {
//     if let Ok(result) = parse::unit(s.as_bytes()) {
//       result.1.into()
//     } else {
//       SharedUnit::None
//     }
//   }
// }

// impl From<String> for SharedUnit {
//   fn from(s: String) -> SharedUnit {
//     let slice = &*s;
//     slice.into()
//   }
// }

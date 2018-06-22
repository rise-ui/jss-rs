#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Angle {
  Degrees(f32),
  Radians(f32),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SharedUnit {
  Percent(f32),
  Angle(Angle),
  Point(f32),
}

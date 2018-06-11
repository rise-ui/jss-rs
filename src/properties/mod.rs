mod background;
mod border;
mod color;
mod cursor;

pub use self::background::*;
pub use self::border::*;
pub use self::color::*;
pub use self::cursor::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Apperance {
  Background(Background),
  BorderRadius(BorderRadius),
  BorderStyles(BorderStyles),
}

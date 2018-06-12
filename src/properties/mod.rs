mod background;
mod border;
mod color;
mod cursor;
mod filter;

pub use self::background::*;
pub use self::border::*;
pub use self::cursor::*;
pub use self::filter::*;
pub use self::color::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Apperance {
  Background(Background),
  BorderRadius(BorderRadius),
  BorderStyles(BorderStyles),
  Filter(Vec<Filter>),
}

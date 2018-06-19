mod background;
mod border;
mod color;
mod common;
mod cursor;
mod filter;
mod transform;

pub use self::background::*;
pub use self::transform::*;
pub use self::border::*;
pub use self::cursor::*;
pub use self::filter::*;
pub use self::common::*;
pub use self::color::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Apperance {
  pub border_radius: Option<BorderRadius>,
  pub border_styles: Option<BorderStyles>,
  pub background: Option<Background>,
  pub transform: Option<Transforms>,
  pub filter: Option<Filters>,
}

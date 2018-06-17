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

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Apperance {
  pub border_radius: Option<BorderRadius>,
  pub border_styles: Option<BorderStyles>,
  pub background: Option<Background>,
  pub filter: Option<Vec<Filter>>,
}

use properties::Apperance;
use yoga::FlexStyle;

pub trait PrepareStyleExt {
  fn get_prepared_styles(&self) -> (Vec<Apperance>, Vec<FlexStyle>);
}

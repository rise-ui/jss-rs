use yoga::FlexStyle;

pub trait PrepareStyleExt {
  fn get_prepared_layout(&self) -> Vec<FlexStyle>;
}

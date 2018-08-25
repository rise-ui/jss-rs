#[derive(Debug, Fail)]
pub enum PropertyError {
  #[fail(display = "invalid property type for {} - expected {}", property, expected)]
  InvalidType {
    property: String,
    expected: String,
  },
  #[fail(display = "invalid property key {}", key)]
  InvalidKey {
    key: String,
  },
}

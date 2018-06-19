pub type Transform = (TransformType, (f32, f32));
pub type Transforms = Vec<Transform>;

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransformType {
  Translate,
  Rotate,
  Skew,
}

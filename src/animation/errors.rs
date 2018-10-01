/// Common animation errors
#[derive(Debug, Fail)]
pub enum AnimationError {
    #[fail(display = "missing field `{}` in started state", 0)]
    MissingProperty(String),
}
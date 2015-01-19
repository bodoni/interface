#![macro_use]

#[derive(Show)]
pub struct Error {
    message: String,
}

macro_rules! raise(
    ($message:expr) => (return Err(Error::new($message)));
);

macro_rules! some(
    ($option:expr, $message:expr) => (
        match $option {
            Some(object) => object,
            None => raise!($message),
        }
    );
);

impl Error {
    #[inline]
    pub fn new(message: &str) -> Error {
        Error {
            message: message.to_string(),
        }
    }
}

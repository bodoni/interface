#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    #[inline]
    pub fn new(message: &str) -> Error {
        Error {
            message: message.to_string(),
        }
    }
}

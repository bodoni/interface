use std::{error, fmt, result};

/// An error.
pub struct Error(pub String);

/// A result.
pub type Result<T> = result::Result<T, Error>;

#[macro_export]
macro_rules! ok(
    ($result:expr) => (match $result {
        Ok(ok) => ok,
        Err(error) => raise!(error),
    });
    ($result:expr, $($argument:tt)+) => (match $result {
        Ok(ok) => ok,
        Err(..) => raise!($($argument)+),
    });
);

#[macro_export]
macro_rules! raise(
    ($message:expr) => (return Err($crate::Error($message.to_string())));
    ($($argument:tt)+) => (return Err($crate::Error(format!($($argument)+))));
);

impl fmt::Debug for Error {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl error::Error for Error {
    #[inline]
    fn description(&self) -> &str {
        &self.0
    }
}

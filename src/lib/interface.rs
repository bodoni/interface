extern crate curve;
extern crate opentype;
extern crate postscript;

#[macro_use]
extern crate glium;

use glium::Frame;
use std::{error, fmt};

/// An error.
pub struct Error(String);

/// A result.
pub type Result<T> = std::result::Result<T, Error>;

pub trait Object {
    fn render(&self, &mut Frame) -> Result<()>;
}

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

macro_rules! raise(
    ($message:expr) => (return Err(::Error($message.to_string())));
    ($($argument:tt)+) => (return Err(::Error(format!($($argument)+))));
);

impl fmt::Debug for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.0
    }
}

mod glyph;
mod scene;

pub use glyph::Glyph;
pub use scene::Scene;

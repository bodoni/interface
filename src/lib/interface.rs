extern crate font;

#[macro_use]
extern crate glium;

#[macro_use]
mod outcome;

mod context;
mod display;
mod frame;
mod glyph;
mod scene;

pub use display::Display;
pub use frame::{Frame, Indices, Vertices};
pub use glyph::Glyph;
pub use outcome::{Error, Result};
pub use scene::Scene;

/// An object.
pub trait Object {
    /// Render the object.
    fn render(&self, &mut Frame) -> Result<()>;
}

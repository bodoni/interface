#![feature(libc, std_misc)]

extern crate cocoa;
extern crate core_foundation;
extern crate "gl" as raw;
extern crate libc;

pub use error::Error;
pub use result::Result;

pub use event::Event;
pub use window::Window;

mod support;

mod error;
mod result;

mod event;
mod window;

pub mod gl;

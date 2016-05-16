use glium::{self, Program};
use std::ops::{Deref, DerefMut};

use context::Context;
use frame::Frame;
use outcome::Result;

const VERTEX_SHADER: &'static str = include_str!("vertex.glsl");
const FRAGMENT_SHADER: &'static str = include_str!("fragment.glsl");

/// A display.
pub struct Display {
    inner: glium::Display,
    context: Context,
}

impl Display {
    /// Create a display.
    pub fn from(inner: glium::Display) -> Result<Display> {
        let program = ok!(Program::from_source(&inner, VERTEX_SHADER, FRAGMENT_SHADER, None));
        Ok(Display { inner: inner, context: Context::new(program) })
    }

    /// Update the display.
    pub fn update<F>(&mut self, mut closure: F) -> Result<()>
        where F: FnMut(&mut Frame) -> Result<()>
    {
        let mut frame = Frame::from(self.inner.draw(), self.context.clone());
        let result = closure(&mut frame);
        let frame = glium::Frame::from(frame);
        ok!(frame.finish());
        result
    }
}

impl Deref for Display {
    type Target = glium::Display;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Display {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

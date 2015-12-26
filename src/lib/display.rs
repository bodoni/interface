use glium::{self, Program};
use std::ops::{Deref, DerefMut};

use context::Context;
use frame::Frame;
use outcome::Result;

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
        frame.finish().unwrap(); // FIXME: Use ok! when glium > 0.13.2.
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

const VERTEX_SHADER: &'static str = r#"
#version 140

in vec2 position;

void main()
{
    gl_Position = vec4(position, 0.0, 1.0);
}
"#;

const FRAGMENT_SHADER: &'static str = r#"
#version 140

out vec4 color;

void main()
{
    color = vec4(0.0, 0.0, 0.0, 1.0);
}
"#;

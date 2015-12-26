use glium::index::IndicesSource;
use glium::vertex::MultiVerticesSource;
use glium::{self, Surface};
use std::ops::{Deref, DerefMut};

use context::Context;
use outcome::Result;

/// A frame.
pub struct Frame {
    inner: glium::Frame,
    context: Context,
}

impl Frame {
    /// Create a frame.
    #[inline]
    pub fn from(inner: glium::Frame, context: Context) -> Frame {
        Frame { inner: inner, context: context }
    }

    /// Draw on the frame.
    pub fn draw<'a, 'b, V: Vertices<'a>, I: Indices<'b>>(&mut self, vertices: V, indices: I)
                                                         -> Result<()> {

        ok!(self.inner.draw(vertices, indices, &self.context.program,
                            &self.context.uniforms, &self.context.parameters));
        Ok(())
    }
}

impl From<Frame> for glium::Frame {
    #[inline]
    fn from(frame: Frame) -> glium::Frame {
        frame.inner
    }
}

/// Indices.
pub trait Indices<'l>: Into<IndicesSource<'l>> {
}

/// Vertices.
pub trait Vertices<'l>: MultiVerticesSource<'l> {
}

impl Deref for Frame {
    type Target = glium::Frame;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Frame {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'l, T: Into<IndicesSource<'l>>> Indices<'l> for T {
}

impl<'l, T: MultiVerticesSource<'l>> Vertices<'l> for T {
}

use glium::Program;
use glium::draw_parameters::DrawParameters;
use glium::uniforms::EmptyUniforms;
use std::ops::Deref;
use std::rc::Rc;

/// A drawing context.
pub struct Context(Rc<Content>);

/// The content of a context.
pub struct Content {
    /// A program.
    pub program: Program,
    /// A set of uniforms.
    pub uniforms: EmptyUniforms,
    /// A set of parameters.
    pub parameters: DrawParameters<'static>,
}

impl Context {
    /// Create a context.
    pub fn new(program: Program) -> Context {
        Context(Rc::new(Content {
            program: program,
            uniforms: uniform!(),
            parameters: Default::default(),
        }))
    }
}

impl Clone for Context {
    #[inline]
    fn clone(&self) -> Context {
        Context(self.0.clone())
    }
}

impl Deref for Context {
    type Target = Content;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

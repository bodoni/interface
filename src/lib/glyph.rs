use curve::bezier;
use glium::index::{NoIndices, PrimitiveType};
use glium::{Display, VertexBuffer};
use postscript::type2::Program;

use {Object, Result};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

pub struct Glyph {
    indices: NoIndices,
    buffer: VertexBuffer<Vertex>,
}

impl Glyph {
    pub fn new<'l>(display: &Display, program: Program<'l>) -> Result<Glyph> {
        let indices = NoIndices(PrimitiveType::LineStrip);
        let buffer = ok!(VertexBuffer::new(display, &try!(construct(program))),
                         "failed to create a vertex buffer");
        Ok(Glyph { indices: indices, buffer: buffer })
    }
}

impl Object for Glyph {
    fn render(&self) -> Result<()> {
        Ok(())
    }
}

fn construct<'l>(mut program: Program<'l>) -> Result<Vec<Vertex>> {
    let mut vertices = Vec::new();
    while let Some((operator, arguments)) = ok!(program.next()) {
    }
    Ok(vertices)
}

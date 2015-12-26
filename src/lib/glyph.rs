use glium::index::{NoIndices, PrimitiveType};
use glium::{Display, VertexBuffer};
use postscript::type2::Program;

use Object;
use frame::Frame;
use outcome::Result;

#[derive(Copy, Clone)]
struct Point {
    position: [f32; 2],
}

implement_vertex!(Point, position);

/// A glyph.
pub struct Glyph {
    vertex_buffer: VertexBuffer<Point>,
    index_buffer: NoIndices,
}

impl Glyph {
    /// Create a glyph.
    pub fn new<'l>(display: &Display, program: Program<'l>) -> Result<Glyph> {
        Ok(Glyph {
            vertex_buffer: ok!(VertexBuffer::new(display, &try!(construct(program))),
                               "failed to create a vertex buffer"),
            index_buffer: NoIndices(PrimitiveType::LineStrip),
        })
    }
}

impl Object for Glyph {
    fn render(&self, frame: &mut Frame) -> Result<()> {
        frame.draw(&self.vertex_buffer, &self.index_buffer)
    }
}

fn construct<'l>(mut program: Program<'l>) -> Result<Vec<Point>> {
    let vertices = Vec::new();
    while let Some((_, _)) = ok!(program.next()) {
    }
    Ok(vertices)
}

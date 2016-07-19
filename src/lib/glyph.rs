use font;
use glium::index::{NoIndices, PrimitiveType};
use glium::{Display, VertexBuffer};

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
    pub fn new<'l>(display: &Display, glyph: font::Glyph) -> Result<Glyph> {
        Ok(Glyph {
            vertex_buffer: ok!(VertexBuffer::new(display, &try!(construct(glyph))),
                               "failed to create a vertex buffer"),
            index_buffer: NoIndices(PrimitiveType::TriangleStrip),
        })
    }
}

impl Object for Glyph {
    fn render(&self, frame: &mut Frame) -> Result<()> {
        frame.draw(&self.vertex_buffer, &self.index_buffer)
    }
}

#[allow(unused_assignments, unused_mut, unused_variables)]
fn construct(glyph: font::Glyph) -> Result<Vec<Point>> {
    use font::Curve::*;
    use font::Operation::*;

    let mut vertices = Vec::new();
    let mut cursor = (0f32, 0f32);
    let mut first = true;
    for operation in glyph.iter() {
        match operation {
            &Move(a) => {
                cursor = a;
                first = true;
            },
            &Line(a) => {
                cursor = a;
                first = false;
            },
            &Curve(Quadratic(a, b)) => {
                cursor = b;
                first = false;
            },
            &Curve(Cubic(a, b, c)) => {
                cursor = c;
                first = false;
            },
        }
    }
    Ok(vertices)
}

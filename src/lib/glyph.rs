use glium::index::{NoIndices, PrimitiveType};
use glium::{Display, VertexBuffer};
use opentype::postscript::type2::Program;

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
    use opentype::postscript::type2::Operator::*;

    let vertices = Vec::new();
    while let Some((operator, operands)) = ok!(program.next()) {
        let count = operands.len();
        match operator {
            RMoveTo => {
                assert!(count == 2);
            },
            HMoveTo => {
                assert!(count == 1);
            },
            VMoveTo => {
                assert!(count == 1);
            },
            RLineTo => {
                assert!(count % 2 == 0);
            },
            HLineTo => {
                if count % 2 == 0 {
                } else {
                }
            },
            VLineTo => {
                if count % 2 == 0 {
                } else {
                }
            },
            RRCurveTo => {
                assert!(count % 6 == 0);
            },
            HHCurveTo => {
                if count % 4 == 0 {
                } else {
                    assert!((count - 1) % 4 == 0);
                }
            },
            HVCurveTo => {
                if count % 8 == 0 {
                } else if (count - 1) % 8 == 0 {
                } else if (count - 4) % 8 == 0 {
                } else {
                    assert!((count - 4 - 1) % 8 == 0);
                }
            },
            VHCurveTo => {
                if count % 8 == 0 {
                } else if (count - 1) % 8 == 0 {
                } else if (count - 4) % 8 == 0 {
                } else {
                    assert!((count - 4 - 1) % 8 == 0);
                }
            },
            VVCurveTo => {
                if count % 4 == 0 {
                } else {
                    assert!((count - 1) % 4 == 0);
                }
            },
            RCurveLine => {
                assert!(count >= 2 && (count - 2) % 6 == 0);
            },
            RLineCurve => {
                assert!(count >= 6 && (count - 6) % 2 == 0);
            },
            HStem | VStem | HStemHM | VStemHM | HintMask | CntrMask => {},
            _ => unreachable!(),
        }
    }
    Ok(vertices)
}

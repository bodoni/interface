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
            index_buffer: NoIndices(PrimitiveType::TriangleStrip),
        })
    }
}

impl Object for Glyph {
    fn render(&self, frame: &mut Frame) -> Result<()> {
        frame.draw(&self.vertex_buffer, &self.index_buffer)
    }
}

fn construct<'l>(mut program: Program<'l>) -> Result<Vec<Point>> {
    use postscript::type2::Operator::*;

    let mut vertices = Vec::new();
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
    circle(0.0, 0.0, 0.5, 100, &mut vertices);
    Ok(vertices)
}

fn circle(x: f32, y: f32, radius: f32, n: usize, vertices: &mut Vec<Point>) {
    use std::f32::consts::PI;

    macro_rules! push(
        ($x:expr, $y:expr) => (vertices.push(Point { position: [$x, $y] }));
    );

    for i in 0..(n + 1) {
        let j = i % n;
        let theta = 2.0 * PI * (j as f32) / (n as f32);
        push!(x + radius * theta.cos(), y + radius * theta.sin());
        if j % 2 == 0 {
            push!(x, y);
        }
    }
}

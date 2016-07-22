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

impl From<(f32, f32)> for Point {
    #[inline]
    fn from((x, y): (f32, f32)) -> Self {
        Point { position: [x, y] }
    }
}

impl From<font::Offset> for Point {
    #[inline]
    fn from(font::Offset(x, y): font::Offset) -> Self {
        Point { position: [x, y] }
    }
}

impl Glyph {
    /// Create a glyph.
    pub fn new<'l>(display: &Display, glyph: font::Glyph) -> Result<Glyph> {
        let mut vertices = try!(construct(glyph));
        scale(&mut vertices, 0.95);
        Ok(Glyph {
            vertex_buffer: ok!(VertexBuffer::new(display, &vertices),
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

fn construct(glyph: font::Glyph) -> Result<Vec<Point>> {
    use curve::bezier;
    use font::Segment::*;

    const POINTS: usize = 10;
    let mut vertices = Vec::new();
    let mut a = font::Offset::from(0.0);
    for contour in glyph.iter() {
        a += contour.offset;
        vertices.push(a.into());
        for segmet in contour.iter() {
            match segmet {
                &Linear(b) => {
                    let b = a + b;
                    let x = bezier::Linear::new(a.0, b.0);
                    let y = bezier::Linear::new(a.1, b.1);
                    for (x, y) in x.trace(POINTS).zip(y.trace(POINTS)).skip(1) {
                        vertices.push((x, y).into());
                    }
                    a = b;
                },
                &Quadratic(b, c) => {
                    let b = a + b;
                    let c = b + c;
                    let x = bezier::Quadratic::new(a.0, b.0, c.0);
                    let y = bezier::Quadratic::new(a.1, b.1, c.1);
                    for (x, y) in x.trace(POINTS).zip(y.trace(POINTS)).skip(1) {
                        vertices.push((x, y).into());
                    }
                    a = c;
                },
                &Cubic(b, c, d) => {
                    let b = a + b;
                    let c = b + c;
                    let d = c + d;
                    let x = bezier::Cubic::new(a.0, b.0, c.0, d.0);
                    let y = bezier::Cubic::new(a.1, b.1, c.1, d.1);
                    for (x, y) in x.trace(POINTS).zip(y.trace(POINTS)).skip(1) {
                        vertices.push((x, y).into());
                    }
                    a = d;
                },
            }
        }
    }
    Ok(vertices)
}

fn scale(vertices: &mut [Point], fraction: f32) {
    use std::f32::INFINITY;

    let (mut min_x, mut max_x, mut min_y, mut max_y) = (INFINITY, -INFINITY, INFINITY, -INFINITY);
    for &Point { position } in vertices.iter() {
        min_x = min_x.min(position[0]);
        max_x = max_x.max(position[0]);
        min_y = min_y.min(position[1]);
        max_y = max_y.max(position[1]);
    }
    for &mut Point { ref mut position } in vertices.iter_mut() {
        position[0] = 2.0 * fraction * (position[0] - min_x) / (max_x - min_x) - fraction;
        position[1] = 2.0 * fraction * (position[1] - min_y) / (max_y - min_y) - fraction;
    }
}

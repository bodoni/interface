use curve::bezier;
use glium::index::{NoIndices, PrimitiveType};
use postscript::type2::Program;

use Result;
use object::Object;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

pub struct Glyph {
    indices: NoIndices,
    buffer: VertexBuffer,
}

impl Glyph {
    #[inline]
    pub fn new<'l>(window: &Window, program: Program<'l>) -> Result<Glyph> {
        let indices = NoIndices(PrimitiveType::LineStrip);
        let buffer = ok!(VertexBuffer::new(window, try!(construct(program))));
        Ok(Glyph { indices: indices, buffer: buffer })
    }
}

impl Object for Glyph {
    fn render(&self) -> Result<()> {
        Ok(())
    }
}

fn construct<'l>(mut program: Program<'l>) -> Result<Vec<Vertex>> {
    const STEPS: usize = 11;
    const WIDTH: f32 = 800.0;
    const HEIGHT: f32 = 800.0;

    let (mut x, mut y) = (0.0, 0.0);
    let mut new = true;

    #[inline(always)]
    fn push(vertices: &mut Vec<f32>, x: f32, y: f32) {
        vertices.push(x / WIDTH - 0.5);
        vertices.push(y / HEIGHT - 0.5);
    }

    #[inline(always)]
    fn reflect(x: f32, x0: f32) -> f32 {
        x0 - (x - x0)
    }

    let mut vertices = Vec::new();
    let mut control = None;

    while Some((operator, arguments)) = ok!(program.next()) {
        match *command {
            MoveTo(..) => new = true,
            ClosePath => {},
            _ => if new {
                push(&mut vertices, x, y);
                new = false;
            },
        }

        match *command {
            MoveTo(Absolute, ref parameters) => {
                for bunch in Bunch::new(parameters, 2) {
                    x = bunch[0];
                    y = bunch[1];
                }
            },
            MoveTo(Relative, ref parameters) => {
                for bunch in Bunch::new(parameters, 2) {
                    x += bunch[0];
                    y += bunch[1];
                }
            },
            LineTo(Absolute, ref parameters) => {
                for bunch in Bunch::new(parameters, 2) {
                    x = bunch[0];
                    y = bunch[1];
                    push(&mut vertices, x, y);
                }
            },
            LineTo(Relative, ref parameters) => {
                for bunch in Bunch::new(parameters, 2) {
                    x += bunch[0];
                    y += bunch[1];
                    push(&mut vertices, x, y);
                }
            },
            CurveTo(Absolute, ref parameters) => {
                for bunch in Bunch::new(parameters, 6) {
                    for (x, y) in bezier::Cubic::new(STEPS,
                                                     x, y,
                                                     bunch[0], bunch[1],
                                                     bunch[2], bunch[3],
                                                     bunch[4], bunch[5]) {
                        push(&mut vertices, x, y);
                    }
                    control = Some((bunch[2], bunch[3]));
                    x = bunch[4];
                    y = bunch[5];
                }
            },
            CurveTo(Relative, ref parameters) => {
                for bunch in Bunch::new(parameters, 6) {
                    for (x, y) in bezier::Cubic::new(STEPS,
                                                     x, y,
                                                     x + bunch[0], y + bunch[1],
                                                     x + bunch[2], y + bunch[3],
                                                     x + bunch[4], y + bunch[5]) {
                        push(&mut vertices, x, y);
                    }
                    control = Some((x + bunch[2], y + bunch[3]));
                    x += bunch[4];
                    y += bunch[5];
                }
            },
            SmoothCurveTo(Absolute, ref parameters) => {
                for bunch in Bunch::new(parameters, 4) {
                    let (x1, y1) = control.unwrap();
                    for (x, y) in bezier::Cubic::new(STEPS,
                                                     x, y,
                                                     reflect(x1, x), reflect(y1, y),
                                                     bunch[0], bunch[1],
                                                     bunch[2], bunch[3]) {
                        push(&mut vertices, x, y);
                    }
                    control = Some((bunch[0], bunch[1]));
                    x = bunch[2];
                    y = bunch[3];
                }
            },
            SmoothCurveTo(Relative, ref parameters) => {
                for bunch in Bunch::new(parameters, 4) {
                    let (x1, y1) = control.unwrap();
                    for (x, y) in bezier::Cubic::new(STEPS,
                                                     x, y,
                                                     reflect(x1, x), reflect(y1, y),
                                                     x + bunch[0], y + bunch[1],
                                                     x + bunch[2], y + bunch[3]) {
                        push(&mut vertices, x, y);
                    }
                    control = Some((x + bunch[0], y + bunch[1]));
                    x += bunch[2];
                    y += bunch[3];
                }
            },
            ClosePath => {},
            _ => {}
        }
    }

    vertices
}

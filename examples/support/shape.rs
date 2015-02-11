use interface::gl::raw;
use interface::gl::raw::types::GLuint;
use svg::path;

use support::bezier;
use support::Bunch;

pub struct Shape {
    array: Vec<f32>,
    buffer: GLuint,
}

macro_rules! ok(
    ($code:expr) => ({
        let result = $code;
        assert_eq!(raw::GetError(), raw::NO_ERROR);
        result
    });
);

impl Shape {
    #[inline]
    pub fn new(data: &path::Data) -> Shape {
        Shape {
            array: construct(data),
            buffer: unsafe {
                let /* mut */ buffer = 0;
                ok!(raw::GenBuffers(1, &buffer as *const _ as *mut _));
                buffer
            },
        }
    }

    pub fn render(&self) {
        use std::mem::size_of;

        let count = self.array.len() / 2;
        let size = 2 * size_of::<f32>() * count;

        unsafe {
            ok!(raw::BindBuffer(raw::ARRAY_BUFFER, self.buffer));
            ok!(raw::VertexAttribPointer(0, 2, raw::FLOAT, raw::FALSE, 0, 0 as *const _));

            ok!(raw::BufferData(raw::ARRAY_BUFFER, size as i64,
                                (&self.array[]).as_ptr() as *const _, raw::STATIC_DRAW));

            ok!(raw::DrawArrays(raw::LINE_STRIP, 0, count as i32));
        }
    }
}

#[unsafe_destructor]
impl Drop for Shape {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            raw::DeleteBuffers(1, &self.buffer as *const _ as *mut _);
        }
    }
}

fn construct(data: &path::Data) -> Vec<f32> {
    use svg::path::Command::*;
    use svg::path::Positioning::*;

    const CURVE_NODES: usize = 11;

    let (mut x, mut y) = (0.0, 0.0);
    let mut new = true;

    #[inline(always)]
    fn push(array: &mut Vec<f32>, x: f64, y: f64) {
        array.push((x / 800.0 - 0.5) as f32);
        array.push((y / 800.0 - 0.5) as f32);
    }

    #[inline(always)]
    fn reflect(x: f64, x0: f64) -> f64 {
        x0 - (x - x0)
    }

    let mut array = Vec::new();
    let mut control = None;

    for command in data.iter() {
        match *command {
            MoveTo(..) => new = true,
            ClosePath => {},
            _ => if new {
                push(&mut array, x, y);
                new = false;
            },
        }

        match *command {
            MoveTo(Absolute, ref parameters) => {
                for bunch in Bunch::new(&parameters[], 2) {
                    x = bunch[0];
                    y = bunch[1];
                }
            },
            MoveTo(Relative, ref parameters) => {
                for bunch in Bunch::new(&parameters[], 2) {
                    x += bunch[0];
                    y += bunch[1];
                }
            },
            LineTo(Absolute, ref parameters) => {
                for bunch in Bunch::new(&parameters[], 2) {
                    x = bunch[0];
                    y = bunch[1];
                    push(&mut array, x, y);
                }
            },
            LineTo(Relative, ref parameters) => {
                for bunch in Bunch::new(&parameters[], 2) {
                    x += bunch[0];
                    y += bunch[1];
                    push(&mut array, x, y);
                }
            },
            CurveTo(Absolute, ref parameters) => {
                for bunch in Bunch::new(&parameters[], 6) {
                    for (x, y) in bezier::Cubic::new(CURVE_NODES,
                                                     x, y,
                                                     bunch[0], bunch[1],
                                                     bunch[2], bunch[3],
                                                     bunch[4], bunch[5]) {
                        push(&mut array, x, y);
                    }
                    control = Some((bunch[2], bunch[3]));
                    x = bunch[4];
                    y = bunch[5];
                }
            },
            CurveTo(Relative, ref parameters) => {
                for bunch in Bunch::new(&parameters[], 6) {
                    for (x, y) in bezier::Cubic::new(CURVE_NODES,
                                                     x, y,
                                                     x + bunch[0], y + bunch[1],
                                                     x + bunch[2], y + bunch[3],
                                                     x + bunch[4], y + bunch[5]) {
                        push(&mut array, x, y);
                    }
                    control = Some((x + bunch[2], y + bunch[3]));
                    x += bunch[4];
                    y += bunch[5];
                }
            },
            SmoothCurveTo(Absolute, ref parameters) => {
                for bunch in Bunch::new(&parameters[], 4) {
                    let (x1, y1) = control.unwrap();
                    for (x, y) in bezier::Cubic::new(CURVE_NODES,
                                                     x, y,
                                                     reflect(x1, x), reflect(y1, y),
                                                     bunch[0], bunch[1],
                                                     bunch[2], bunch[3]) {
                        push(&mut array, x, y);
                    }
                    control = Some((bunch[0], bunch[1]));
                    x = bunch[2];
                    y = bunch[3];
                }
            },
            SmoothCurveTo(Relative, ref parameters) => {
                for bunch in Bunch::new(&parameters[], 4) {
                    let (x1, y1) = control.unwrap();
                    for (x, y) in bezier::Cubic::new(CURVE_NODES,
                                                     x, y,
                                                     reflect(x1, x), reflect(y1, y),
                                                     x + bunch[0], y + bunch[1],
                                                     x + bunch[2], y + bunch[3]) {
                        push(&mut array, x, y);
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

    array
}

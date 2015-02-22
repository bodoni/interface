use std::ffi::CString;
use std::path::Path;

use interface::gl;
use interface::gl::raw;
use interface::gl::raw::types::{GLenum, GLuint};
use interface::Window;

use svg;

use support::Shape;

macro_rules! ok(
    ($code:expr) => ({
        let result = $code;
        assert_eq!(raw::GetError(), raw::NO_ERROR);
        result
    });
);

pub struct Scene {
    shapes: Vec<Shape>,
}

impl Scene {
    pub fn new(window: &Window, path: &Path) -> Scene {
        use svg::{Event, Tag};
        use svg::path::Data;

        gl::select(window);

        let file = svg::open(path).unwrap();
        let mut shapes = Vec::new();

        for event in file.parse() {
            match event {
                Event::Tag(Tag::Path(_, attributes)) => {
                    let data = attributes.get("d").unwrap();
                    shapes.push(Shape::new(&Data::parse(data).unwrap()));
                },
                _ => {},
            }
        }

        unsafe { create_program() };

        Scene{
            shapes: shapes,
        }
    }

    pub fn render(&self) {
        unsafe {
            raw::ClearColor(1.0, 1.0, 1.0, 1.0);
            raw::Clear(raw::COLOR_BUFFER_BIT);
        };

        for shape in self.shapes.iter() {
            shape.render();
        }
    }
}

unsafe fn create_shader(code: &[u8], typo: GLenum) -> GLuint {
    let code = CString::new(code).unwrap();

    let shader = raw::CreateShader(typo);
    ok!(raw::ShaderSource(shader, 1, &code.as_ptr(), 0 as *const _));
    ok!(raw::CompileShader(shader));

    let /* mut */ status = 0;
    raw::GetShaderiv(shader, raw::COMPILE_STATUS, &status as *const _ as *mut _);
    assert!(status == raw::TRUE);

    shader
}

unsafe fn create_program() {
    use support::shader;

    let vertex = create_shader(shader::VERTEX, raw::VERTEX_SHADER);
    let fragment = create_shader(shader::FRAGMENT, raw::FRAGMENT_SHADER);

    let program = raw::CreateProgram();
    assert!(program != 0);

    ok!(raw::AttachShader(program, vertex));
    ok!(raw::AttachShader(program, fragment));

    ok!(raw::LinkProgram(program));
    ok!(raw::UseProgram(program));

    let position = CString::new("position").unwrap();
    let position = raw::GetAttribLocation(program, position.as_ptr() as *const _);
    assert!(position == 0);

    ok!(raw::VertexAttribPointer(position as GLuint, 2, raw::FLOAT,
                                 raw::FALSE, 0, 0 as *const _));

    let array = 0;
    ok!(raw::GenVertexArrays(1, &array as *const _ as *mut _));
    ok!(raw::BindVertexArray(array));

    ok!(raw::EnableVertexAttribArray(position as GLuint));
}

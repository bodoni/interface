use gl::types::{GLenum, GLuint};
use gl;
use std::ffi::CString;
use std::path::Path;
use svg;

use support::Shape;

macro_rules! ok(
    ($code:expr) => ({
        let result = $code;
        assert_eq!(gl::GetError(), gl::NO_ERROR);
        result
    });
);

pub struct Scene {
    shapes: Vec<Shape>,
}

impl Scene {
    pub fn new<T: AsRef<Path>>(path: T) -> Scene {
        use svg::{Event, Tag};
        use svg::path::Data;

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

        Scene { shapes: shapes }
    }

    pub fn render(&self) {
        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        };

        for shape in self.shapes.iter() {
            shape.render();
        }
    }
}

unsafe fn create_shader(code: &[u8], typo: GLenum) -> GLuint {
    let code = CString::new(code).unwrap();

    let shader = gl::CreateShader(typo);
    ok!(gl::ShaderSource(shader, 1, &code.as_ptr(), 0 as *const _));
    ok!(gl::CompileShader(shader));

    let /* mut */ status = 0;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &status as *const _ as *mut _);
    assert!(status == gl::TRUE);

    shader
}

unsafe fn create_program() {
    use support::shader;

    let vertex = create_shader(shader::VERTEX, gl::VERTEX_SHADER);
    let fragment = create_shader(shader::FRAGMENT, gl::FRAGMENT_SHADER);

    let program = gl::CreateProgram();
    assert!(program != 0);

    ok!(gl::AttachShader(program, vertex));
    ok!(gl::AttachShader(program, fragment));

    ok!(gl::LinkProgram(program));
    ok!(gl::UseProgram(program));

    let position = CString::new("position").unwrap();
    let position = gl::GetAttribLocation(program, position.as_ptr() as *const _);
    assert!(position == 0);

    ok!(gl::VertexAttribPointer(position as GLuint, 2, gl::FLOAT, gl::FALSE, 0, 0 as *const _));

    let array = 0;
    ok!(gl::GenVertexArrays(1, &array as *const _ as *mut _));
    ok!(gl::BindVertexArray(array));

    ok!(gl::EnableVertexAttribArray(position as GLuint));
}

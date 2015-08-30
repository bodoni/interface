use glium::Program;
use glium::glutin::Window;

use Result;
use object::Object;

pub struct Scene {
    program: Program,
    objects: Vec<Box<Object>>,
}

impl Scene {
    #[inline]
    pub fn new(window: &Window) -> Result<Scene> {
        let program = ok!(Program::from_source(window, VERTEX_SHADER, FRAGMENT_SHADER, None));
        Ok(Scene { program: program, objects: vec![] })
    }

    #[inline]
    pub fn append<T: 'static + Object>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }
}

impl Object for Scene {
    fn render(&self) -> Result<()> {
        for object in self.objects.iter() {
            try!(object.render());
        }
        Ok(())
    }
}

const VERTEX_SHADER: &'static str = r#"
#version 140

in vec2 position;

void main()
{
    gl_Position = vec4(position, 0.0, 1.0);
}
"#;

const FRAGMENT_SHADER: &'static str = r#"
#version 140

out vec4 color;

void main()
{
    color = vec4(0.0, 0.0, 0.0, 1.0);
}
"#;

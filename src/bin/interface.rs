extern crate glium;
extern crate opentype;
extern crate postscript;

#[macro_use]
extern crate interface;

use interface::{Display, Glyph, Object, Result, Scene};

fn main() {
    match start() {
        Err(error) => println!("Error: {}.", error),
        _ => {},
    }
}

fn start() -> Result<()> {
    use glium::Surface;
    use glium::glutin::Event;

    let mut display = try!(create_display());
    let scene = try!(create_scene(&display));

    loop {
        try!(display.update(|frame| {
            frame.clear_color(0.259, 0.545, 0.792, 1.0);
            scene.render(frame)
        }));
        for event in display.poll_events() {
            match event {
                Event::Closed => return Ok(()),
                _ => {},
            }
        }
    }
}

fn create_display() -> Result<Display> {
    use glium::DisplayBuild;
    use glium::glutin::WindowBuilder;

    let display = ok!(WindowBuilder::new()
                                    .with_dimensions(600, 600)
                                    .with_title("Interface".to_string())
                                    .build_glium());
    Display::from(display)
}

fn create_scene(display: &Display) -> Result<Scene> {
    use opentype::File;
    use postscript::type2::Program;

    let file = ok!(File::open("tests/fixtures/SourceSerifPro-Regular.otf"));
    let fontset = match file.postscript_fontset {
        Some(ref fontset) => fontset,
        _ => raise!("failed to find a font set"),
    };
    let program = Program::new(&fontset.charstrings[0][134],
                               &fontset.global_subroutines,
                               &fontset.local_subroutines[0]);

    let mut scene = try!(Scene::new(&display));
    scene.append(try!(Glyph::new(&display, program)));

    Ok(scene)
}

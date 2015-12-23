extern crate glium;
extern crate interface;
extern crate opentype;
extern crate postscript;

use glium::glutin::{Event, WindowBuilder};
use glium::{DisplayBuild, Surface};
use interface::{Glyph, Object, Scene};
use opentype::File;
use postscript::type2::Program;

macro_rules! ok(($result:expr) => ($result.unwrap()));

fn main() {
    let file = ok!(File::open("tests/fixtures/SourceSerifPro-Regular.otf"));
    let fontset = match file.postscript_fontset {
        Some(ref fontset) => fontset,
        _ => unreachable!(),
    };
    let program = Program::new(&fontset.charstrings[0][134], &fontset.global_subroutines,
                               &fontset.local_subroutines[0]);

    let display = ok!(WindowBuilder::new().with_title("Interface".to_string()).build_glium());
    let mut scene = ok!(Scene::new(&display));
    scene.append(ok!(Glyph::new(&display, program)));

    'outer: loop {
        let mut frame = display.draw();
        frame.clear_color(0.259, 0.545, 0.792, 1.0);
        ok!(scene.render(&mut frame));
        ok!(frame.finish());

        for event in display.poll_events() {
            match event {
                Event::Closed => break 'outer,
                _ => {},
            }
        }
    }
}

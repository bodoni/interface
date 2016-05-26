extern crate font;
extern crate glium;

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

    Display::from(ok!(WindowBuilder::new()
                                    .with_dimensions(600, 600)
                                    .with_title("Interface")
                                    .build_glium()))
}

fn create_scene(display: &Display) -> Result<Scene> {
    use font::File;

    let file = ok!(File::open("tests/fixtures/SourceSerifPro-Regular.otf"));
    let font = match file.get(0) {
        Some(font) => font,
        _ => raise!("failed to find a font"),
    };
    let glyph = match ok!(font.draw('&')) {
        Some(glyph) => glyph,
        _ => raise!("failed to draw a glyph"),
    };
    let mut scene = try!(Scene::new(&display));
    scene.append(try!(Glyph::new(&display, glyph)));
    Ok(scene)
}

#![feature(path, std_misc, unsafe_destructor)]

extern crate interface;
extern crate svg;

use std::path::Path;

use support::Scene;

mod support;

fn main() {
    use interface::{Event, Window};

    let mut window = Window::new().unwrap();
    let scene = Scene::new(&window, &Path::new("examples/benton.svg"));

    Event::subscribe(&mut window);
    loop {
        match window.react() {
            Some(Event::WindowClosed) => break,
            _ => {},
        }
        scene.render();
        window.update();
    }
}

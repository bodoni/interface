extern crate svg;
extern crate gl;
extern crate glutin;

use glutin::{Event, Window};

use support::Scene;

mod support;

fn main() {
    let window = Window::new().unwrap();
    window.set_inner_size(600, 600);

    unsafe {
        window.make_current().unwrap();
        gl::load_with(|symbol| window.get_proc_address(symbol));
    }

    let scene = Scene::new("examples/benton.svg");

    for event in window.wait_events() {
        match event {
            Event::Closed => break,
            _ => ()
        }
        scene.render();
        window.swap_buffers().unwrap();
    }
}

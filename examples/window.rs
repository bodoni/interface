extern crate interface;

use interface::gl;
use interface::{Event, Window};

fn main() {
    let mut window = Window::new().unwrap();

    gl::select(&window);
    unsafe { gl::raw::ClearColor(0.259, 0.545, 0.792, 1.0) };

    Event::subscribe(&mut window);
    loop {
        match window.react() {
            Some(Event::WindowClosed) => break,
            _ => {},
        }
        unsafe { gl::raw::Clear(gl::raw::COLOR_BUFFER_BIT) };
        window.update();
    }
}

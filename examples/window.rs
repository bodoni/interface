#![allow(unstable)]

extern crate gl;
extern crate interface;

use interface::{OpenGL, Window};
use interface::Event::WindowClosed;

fn main() {
    let mut window = Window::new().unwrap();

    gl::load_with(|name| OpenGL::resolve(name));
    unsafe { gl::ClearColor(0.259, 0.545, 0.792, 1.0) };

    loop {
        match window.react() {
            Some(WindowClosed) => break,
            _ => {
                unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
                window.update();
            },
        }
    }
}

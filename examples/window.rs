#![allow(unstable)]

extern crate interface;

use interface::{gl, Window};
use interface::Event::{self, WindowClosed};

fn main() {
    let mut window = Window::new().unwrap();

    gl::select(&window);
    unsafe { gl::raw::ClearColor(0.259, 0.545, 0.792, 1.0) };

    Event::subscribe(&mut window);
    loop {
        match window.react() {
            Some(WindowClosed) => break,
            _ => {},
        }
        unsafe { gl::raw::Clear(gl::raw::COLOR_BUFFER_BIT) };
        window.update();
    }
}

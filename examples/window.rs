#![allow(unstable)]

extern crate interface;

fn main() {
    let mut window = interface::Window::new().unwrap();

    while !window.is_closed() {
        window.react();
        window.update();
    }
}

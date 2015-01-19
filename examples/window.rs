#![allow(unstable)]

extern crate interface;

use interface::Window;
use interface::Event::WindowClosed;

fn main() {
    let mut window = Window::new().unwrap();
    loop {
        match window.react() {
            Some(WindowClosed) => break,
            _ => window.update(),
        }
    }
}

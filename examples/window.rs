#![allow(unstable)]

extern crate interface;

use std::io::timer;
use std::time::duration::Duration;

fn main() {
    let _ = interface::Window::new().unwrap();
    timer::sleep(Duration::seconds(5));
}

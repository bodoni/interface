extern crate glium;

use glium::{DisplayBuild, Surface};
use glium::glutin::{Event, WindowBuilder};

macro_rules! ok(
    ($result:expr) => (match $result {
        Ok(value) => value,
        _ => unreachable!(),
    });
);

fn main() {
}

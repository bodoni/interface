extern crate glium;
extern crate interface;

use glium::{DisplayBuild, Surface};
use glium::glutin::{Event, WindowBuilder};

macro_rules! ok(($result:expr) => ($result.unwrap()));

fn main() {
    let display = ok!(WindowBuilder::new().with_title("Interface".to_string()).build_glium());
    'outer: loop {
        let mut target = display.draw();
        target.clear_color(0.259, 0.545, 0.792, 1.0);
        ok!(target.finish());

        for event in display.poll_events() {
            match event {
                Event::Closed => break 'outer,
                _ => {},
            }
        }
    }
}

extern crate gl;
extern crate glutin;

use glutin::{Event, Window};

fn main() {
    let window = Window::new().unwrap();

    unsafe {
        window.make_current().unwrap();
        gl::load_with(|symbol| window.get_proc_address(symbol));
        gl::ClearColor(0.259, 0.545, 0.792, 1.0);
    }

    for event in window.wait_events() {
        match event {
            Event::Closed => break,
            _ => ()
        }
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
        window.swap_buffers().unwrap();
    }
}

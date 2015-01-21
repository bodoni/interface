use cocoa::base::id;

use Window;

const DELEGATE_NAME: &'static [u8] = b"Delegate\0";
const DELEGATE_IVAR: &'static [u8] = b"data";

#[derive(Clone, Copy, Show)]
pub enum Event {
    LeftMouseDown,
    WindowClosed,
    WindowResized,
}

impl Event {
    #[inline]
    pub fn notify(window: &mut Window) {
        unsafe { create_delegate(::window::get_window(window), window as *mut _) };
    }
}

unsafe fn create_delegate(window: id, data: *mut Window) -> id {
    use cocoa::base::{class, msg_send, selector};
    use cocoa::base::{class_addMethod, class_addIvar};
    use cocoa::base::{objc_allocateClassPair, objc_registerClassPair};
    use cocoa::base::object_setInstanceVariable;
    use libc::intptr_t;
    use std::ffi::CString;
    use std::mem::size_of;

    let object = class("NSObject");
    let delegate = objc_allocateClassPair(object,
                                          DELEGATE_NAME.as_ptr() as *const _,
                                          0);

    class_addMethod(delegate,
                    selector("windowShouldClose:"),
                    window_should_close,
                    CString::from_slice("B@:@".as_bytes()).as_ptr());

    class_addMethod(delegate,
                    selector("windowDidResize:"),
                    window_did_resize,
                    CString::from_slice("V@:@".as_bytes()).as_ptr());

    class_addIvar(delegate,
                  DELEGATE_IVAR.as_ptr() as *const _,
                  size_of::<intptr_t>() as u64,
                  3 /* log2(8) */,
                  CString::from_slice("?".as_bytes()).as_ptr());

    objc_registerClassPair(delegate);

    let delegate: id = msg_send()(delegate, selector("alloc"));
    let _: id = msg_send()(delegate, selector("init"));
    let _: id = msg_send()(window, selector("setDelegate:"), delegate);

    object_setInstanceVariable(delegate,
                               DELEGATE_IVAR.as_ptr() as *const _,
                               data as *mut _);

    delegate
}

extern fn window_should_close(this: id, _: id) -> id {
    unsafe {
        let window = get_data::<Window>(this);
        (*window).send(Event::WindowClosed);
    }
    0
}

extern fn window_did_resize(this: id, _: id) -> id {
    unsafe {
        let window = get_data::<Window>(this);
        (*window).send(Event::WindowResized);
    }
    0
}

unsafe fn get_data<T>(this: id) -> *mut T {
    use cocoa::base::object_getInstanceVariable;
    use std::ptr::null_mut;

    let mut value = null_mut();

    object_getInstanceVariable(this,
                               DELEGATE_IVAR.as_ptr() as *const _,
                               &mut value);

    assert!(!value.is_null());

    value as *mut _
}

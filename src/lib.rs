#![allow(unstable)]

extern crate cocoa;
extern crate libc;

use cocoa::base::{id, nil};
use std::cell::Cell;
use std::collections::RingBuf;

static DELEGATE_NAME: &'static [u8] = b"InterfaceDelegate\0";
static DELEGATE_WINDOW: &'static [u8] = b"interface_window";

#[derive(Show)]
pub struct Error {
    message: String,
}

pub type Result<T> = std::result::Result<T, Error>;

macro_rules! raise(
    ($message:expr) => (return Err(Error::new($message)));
);

macro_rules! some(
    ($option:expr, $message:expr) => (
        match $option {
            Some(object) => object,
            None => raise!($message),
        }
    );
);

#[derive(Clone, Show, Copy)]
pub enum Event {
    LeftMouseDown,
}

#[allow(dead_code, missing_copy_implementations)]
pub struct Window {
    window: id,
    view: id,
    context: id,
    delegate: id,

    is_closed: Cell<bool>,
}

impl Error {
    #[inline]
    fn new(message: &str) -> Error {
        Error {
            message: message.to_string(),
        }
    }
}

impl Window {
    pub fn new() -> Result<Box<Window>> {
        use cocoa::base::{class, msg_send, selector};
        use cocoa::base::{class_addMethod, class_addIvar};
        use cocoa::base::{objc_allocateClassPair, objc_registerClassPair};
        use cocoa::base::object_setInstanceVariable;
        use cocoa::appkit::{NSApplication, NSWindow};
        use libc::intptr_t;
        use std::ffi::CString;
        use std::mem::size_of;

        unsafe {
            let application = some!(create_application(), "cannot create an application");
            let window = some!(create_window("Hello"), "cannot create a window");
            let view = some!(create_view(window), "cannot create a view");
            let context = some!(create_context(view), "cannot create a context");

            application.activateIgnoringOtherApps_(true);
            window.makeKeyAndOrderFront_(nil);

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
                          DELEGATE_WINDOW.as_ptr() as *const _,
                          size_of::<intptr_t>() as u64,
                          3 /* log2(8) */,
                          CString::from_slice("?".as_bytes()).as_ptr());

            objc_registerClassPair(delegate);

            let delegate: id = msg_send()(delegate, selector("alloc"));
            let _: id = msg_send()(delegate, selector("init"));
            let _: id = msg_send()(window, selector("setDelegate:"), delegate);

            let mut window = Box::new(Window {
                window: window,
                view: view,
                context: context,
                delegate: delegate,

                is_closed: Cell::new(false),
            });

            object_setInstanceVariable(delegate,
                                       DELEGATE_WINDOW.as_ptr() as *const _,
                                       &mut *window as *mut Window as *mut _);

            Ok(window)
        }
    }

    pub fn react(&mut self) -> RingBuf<Event> {
        use cocoa::appkit::{NSApp, NSApplication, NSDate, NSDefaultRunLoopMode};
        use cocoa::appkit::NSEventMask::NSAnyEventMask;

        unsafe {
            let event = NSApp().nextEventMatchingMask_untilDate_inMode_dequeue_(
                NSAnyEventMask as u64,
                NSDate::distantFuture(nil),
                NSDefaultRunLoopMode,
                false);

            NSApp().sendEvent_(event);
        }

        unsafe { self.poll() }
    }

    unsafe fn poll(&mut self) -> RingBuf<Event> {
        use cocoa::appkit::{NSApp, NSApplication, NSDate, NSEvent};
        use cocoa::appkit::NSDefaultRunLoopMode;
        use cocoa::appkit::NSEventMask::NSAnyEventMask;
        use cocoa::appkit::NSEventType::NSLeftMouseDown;

        let mut events = RingBuf::new();

        loop {
            let event = NSApp().nextEventMatchingMask_untilDate_inMode_dequeue_(
                NSAnyEventMask as u64,
                NSDate::distantPast(nil),
                NSDefaultRunLoopMode,
                true);

            if event == nil {
                break;
            }

            NSApp().sendEvent_(event);

            match event.get_type() {
                NSLeftMouseDown => events.push_back(Event::LeftMouseDown),
                _ => {},
            }
        }
        events
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed.get()
    }

    pub fn update(&self) {
        use cocoa::appkit::NSOpenGLContext;

        unsafe {
            self.context.flushBuffer();
        }
    }
}

unsafe fn create_application() -> Option<id> {
    use cocoa::appkit::{NSApp, NSApplication};
    use cocoa::appkit::NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular;

    let application = NSApp();
    if application == nil {
        None
    } else {
        application.setActivationPolicy_(NSApplicationActivationPolicyRegular);
        application.finishLaunching();
        Some(application)
    }
}

unsafe fn create_window(title: &str) -> Option<id> {
    use cocoa::base::NSUInteger;
    use cocoa::appkit::{NSPoint, NSRect, NSSize, NSString};
    use cocoa::appkit::NSWindow;
    use cocoa::appkit::NSWindowMask::{NSClosableWindowMask,
                                      NSMiniaturizableWindowMask,
                                      NSResizableWindowMask,
                                      NSTitledWindowMask};
    use cocoa::appkit::NSBackingStoreType::NSBackingStoreBuffered;

    let frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(800.0, 600.0));

    let masks = NSClosableWindowMask as NSUInteger |
                NSMiniaturizableWindowMask as NSUInteger |
                NSResizableWindowMask as NSUInteger |
                NSTitledWindowMask as NSUInteger;

    let window = NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
        frame,
        masks,
        NSBackingStoreBuffered,
        false);

    if window == nil {
        None
    } else {
        let title = NSString::alloc(nil).init_str(title);
        window.setTitle_(title);
        window.setAcceptsMouseMovedEvents_(true);
        window.center();
        Some(window)
    }
}

unsafe fn create_view(window: id) -> Option<id> {
    use cocoa::appkit::{NSView, NSWindow};

    let view = NSView::alloc(nil).init();
    if view == nil {
        None
    } else {
        view.setWantsBestResolutionOpenGLSurface_(true);
        window.setContentView_(view);
        Some(view)
    }
}

unsafe fn create_context(view: id) -> Option<id> {
    use cocoa::appkit::NSOpenGLPFAOpenGLProfiles::NSOpenGLProfileVersion4_1Core;
    use cocoa::appkit::{NSOpenGLContext, NSOpenGLPixelFormat};
    use cocoa::appkit::NSOpenGLPixelFormatAttribute::{NSOpenGLPFAAlphaSize,
                                                      NSOpenGLPFAClosestPolicy,
                                                      NSOpenGLPFAColorSize,
                                                      NSOpenGLPFADepthSize,
                                                      NSOpenGLPFADoubleBuffer,
                                                      NSOpenGLPFAOpenGLProfile,
                                                      NSOpenGLPFAStencilSize};

    let profile = NSOpenGLProfileVersion4_1Core as u32;

    let attributes = [
        NSOpenGLPFAAlphaSize as u32, 8,
        NSOpenGLPFAClosestPolicy as u32,
        NSOpenGLPFAColorSize as u32, 24,
        NSOpenGLPFADepthSize as u32, 24,
        NSOpenGLPFADoubleBuffer as u32,
        NSOpenGLPFAOpenGLProfile as u32, profile,
        NSOpenGLPFAStencilSize as u32, 8,
        0,
    ];

    let format = NSOpenGLPixelFormat::alloc(nil).initWithAttributes_(&attributes);
    if format == nil {
        return None;
    }

    let context = NSOpenGLContext::alloc(nil).initWithFormat_shareContext_(format, nil);
    if context == nil {
        None
    } else {
        context.setView_(view);
        Some(context)
    }
}

extern fn window_should_close(this: id, _: id) -> id {
    use cocoa::base::object_getInstanceVariable;
    use std::ptr::null_mut;

    unsafe {
        let mut value = null_mut();

        object_getInstanceVariable(this,
                                   DELEGATE_WINDOW.as_ptr() as *const _,
                                   &mut value);

        assert!(value != nil as *mut _);

        let window = value as *mut Window;

        (*window).is_closed.set(true);
    }

    0
}

extern fn window_did_resize(_: id, _: id) -> id {
    0
}

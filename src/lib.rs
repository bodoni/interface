#![allow(unstable)]

extern crate cocoa;
extern crate libc;

use cocoa::base::{id, nil};

static DELEGATE_NAME: &'static [u8] = b"interface_window_delegate\0";
static DELEGATE_STATE_IVAR: &'static [u8] = b"interface_state";

#[derive(Show)]
pub struct Error {
    message: String,
}

pub type Result<T> = std::result::Result<T, Error>;

#[allow(dead_code, missing_copy_implementations)]
pub struct Window {
    window: id,
    view: id,
    context: id,
    delegate: id,
}

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

impl Error {
    #[inline]
    fn new(message: &str) -> Error {
        Error {
            message: message.to_string(),
        }
    }
}

impl Window {
    pub fn new() -> Result<Window> {
        use cocoa::base::{class, msg_send, selector};
        use cocoa::base::{class_addMethod, class_addIvar};
        use cocoa::base::{objc_allocateClassPair, objc_registerClassPair};
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
                                                  DELEGATE_NAME.as_ptr() as *const _, 0);

            class_addMethod(delegate,
                            selector("windowShouldClose:"),
                            window_should_close,
                            CString::from_slice("B@:@".as_bytes()).as_ptr());

            class_addMethod(delegate,
                            selector("windowDidResize:"),
                            window_did_resize,
                            CString::from_slice("V@:@".as_bytes()).as_ptr());

            class_addIvar(delegate,
                          DELEGATE_STATE_IVAR.as_ptr() as *const _,
                          size_of::<intptr_t>() as u64,
                          3,
                          CString::from_slice("?".as_bytes()).as_ptr());

            objc_registerClassPair(delegate);

            let delegete: id = msg_send()(delegate, selector("alloc"));
            let delegete: id = msg_send()(delegete, selector("init"));
            let _: id = msg_send()(window, selector("setDelegate:"), delegete);

            Ok(Window {
                window: window,
                view: view,
                context: context,
                delegate: delegate,
            })
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
        false,
        );

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

extern fn window_should_close(_: id, _: id) -> id {
    0
}

extern fn window_did_resize(_: id, _: id) -> id {
    0
}

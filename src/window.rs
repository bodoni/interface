use cocoa::base::{id, nil};
use std::collections::RingBuf;

use {Error, Event, Result};

#[allow(dead_code)]
pub struct Window {
    application: id,
    window: id,
    view: id,
    context: id,

    events: RingBuf<Event>,
}

impl Window {
    pub fn new() -> Result<Window> {
        use cocoa::appkit::{NSApplication, NSWindow};

        unsafe {
            let application = some!(create_application(), "cannot create an application");
            application.activateIgnoringOtherApps_(true);

            let window = some!(create_window("Hello"), "cannot create a window");
            window.makeKeyAndOrderFront_(nil);

            let view = some!(create_view(window), "cannot create a view");
            let context = some!(create_context(view), "cannot create a context");

            Ok(Window {
                application: application,
                window: window,
                view: view,
                context: context,

                events: RingBuf::new(),
            })
        }
    }

    pub fn react(&mut self) -> Option<Event> {
        unsafe { self.poll() };
        self.events.pop_front()
    }

    pub fn send(&mut self, event: Event) {
        self.events.push_back(event);
    }

    pub fn update(&self) {
        use cocoa::appkit::NSOpenGLContext;

        unsafe { self.context.flushBuffer() };
    }

    unsafe fn poll(&mut self) {
        use cocoa::appkit::{NSApp, NSApplication, NSDate, NSEvent};
        use cocoa::appkit::NSDefaultRunLoopMode;
        use cocoa::appkit::NSEventMask::NSAnyEventMask;
        use cocoa::appkit::NSEventType::NSLeftMouseDown;

        let event = NSApp().nextEventMatchingMask_untilDate_inMode_dequeue_(
            NSAnyEventMask as u64,
            NSDate::distantFuture(nil),
            NSDefaultRunLoopMode,
            false);

        NSApp().sendEvent_(event);

        loop {
            let event = NSApp().nextEventMatchingMask_untilDate_inMode_dequeue_(
                NSAnyEventMask as u64,
                NSDate::distantPast(nil),
                NSDefaultRunLoopMode,
                true);

            if is_nil!(event) {
                break;
            }

            NSApp().sendEvent_(event);

            match event.get_type() {
                NSLeftMouseDown => self.send(Event::LeftMouseDown),
                _ => {},
            }
        }
    }
}

unsafe fn create_application() -> Option<id> {
    use cocoa::appkit::{NSApp, NSApplication};
    use cocoa::appkit::NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular;

    let application = NSApp();
    if is_nil!(application) {
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

    if is_nil!(window) {
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
    if is_nil!(view) {
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
    if is_nil!(format) {
        return None;
    }

    let context = NSOpenGLContext::alloc(nil).initWithFormat_shareContext_(format, nil);
    if is_nil!(context) {
        None
    } else {
        context.setView_(view);
        Some(context)
    }
}

#[inline]
pub fn get_window(window: &Window) -> id {
    window.window
}

#[inline]
pub fn get_context(window: &Window) -> id {
    window.context
}

use std::sync::{Once, ONCE_INIT};

pub use raw;

use Window;

static RESOLVE: Once = ONCE_INIT;

pub fn select(window: &Window) {
    use cocoa::base::{id, msg_send, selector};
    use cocoa::appkit::NSOpenGLContext;

    RESOLVE.call_once(|| raw::load_with(|name| resolve(name)));

    unsafe {
        let context = ::window::get_context(window);
        let _: id = msg_send()(context, selector("update"));
        context.makeCurrentContext();
    }
}

fn resolve<T>(name: &str) -> *const T {
    use core_foundation::base::TCFType;
    use core_foundation::bundle::{CFBundleGetBundleWithIdentifier,
                                  CFBundleGetFunctionPointerForName};
    use core_foundation::string::CFString;
    use std::str::FromStr;

    let framework: CFString = FromStr::from_str("com.apple.opengl").unwrap();
    let framework = unsafe {
        CFBundleGetBundleWithIdentifier(framework.as_concrete_TypeRef())
    };

    let symbol: CFString = FromStr::from_str(name).unwrap();
    let symbol = unsafe {
        CFBundleGetFunctionPointerForName(framework, symbol.as_concrete_TypeRef())
    };

    symbol as *const _
}
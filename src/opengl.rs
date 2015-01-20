#[derive(Copy)]
pub struct OpenGL;

impl OpenGL {
    pub fn resolve<T>(name: &str) -> *const T {
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
}

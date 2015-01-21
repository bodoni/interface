#![macro_use]

macro_rules! is_nil(
    ($result:expr) => ($result == ::cocoa::base::nil);
);

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

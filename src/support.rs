#![macro_use]

macro_rules! is_nil(
    ($result:expr) => ($result == ::cocoa::base::nil);
);

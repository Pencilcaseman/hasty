#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn test_function(x: i32) -> i32 {
    unsafe {
        hasty_test_function(x)
    }
}

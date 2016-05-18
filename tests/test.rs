extern crate libc;
#[macro_use] extern crate va_list;

use libc::{c_char, c_int, c_uint};

extern "C" {
    fn vprintf(f: *const c_char, ...) -> c_int;
}

#[test]
fn test1() {
    to_va_list!(|v| {
        println!("{:?}", v);
        vprintf(b"%d %d\n\0".as_ptr() as *const c_char, v);
    }, 1, 2);
}
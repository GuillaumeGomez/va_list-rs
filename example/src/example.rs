extern crate libc;
#[macro_use] extern crate va_list;

use libc::{c_char, c_int, c_uint, c_double};

extern "C" {
    fn vprintf(f: *const c_char, v: va_list::va_list) -> c_int;
}

fn main() {
    to_va_list!(|v: va_list::va_list| {
        vprintf(b"%d %d %s %f\n\0".as_ptr() as *const c_char, v);
    }, 1, 2, b"salut!\0".as_ptr(), 32f32 as c_double);
}

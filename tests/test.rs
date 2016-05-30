extern crate libc;
#[macro_use] extern crate va_list;

use libc::{c_char, c_int};

extern "C" {
    fn vprintf(f: *const c_char, v: va_list::va_list) -> c_int;
}

#[test]
fn test1() {
    unsafe {
        to_va_list!(|v: va_list::va_list| {
            vprintf(b"%d %d\n\0".as_ptr() as *const c_char, v);
        }, 1 as c_int, 2 as c_int);
        to_va_list!(|v: va_list::va_list| {
            vprintf(b"%d %d %d\n\0".as_ptr() as *const c_char, v);
        }, 1 as c_int, 2 as c_int, 3 as c_int);
        to_va_list!(|v: va_list::va_list| {
            vprintf(b"%d %d %d %d\n\0".as_ptr() as *const c_char, v);
        }, 1 as c_int, 2 as c_int, 3 as c_int, 4 as c_int);
        to_va_list!(|v: va_list::va_list| {
            vprintf(b"%d %d %d %d %d\n\0".as_ptr() as *const c_char, v);
        }, 1 as c_int, 2 as c_int, 3 as c_int, 4 as c_int, 5 as c_int);
    }
}

#[cfg(all(target_os = "windows", target_env = "msvc"))]
mod platform {
    #[link(name = "legacy_stdio_definitions")] extern {}
}

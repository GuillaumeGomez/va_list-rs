extern crate libc;
#[macro_use] extern crate va_list;

use libc::{c_char, c_int};

extern "C" {
    fn vprintf(f: *const c_char, v: va_list::va_list) -> c_int;
}

fn main() {
    to_va_list!(|v: va_list::va_list| {
        vprintf(b"%d %d %s\n\0".as_ptr() as *const c_char, v);
    }, 1 as c_int, 2 as c_int, b"salut!\0".as_ptr());
}

#[cfg(all(target_os = "windows", target_env = "msvc"))]
mod platform {
    #[link(name = "legacy_stdio_definitions")] extern {}
}

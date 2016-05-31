# va_list-rs [![Build Status](https://travis-ci.org/GuillaumeGomez/va_list-rs.png?branch=master)](https://travis-ci.org/GuillaumeGomez/va_list-rs) [![Build status](https://ci.appveyor.com/api/projects/status/81oe6cvg34hu2449/branch/master?svg=true)](https://ci.appveyor.com/project/GuillaumeGomez/va-list-rs/branch/master)

A way to use C `va_list` from Rust.

## Example:

```Rust
extern crate libc;
#[macro_use] extern crate va_list;

use libc::{c_char, c_int};

// Here we declare the C function with va_list that we'll use.
extern "C" {
    fn vprintf(f: *const c_char, v: va_list::va_list) -> c_int;
}

fn main() {
    // You just have to call this macro and it'll return you the va_list.
    unsafe {
        to_va_list!(|v: va_list::va_list| {
            // And now you can just give the va_list to the C function:
            vprintf(b"%d %d %s\n\0".as_ptr() as *const c_char, v);
        },
        1, 2, b"salut!\0".as_ptr()); // We pass the arguments after the closure.
    }
}
```

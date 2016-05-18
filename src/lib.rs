#![feature(type_ascription)]
#![allow(dead_code)]
#![allow(private_in_public)]

extern crate libc;

#[macro_export]
macro_rules! counter {
    ($arg:expr) => {{
        1
    }};
    ($arg:expr, $arg2:expr) => {{
        2
    }};
    ($arg:expr, $arg2:expr, $($args:expr),*) => {{
        counter!($($args),*) + 2
    }};
    ($arg:expr, $($args:expr),*) => {{
        counter!($($args),*) + 1
    }}
}

#[macro_export]
macro_rules! to_va_list {
    ($func:expr, $($args:expr),*) => {{
        unsafe extern "C" fn call_func(f: *mut libc::c_void, ap: $crate::va_list) {
            let f: &Box<Fn($crate::va_list) + 'static> = std::mem::transmute(f);
            f(ap);
        }

        #[repr(C)]
        pub struct Wrap {
            pub f: extern "C" fn(*mut libc::c_void, $crate::va_list),
            pub c: *mut libc::c_void,
            pub len: c_uint,
        }

        extern "C" {
            fn create_va_list(w: *mut Wrap, ...);
        }

        unsafe {
            let fu = $func;
            let wrap = Wrap {
                f: std::mem::transmute(call_func as usize),
                c: $crate::convert_closure(fu),
                len: counter!($($args),*),
            };
            create_va_list(Box::into_raw(Box::new(wrap)), $($args),*);
        }
    }}
}

#[doc(hidden)]
pub fn convert_closure<F: Fn(va_list) + 'static>(f: F) -> *mut libc::c_void {
    let f: Box<Box<Fn(va_list) + 'static>> = Box::new(Box::new(f));
    Box::into_raw(f) as *mut _
}

#[repr(C)]
struct _va_list;
#[allow(non_camel_case_types)]
pub type va_list = *mut _va_list;

#![allow(dead_code)]
#![allow(private_in_public)]
#![allow(improper_ctypes)]

extern crate libc;

#[doc(hidden)]
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
}

/// This macro is the only thing provided by this crate.
///
/// ## Arguments
///
/// It expects a closure as first argument and then the arguments
/// that'll be used to build the `va_list`.
///
/// ## Example
///
/// ```ignore
/// to_va_list!(|v: va_list::va_list| {
///     vprintf(b"%d %d %s %f\n\0".as_ptr() as *const c_char, v);
/// },
/// 1, 2, b"salut!\0".as_ptr(), 32f32 as c_double);
/// ```
///
/// ## Warning
///
/// It might exist a limit to the number of arguments you can actually pass.
/// A big one, but don't try to send thousands of them at once.
#[macro_export]
macro_rules! to_va_list {
    ($func:expr, $($args:expr),*) => {{
        unsafe extern "C" fn call_func(f: *mut libc::c_void, ap: $crate::va_list) {
            let f: &Box<Fn($crate::va_list) + 'static> = std::mem::transmute(f);
            f(ap);
        }

        unsafe {
            let fu = $func;
            let wrap = $crate::Wrap {
                f: std::mem::transmute(call_func as usize),
                c: $crate::convert_closure(fu),
                len: counter!($($args),*),
            };
            $crate::create_va_list(Box::into_raw(Box::new(wrap)), $($args),*);
        }
    }}
}


#[repr(C)]
#[doc(hidden)]
pub struct Wrap {
    pub f: extern "C" fn(*mut libc::c_void, va_list),
    pub c: *mut libc::c_void,
    pub len: libc::c_uint,
}

extern "C" {
    #[allow(improper_ctypes)]
    #[doc(hidden)]
    pub fn create_va_list(w: *mut Wrap, ...);
}

#[doc(hidden)]
pub fn convert_closure<F: Fn(va_list) + 'static>(f: F) -> *mut libc::c_void {
    let f: Box<Box<Fn(va_list) + 'static>> = Box::new(Box::new(f));
    Box::into_raw(f) as *mut _
}

#[doc(hidden)]
#[allow(non_camel_case_types)]
enum _va_list {}

#[allow(non_camel_case_types)]
pub type va_list = *mut _va_list;

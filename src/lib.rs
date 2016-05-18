#![feature(type_ascription)]
#![allow(dead_code)]

extern crate libc;

use libc::{c_uint, c_void};

use std::mem::transmute;

#[macro_export]
macro_rules! to_va_list {
    ($func:expr, $($args:expr),*) => {{
        let v = vec!($($args),*);
        unsafe {
            $crate::create_va_list($crate::call_func, $crate::convert_closure($func),
                                   v.len() as c_uint, $($args),*);
        }
    }}
}

#[doc(hidden)]
pub fn convert_closure<F: Fn(*mut va_list)>(f: F) -> *mut c_void {
    let f: Box<Box<F>> = Box::new(Box::new(f));
    Box::into_raw(f) as *mut _
}

#[doc(hidden)]
pub fn call_func(f: *mut c_void, ap: va_list) {
    let f: &Box<Fn(va_list) + 'static> = unsafe { transmute(f) };
    f(ap);
}

#[repr(C)]
pub struct va_list;

extern "C" {
    pub fn create_va_list(f: fn(*mut c_void, va_list), a: *mut c_void, n: c_uint, ...);
}

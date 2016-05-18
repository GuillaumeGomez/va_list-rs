extern crate libc;
extern crate c_vec;

use libc::c_void;

macro_rules! caller {
    ($func:expr, $size:expr, $list:expr) => (
        if let Some(el) = $list.pop() {
            caller!($func, $list, $size, el)
        } else {
            call_func($func, $size)
        }
    );
    ($func:expr, $size:expr, $list:expr, $arg:expr) => (
        if let Some(el) = $list.pop() {
            caller!($func, $list, $size, el, $arg)
        } else {
            call_func($func, $size)
        }
    );
    ($func:expr, $size:expr, $list:expr, $($args:expr),*) => (
        if let Some(el) = $list.pop() {
            caller!($func, $list, el, $($args),*)
        } else {
            call_func($func, $size, $($args),*)
        }
    )
}

pub struct VaList {
    args: Vec<*mut c_void>,
}

impl VaList {
    pub fn new() -> VaList {
        VaList {
            args: vec!(),
        }
    }

    pub fn add_arg<T>(&mut self, arg: &T) {
        self.push(arg as *mut _ as *mut c_void);
    }

    pub fn to_c_va_list<F: Fn(*mut va_list)>(self, func: F) {
        let f = transmute(func);
        caller!(func, self.args.len(), self.args);
    }
}

// part to call in C directly
extern "C" fn call_func(f: *mut c_void, num_args: u32, ...) {
    let f: &Box_<Fn(*mut va_list) + 'static> = transmute(f);
    va_list ap;
    va_start(ap, num_args);
    f(ap);
    va_end(ap);
}

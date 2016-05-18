extern crate libc;

use libc::c_void;

#[macro_export]
macro_rules! caller {
    ($func:expr, $nb_args:expr, $($args:expr),*) => {{
        c_func(convert_closure($func), $nb_args, $($args:expr),*);
    }}
}

fn convert_closure<F: Fn(*mut va_list)>(f: F) -> *mut c_void {
    let f: Box<Box<F>> = Box::new(Box::new(f));
    Box::into_raw(f) as *mut _
}

pub struct va_list;

// part to call in C directly
/*extern "C" fn call_func(f: *mut c_void, num_args: u32, ...) {
    let f: &Box_<Fn(*mut va_list) + 'static> = transmute(f);
    va_list ap;
    va_start(ap, num_args);
    f(ap);
    va_end(ap);
}*/

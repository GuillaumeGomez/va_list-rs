extern crate gcc;

fn main() {
    gcc::compile_library("libva_list.a", &["ffi/va_list.c"])
}

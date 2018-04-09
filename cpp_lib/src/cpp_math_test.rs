
use std::os::raw::c_int;

extern {
    fn lib_cpp_add(a: c_int, b: c_int) -> c_int;
}

pub fn pub_lib_cpp_add(a: c_int, b: c_int) -> c_int {
    unsafe {
        return lib_cpp_add(a, b);
    }
}
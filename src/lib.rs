extern crate nanners;
extern crate nanners_sys;

use nanners::*;

#[no_mangle]
pub extern fn make_a_pi() -> f64 {
    3.14
}

// Uncommenting this causes a link error.
/*
#[no_mangle]
pub extern fn method_in_rust_make_a_pi(info: &mut FunctionCallbackInfo) {
    info.get_return_value().set_f64(make_a_pi());
}
*/

extern crate nanners;
extern crate nanners_sys;

use nanners::*;

use std::ffi::CString;

#[no_mangle]
pub extern fn make_a_pi(info: &mut FunctionCallbackInfo) {
    info.get_return_value().set_f64(3.14);
}

#[no_mangle]
pub extern fn init_module(mut target: LocalObject) {
    target.export(&CString::new("make_a_pi").unwrap(), make_a_pi);
}

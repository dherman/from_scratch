extern crate nanners;
extern crate nanners_sys;

use nanners::*;

use std::ffi::CString;

#[no_mangle]
pub extern fn make_a_pi(info: &mut FunctionCallbackInfo) {
    info.set_return(Scope::new().number(3.14));
}

#[no_mangle]
pub extern fn make_a_point_array(info: &mut FunctionCallbackInfo) {
    let scope = Scope::new();
    let mut array = scope.array(3);
    array.set(0, scope.integer(17));
    array.set(1, scope.integer(42));
    array.set(2, scope.integer(1999));
    info.set_return(array);
}

#[no_mangle]
pub extern fn init_module<'a>(mut target: Local<'a, Object>) {
    target.export(&CString::new("make_a_pi").unwrap(), make_a_pi);
    target.export(&CString::new("make_a_point_array").unwrap(), make_a_point_array);
}

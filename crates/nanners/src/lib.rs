extern crate nanners_sys;

use std::mem;
use nanners_sys::raw;
use nanners_sys::{Nan_ReturnValue_Set_double, Nan_FunctionCallbackInfo_GetReturnValue};

#[repr(C)]
pub struct ReturnValue(raw::ReturnValue);

impl ReturnValue {
    pub fn set_f64(&mut self, f: f64) {
        let &mut ReturnValue(ref mut rv) = self;
        unsafe {
            Nan_ReturnValue_Set_double(mem::transmute(rv), f);
        }
    }
}

#[repr(C)]
pub struct FunctionCallbackInfo(raw::FunctionCallbackInfo);

impl FunctionCallbackInfo {
    pub fn get_return_value(&mut self) -> ReturnValue {
        let &mut FunctionCallbackInfo(ref mut info) = self;
        unsafe {
            ReturnValue(Nan_FunctionCallbackInfo_GetReturnValue(mem::transmute(info)))
        }
    }
}

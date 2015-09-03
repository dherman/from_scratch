extern crate nanners_sys;

use std::mem;

#[link(name = "nanners")]
extern {
    fn add17(x: u32) -> u32;
    fn Nan_FunctionCallbackInfo_GetReturnValue(info: *mut FunctionCallbackInfo) -> ReturnValue;
    fn Nan_ReturnValue_Set_double(rv: *mut ReturnValue, f: f64);
}

#[repr(C)]
pub struct ReturnValue {
    #[allow(dead_code)]
    data: [u8; 16] // FIXME: this was calculated from sizes.cc; automate and autogenerate this
}

impl ReturnValue {
    pub fn set_f64(&mut self, f: f64) {
        unsafe {
            Nan_ReturnValue_Set_double(mem::transmute(self), f);
        }
    }
}

#[repr(C)]
pub struct FunctionCallbackInfo {
    #[allow(dead_code)]
    data: [u8; 40] // FIXME: this was calculated from sizes.cc; automate and autogenerate this
}

impl FunctionCallbackInfo {
    pub fn get_return_value(&mut self) -> ReturnValue {
        unsafe {
            Nan_FunctionCallbackInfo_GetReturnValue(mem::transmute(self))
        }
    }
}

#[cfg(test)]
mod test {
    use ::add17;

    #[test]
    fn go() {
        assert!(unsafe { add17(1) } == 18);
    }
}

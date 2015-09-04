pub mod raw;

use raw::{ReturnValue, FunctionCallbackInfo, LocalObject};

extern "system" {
    pub fn add17(x: u32) -> u32;
    pub fn Nan_FunctionCallbackInfo_GetReturnValue(info: &mut FunctionCallbackInfo) -> ReturnValue;
    pub fn Nan_ReturnValue_Set_double(rv: &mut ReturnValue, f: f64);
    pub fn Nan_Export(target: LocalObject, name: *const u8, f: extern fn(&mut FunctionCallbackInfo));
}

pub mod raw;

use raw::{FunctionCallbackInfo, LocalObject, LocalString, LocalInteger, LocalNumber, LocalArray, LocalValue, MaybeLocalString, HandleScope, EscapableHandleScope};

extern "system" {
    pub fn Nan_FunctionCallbackInfo_SetReturnValue(info: &mut FunctionCallbackInfo, value: LocalValue);
    pub fn Nan_Export(target: &mut LocalObject, name: *const u8, f: extern fn(&mut FunctionCallbackInfo));
    pub fn Nan_NewObject() -> LocalObject;
    pub fn Nan_NewString(value: *const u8) -> MaybeLocalString;
    pub fn Nan_NewStringN(value: *const u8, length: i32) -> MaybeLocalString;
    pub fn Nan_NewInteger(x: i32) -> LocalInteger;
    pub fn Nan_NewNumber(v: f64) -> LocalNumber;
    pub fn Nan_NewArray(length: u32) -> LocalArray;
    pub fn Nan_ArraySet(array: &mut LocalArray, index: u32, value: LocalValue) -> bool;

    pub fn Nan_MaybeLocalString_ToOption(maybe: &MaybeLocalString, out: &mut LocalString) -> bool;
    pub fn Nan_MaybeLocalString_IsEmpty(maybe: &MaybeLocalString) -> bool;

    pub fn Nan_HandleScope_Drop(scope: &mut HandleScope);
    pub fn Nan_HandleScope_PlacementNew(scope: &mut HandleScope);
    pub fn Nan_EscapableHandleScope_Drop(scope: &mut EscapableHandleScope);
    pub fn Nan_EscapableHandleScope_PlacementNew(scope: &mut EscapableHandleScope);

    //pub fn Nan_MaybeLocalString_ToLocal(maybe: &MaybeLocalString, out: &mut LocalString) -> bool;
    //pub fn Nan_MaybeLocalString_ToLocal(maybe: &mut MaybeLocalString, &mut ) -> LocalString;
}

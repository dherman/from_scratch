use std::mem;

#[repr(C)]
pub struct ReturnValue {
    #[allow(dead_code)]
    data: [u8; 8] // FIXME: this was calculated from sizes.cc; automate and autogenerate this
}

#[repr(C)]
#[derive(Clone)]
pub struct LocalObject {
    #[allow(dead_code)]
    data: [u8; 8] // FIXME: this was calculated from sizes.cc; automate and autogenerate this
}

#[repr(C)]
pub struct LocalValue {
    data: [u8; 8] // FIXME: this was calculated from sizes.cc; automate and autogenerate this
}

impl LocalValue {
    pub fn from_data(data: &[u8; 8]) -> LocalValue {
        LocalValue {
            data: data.clone()
        }
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct LocalString {
    #[allow(dead_code)]
    data: [u8; 8] // FIXME: this was calculated from sizes.cc; automate and autogenerate this
}

/*
#[repr(C)]
pub struct MaybeLocalString {
    #[allow(dead_code)]
    data: [u8; 8] // FIXME: this was calculated from sizes.cc; automate and autogenerate this
}
 */

#[repr(C)]
#[derive(Clone)]
pub struct LocalInteger {
    #[allow(dead_code)]
    data: [u8; 8] // FIXME: this was calculated from sizes.cc; automate and autogenerate this
}

#[repr(C)]
#[derive(Clone)]
pub struct LocalNumber {
    #[allow(dead_code)]
    data: [u8; 8] // FIXME: this was calculated from sizes.cc; automate and autogenerate this
}

#[repr(C)]
#[derive(Clone)]
pub struct LocalArray {
    #[allow(dead_code)]
    data: [u8; 8] // FIXME: this was calculated from sizes.cc; automate and autogenerate this
}

#[repr(C)]
pub struct FunctionCallbackInfo {
    #[allow(dead_code)]
    data: [u8; 16] // FIXME: this was calculated from sizes.cc; automate and autogenerate this
}

#[repr(C)]
pub struct HandleScope {
    #[allow(dead_code)]
    data: [u8; 24] // FIXME: this was calculated from sizes.cc; automate and autogenerate this
}

impl HandleScope {
    pub unsafe fn alloc() -> Self {
        mem::uninitialized()
    }
}

#[repr(C)]
pub struct EscapableHandleScope {
    #[allow(dead_code)]
    data: [u8; 32] // FIXME: this was calculated from sizes.cc; automate and autogenerate this
}

impl EscapableHandleScope {
    pub unsafe fn alloc() -> Self {
        mem::uninitialized()
    }
}

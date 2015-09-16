use std::mem;
use std::os::raw::c_void;

#[repr(C)]
#[allow(raw_pointer_derive)]
#[derive(Clone)]
pub struct Local {
    pub handle: *mut c_void
}

#[repr(C)]
pub struct FunctionCallbackInfo;

#[repr(C)]
pub struct HandleScope {
    #[allow(dead_code)]
    data: [*mut c_void; 3]
}

impl HandleScope {
    pub unsafe fn alloc() -> Self {
        mem::uninitialized()
    }
}

#[repr(C)]
pub struct EscapableHandleScope {
    #[allow(dead_code)]
    data: [*mut c_void; 4]
}

impl EscapableHandleScope {
    pub unsafe fn alloc() -> Self {
        mem::uninitialized()
    }
}

#[repr(C)]
pub struct ReturnValue {
    #[allow(dead_code)]
    data: [u8; 8] // FIXME: this was calculated from sizes.cc; automate and autogenerate this
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct LocalObject {
    #[allow(dead_code)]
    data: [u8; 8] // FIXME: this was calculated from sizes.cc; automate and autogenerate this
}

#[repr(C)]
pub struct FunctionCallbackInfo {
    #[allow(dead_code)]
    data: [u8; 16] // FIXME: this was calculated from sizes.cc; automate and autogenerate this
}

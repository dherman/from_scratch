extern crate nanners_sys;

use std::mem;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut, Drop};
use std::ffi::CStr;
use nanners_sys::raw;
use nanners_sys::{Nan_FunctionCallbackInfo_SetReturnValue, Nan_Export, Nan_NewObject, /*Nan_MaybeLocalString_ToOption, Nan_MaybeLocalString_IsEmpty,*/ Nan_HandleScope_Drop, Nan_HandleScope_PlacementNew, Nan_NewInteger, Nan_NewNumber, Nan_NewArray, Nan_ArraySet, Nan_EscapableHandleScope_Drop, Nan_EscapableHandleScope_PlacementNew};

#[repr(C)]
pub struct FunctionCallbackInfo(raw::FunctionCallbackInfo);

impl FunctionCallbackInfo {
    // GC: Storing a Local in a ReturnValue keeps it alive independent of any HandleScope.
    pub fn set_return<'a, 'b, T: Clone + Value>(&'a mut self, value: Local<'b, T>) {
        let &mut FunctionCallbackInfo(ref mut info) = self;
        unsafe {
            Nan_FunctionCallbackInfo_SetReturnValue(info, value.to_local_value());
        }
    }
}

pub trait Value { }

#[repr(C)]
pub struct Local<'a, T: Clone + Value + 'a> {
    value: T,
    phantom: PhantomData<&'a T>
}

impl<'a, T: Clone + Value> Deref for Local<'a, T> {
    type Target = T;
    fn deref<'b>(&'b self) -> &'b T {
        &self.value
    }
}

impl<'a, T: Clone + Value> DerefMut for Local<'a, T> {
    fn deref_mut<'b>(&'b mut self) -> &'b mut T {
        &mut self.value
    }
}

impl<'a, T: Clone + Value> Local<'a, T> {
    unsafe fn to_local_value(&self) -> raw::LocalValue {
        raw::LocalValue::from_data(mem::transmute(&self.value))
    }
}

#[repr(C)]
pub struct String(raw::LocalString);

impl Value for String { }

#[repr(C)]
#[derive(Clone)]
pub struct Integer(raw::LocalInteger);

impl Value for Integer { }

impl Integer {
    fn new<'a>(i: i32) -> Local<'a, Integer> {
        Local {
            value: Integer(unsafe { Nan_NewInteger(i) }),
            phantom: PhantomData
        }
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct Number(raw::LocalNumber);

impl Value for Number { }

impl Number {
    fn new<'a>(v: f64) -> Local<'a, Number> {
        Local {
            value: Number(unsafe { Nan_NewNumber(v) }),
            phantom: PhantomData
        }
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct Object(raw::LocalObject);

impl Value for Object { }

impl Object {
    fn new<'a>() -> Local<'a, Object> {
        Local {
            value: Object(unsafe { Nan_NewObject() }),
            phantom: PhantomData
        }
    }

    pub fn export(&mut self, name: &CStr, f: extern fn(&mut FunctionCallbackInfo)) {
        let &mut Object(ref mut object) = self;
        unsafe {
            Nan_Export(object, mem::transmute(name.as_ptr()), mem::transmute(f));
        }
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct Array(raw::LocalArray);

impl Value for Array { }

impl Array {
    fn new<'a>(len: u32) -> Local<'a, Array> {
        Local {
            value: Array(unsafe { Nan_NewArray(len) }),
            phantom: PhantomData
        }
    }

    pub fn set<'a, T: Clone + Value>(&mut self, index: u32, value: Local<'a, T>) -> bool {
        match self {
            &mut Array(ref mut array) => {
                unsafe {
                    Nan_ArraySet(array, index, value.to_local_value())
                }
            }
        }
    }
}

pub struct EscapeScope<'a> {
    value: raw::EscapableHandleScope,
    phantom: PhantomData<&'a ()>
}

impl<'a> Drop for EscapeScope<'a> {
    fn drop(&mut self) {
        unsafe {
            Nan_EscapableHandleScope_Drop(&mut self.value);
        }
    }
}

impl<'a> EscapeScope<'a> {
    pub fn new() -> Self {
        let mut result = EscapeScope {
            value: unsafe { raw::EscapableHandleScope::alloc() },
            phantom: PhantomData
        };
        unsafe {
            Nan_EscapableHandleScope_PlacementNew(&mut result.value);
        }
        result
    }

    pub fn integer(&self, i: i32) -> Local<'a, Integer> {
        Integer::new(i)
    }

    pub fn number(&self, v: f64) -> Local<'a, Number> {
        Number::new(v)
    }

    pub fn array(&self, len: u32) -> Local<'a, Array> {
        Array::new(len)
    }

    pub fn object(&self) -> Local<'a, Object> {
        Object::new()
    }

    pub fn escape<'b, T: Clone + Value>(local: Local<'a, T>) -> Local<'b, T> {
        Local {
            value: local.value.clone(),
            phantom: PhantomData
        }
    }
}

pub struct Scope<'a> {
    value: raw::HandleScope,
    phantom: PhantomData<&'a ()>
}

impl<'a> Drop for Scope<'a> {
    fn drop(&mut self) {
        unsafe {
            Nan_HandleScope_Drop(&mut self.value);
        }
    }
}

impl<'a> Scope<'a> {
    // Note that this does the placement new on the callee stack frame and then
    // copies the result out to the caller stack frame. This relies on the fact
    // that the constructor is not making use of the `this` pointer.
    pub fn new() -> Self {
        let mut result = Scope {
            value: unsafe { raw::HandleScope::alloc() },
            phantom: PhantomData
        };
        unsafe {
            Nan_HandleScope_PlacementNew(&mut result.value);
        }
        result
    }

    pub fn integer(&self, i: i32) -> Local<'a, Integer> {
        Integer::new(i)
    }

    pub fn number(&self, v: f64) -> Local<'a, Number> {
        Number::new(v)
    }

    pub fn object(&self) -> Local<'a, Object> {
        Object::new()
    }

    pub fn array(&self, len: u32) -> Local<'a, Array> {
        Array::new(len)
    }
}

/*
#[repr(C)]
pub struct LocalObject(raw::LocalObject);

impl LocalObject {
    pub fn new() -> LocalObject {
        unsafe {
            LocalObject(Nan_NewObject())
        }
    }

    pub fn export(&mut self, name: &CStr, f: extern fn(&mut FunctionCallbackInfo)) {
        let &mut LocalObject(ref mut object) = self;
        unsafe {
            Nan_Export(object, mem::transmute(name.as_ptr()), mem::transmute(f));
        }
    }
}
 */

/*
#[repr(C)]
pub struct LocalString(raw::LocalString);

trait RawMaybeLocalStringExt {
    fn is_empty(&self) -> bool;
    fn to_option(&self) -> Option<LocalString>;
}

impl RawMaybeLocalStringExt for raw::MaybeLocalString {
    fn is_empty(&self) -> bool {
        unsafe {
            Nan_MaybeLocalString_IsEmpty(self)
        }
    }

    fn to_option(&self) -> Option<LocalString> {
        unsafe {
            let mut tmp: raw::LocalString = mem::uninitialized();
            if Nan_MaybeLocalString_ToOption(self, &mut tmp) {
                Some(LocalString(tmp.clone()))
            } else {
                None
            }
        }
    }
}
 */

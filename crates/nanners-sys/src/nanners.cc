#include <new>
#include <nan.h>
#include <stdint.h>
#include <stdio.h>
#include "nanners.h"

extern "C" void Nan_FunctionCallbackInfo_SetReturnValue(Nan_FunctionCallbackInfo *info, Nan_Local value) {
  info->value.GetReturnValue().Set(value.value);
}

extern "C" void Nan_Export(Nan_LocalObject *target, const char *name, Nan::FunctionCallback f) {
  Nan::Export(target->value, name, f);
}

extern "C" Nan_LocalObject Nan_NewObject() {
  struct Nan_LocalObject result = { Nan::New<v8::Object>() };
  return result;
}

extern "C" Nan_MaybeLocalString Nan_NewString(const char *value) {
  struct Nan_MaybeLocalString result = { Nan::New<v8::String>(value) };
  return result;
}

extern "C" Nan_MaybeLocalString Nan_NewStringN(const char *value, int32_t length) {
  struct Nan_MaybeLocalString result = { Nan::New<v8::String>(value, length) };
  return result;
}

extern "C" Nan_LocalInteger Nan_NewInteger(int32_t x) {
  v8::Isolate *isolate = v8::Isolate::GetCurrent();
  struct Nan_LocalInteger result = { v8::Integer::New(isolate, x) };
  return result;
}

extern "C" Nan_LocalNumber Nan_NewNumber(double value) {
  v8::Isolate *isolate = v8::Isolate::GetCurrent();
  struct Nan_LocalNumber result = { v8::Number::New(isolate, value) };
  return result;
}

extern "C" Nan_LocalArray Nan_NewArray(uint32_t length) {
  v8::Isolate *isolate = v8::Isolate::GetCurrent();
  struct Nan_LocalArray result = { v8::Array::New(isolate, length) };
  return result;
}

extern "C" bool Nan_ArraySet(Nan_LocalArray *array, uint32_t index, Nan_Local value) {
  return array->value->Set(index, value.value);
}

extern "C" bool Nan_MaybeLocalString_IsEmpty(Nan_MaybeLocalString *maybe) {
  return maybe->value.IsEmpty();
}

extern "C" bool Nan_MaybeLocalString_ToLocal(Nan_MaybeLocalString *maybe, Nan_LocalString *out) {
  return maybe->value.ToLocal(&out->value);
}

extern "C" void Nan_HandleScope_Drop(Nan_HandleScope *scope) {
  scope->value.~HandleScope();
}

extern "C" void Nan_HandleScope_PlacementNew(Nan_HandleScope *scope) {
  ::new (scope) Nan::HandleScope();
}

extern "C" void Nan_EscapableHandleScope_Drop(Nan_EscapableHandleScope *scope) {
  scope->value.~EscapableHandleScope();
}

extern "C" void Nan_EscapableHandleScope_PlacementNew(Nan_EscapableHandleScope *scope) {
  ::new (scope) Nan::EscapableHandleScope();
}

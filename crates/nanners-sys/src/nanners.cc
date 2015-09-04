#include <nan.h>
#include <stdint.h>
#include "nanners.h"

extern "C" uint32_t add17(uint32_t x) {
  return x + 17;
}

extern "C" uint32_t add5(uint32_t x) {
  return x + 5;
}

extern "C" uint32_t stuff(Nan_ReturnValue x) {
  return 5;
}

extern "C" Nan_ReturnValue Nan_FunctionCallbackInfo_GetReturnValue(Nan_FunctionCallbackInfo *info) {
  struct Nan_ReturnValue result = { info->value.GetReturnValue() };
  return result;
}

extern "C" void Nan_ReturnValue_Set_double(Nan_ReturnValue *rv, double f) {
  rv->value.Set(f);
}

extern "C" void Nan_Export(Nan_LocalObject target, const char *name, Nan::FunctionCallback f) {
  Nan::Export(target.value, name, f);
}


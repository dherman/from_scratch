#include <nan.h>
#include <stdint.h>
#include <v8.h>

extern "C" {

  struct Nan_FunctionCallbackInfo {
    Nan::FunctionCallbackInfo<v8::Value> value;
  };

  struct Nan_LocalArray {
    v8::Local<v8::Array> value;
  };

  struct Nan_LocalObject {
    v8::Local<v8::Object> value;
  };

  struct Nan_LocalString {
    v8::Local<v8::String> value;
  };

  struct Nan_LocalInteger {
    v8::Local<v8::Integer> value;
  };

  struct Nan_LocalNumber {
    v8::Local<v8::Number> value;
  };

  struct Nan_Local {
    v8::Local<v8::Value> value;
  };

  struct Nan_MaybeLocalString {
    Nan::MaybeLocal<v8::String> value;
  };

  struct Nan_HandleScope {
    Nan::HandleScope value;
  };

  struct Nan_EscapableHandleScope {
    Nan::EscapableHandleScope value;
  };

  struct Nan_MaybeBool {
    Nan::Maybe<bool> value;
  };

  void Nan_FunctionCallbackInfo_SetReturnValue(Nan_FunctionCallbackInfo *info, Nan_Local value);

  void Nan_Export(Nan_LocalObject *target, const char *name, Nan::FunctionCallback f);

  Nan_LocalObject Nan_NewObject();

  Nan_MaybeLocalString Nan_NewString(const char *value);

  Nan_MaybeLocalString Nan_NewStringN(const char *value, int32_t length);

  Nan_LocalInteger Nan_NewInteger(int32_t x);

  Nan_LocalNumber Nan_NewNumber(double value);

  Nan_LocalArray Nan_NewArray(uint32_t length);

  bool Nan_ArraySet(Nan_LocalArray *array, uint32_t index, Nan_Local value);

  bool Nan_MaybeLocalString_IsEmpty(Nan_MaybeLocalString *maybe);
  bool Nan_MaybeLocalString_ToLocal(Nan_MaybeLocalString *maybe, Nan_LocalString *out);
  // bool Nan_MaybeLocalString_ToLocal(Nan_MaybeLocalString *maybe, Nan_LocalString *out);

  void Nan_HandleScope_Drop(Nan_HandleScope *scope);
  void Nan_HandleScope_PlacementNew(Nan_HandleScope *scope);
  void Nan_EscapableHandleScope_Drop(Nan_EscapableHandleScope *scope);
  void Nan_EscapableHandleScope_PlacementNew(Nan_EscapableHandleScope *scope);
}

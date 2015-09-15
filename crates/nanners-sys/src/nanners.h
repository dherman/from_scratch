#include <nan.h>
#include <stdint.h>
#include <v8.h>

extern "C" {

  void Nan_FunctionCallbackInfo_SetReturnValue(Nan::FunctionCallbackInfo<v8::Value> *info, v8::Local<v8::Value> value);

  void Nan_Export(v8::Local<v8::Object> *target, const char *name, Nan::FunctionCallback f);

  void Nan_NewObject(v8::Local<v8::Object> *out);

  void Nan_NewInteger(v8::Local<v8::Integer> *out, int32_t x);

  void Nan_NewNumber(v8::Local<v8::Number> *out, double value);

  void Nan_NewArray(v8::Local<v8::Array> *out, uint32_t length);

  bool Nan_ArraySet(v8::Local<v8::Array> *array, uint32_t index, v8::Local<v8::Value> value);

  void Nan_HandleScope_Drop(Nan::HandleScope *scope);
  void Nan_HandleScope_PlacementNew(Nan::HandleScope *scope);
  void Nan_EscapableHandleScope_Drop(Nan::EscapableHandleScope *scope);
  void Nan_EscapableHandleScope_PlacementNew(Nan::EscapableHandleScope *scope);

  typedef void(*Nan_ScopedCallback)(void*, Nan::HandleScope*, void*);

  void Nan_Scoped(void *out, void *closure, Nan_ScopedCallback callback);

  void Nan_UpcastArray(v8::Local<v8::Value> *out, v8::Local<v8::Array> *array);

  /*
  void Nan_NewString(Nan::MaybeLocal<v8::String> *out, const char *value);
  void Nan_NewStringN(Nan::MaybeLocal<v8::String> *out, const char *value, int32_t length);
  bool Nan_MaybeLocalString_IsEmpty(Nan::MaybeLocal<v8::String> *maybe);
  bool Nan_MaybeLocalString_ToLocal(Nan::MaybeLocal<v8::String> *maybe, Nan_LocalString *out);
  */
}

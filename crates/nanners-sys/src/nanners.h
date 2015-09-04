#include <nan.h>
#include <stdint.h>
#include <v8.h>

extern "C" {

  /*
  struct Nan_Value {
    v8::Value value;
  };
  */

  struct Nan_FunctionCallbackInfo {
    Nan::FunctionCallbackInfo<v8::Value> value;
  };

  struct Nan_ReturnValue {
    Nan::ReturnValue<v8::Value> value;
  };

  struct Nan_LocalObject {
    v8::Local<v8::Object> value;
  };

  struct Nan_HandleObject {
    v8::Handle<v8::Object> value;
  };

  Nan_ReturnValue Nan_FunctionCallbackInfo_GetReturnValue(Nan_FunctionCallbackInfo *info);

  void Nan_ReturnValue_Set_double(Nan_ReturnValue *rv, double f);

  void Nan_Export(Nan_LocalObject target, const char *name, Nan::FunctionCallback f);

  uint32_t add17(uint32_t x);
  uint32_t stuff(Nan_ReturnValue x);

}

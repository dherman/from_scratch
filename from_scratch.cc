#include <nan.h>
#include <v8.h>

extern "C" double make_a_pi();
extern "C" void method_in_rust_make_a_pi(Nan::FunctionCallbackInfo<v8::Value> *info);

// This version was working, instead of method_in_rust_make_a_pi:
// NAN_METHOD(method_make_a_pi) {
//   info.GetReturnValue().Set(make_a_pi());
// }

NAN_MODULE_INIT(init_all) {
  Nan::Export(target, "make_a_pi", (Nan::FunctionCallback)method_in_rust_make_a_pi);
}

NODE_MODULE(noder, init_all)

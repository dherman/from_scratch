#include <nan.h>

extern "C" double make_a_pi();

// Uncommenting this causes a linker error when doing cargo build --release on root crate
// extern "C" void method_in_rust_make_a_pi(v8::FunctionCallbackInfo *info);

// This works:
NAN_METHOD(method_make_a_pi) {
  info.GetReturnValue().Set(make_a_pi());
}

NAN_MODULE_INIT(init_all) {
  Nan::Export(target, "make_a_pi", method_make_a_pi);
}

NODE_MODULE(noder, init_all)

#include <nan.h>

extern "C" double make_a_pi();
extern "C" void method_make_a_pi(Nan::FunctionCallbackInfo<v8::Value> *info);

NAN_MODULE_INIT(init_all) {
  Nan::Export(target, "make_a_pi", (Nan::FunctionCallback)method_make_a_pi);
}

NODE_MODULE(noder, init_all)

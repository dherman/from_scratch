#include <nan.h>

extern "C" void init_module(v8::Local<v8::Object> target);

NODE_MODULE(from_scratch, init_module)

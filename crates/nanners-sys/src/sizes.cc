#include <stdio.h>
#include <v8.h>
#include "nanners.h"

int main() {
  printf("{\n");
  printf("  \"v8::Value\": %lu\n", sizeof(v8::Value));
  printf("  \"Nan_FunctionCallbackInfo\": %lu\n", sizeof(Nan_FunctionCallbackInfo));
  printf("  \"Nan_ReturnValue\": %lu\n", sizeof(Nan_ReturnValue));
  printf("}\n");
  return 0;
}

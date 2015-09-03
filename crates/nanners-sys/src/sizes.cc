#include <stdio.h>
#include <v8.h>
#include "nanners.h"

// This program calculates the sizes of types so opaque Rust structs
// that wrap them can be defined with the right size (see src/lib.rs
// in the nanners crate).

// FIXME: this program should be autogenerated from a declarative file
// that lists out the nan/v8/node types I want to wrap.

int main() {
  printf("{\n");
  printf("  \"Nan_FunctionCallbackInfo\": %lu\n", sizeof(Nan_FunctionCallbackInfo));
  printf("  \"Nan_ReturnValue\": %lu\n", sizeof(Nan_ReturnValue));
  printf("}\n");
  return 0;
}

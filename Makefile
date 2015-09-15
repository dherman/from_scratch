rustsrc=$(wildcard **/build.rs) $(wildcard src/*.rs) $(wildcard crates/nanners/src/*.rs) $(wildcard crates/nanners-sys/src/*.rs)
csrc=$(wildcard crates/nanners-sys/src/*.h) $(wildcard crates/nanners-sys/src/*.cc)
staticlib=target/release/libfrom_scratch.a
dylib=target/release/libfrom_scratch.dylib

build/Release/from_scratch.node: $(dylib) from_scratch.cc
	npm install

$(dylib): $(staticlib)
	npm run build
	gcc -dynamiclib -Wl,-undefined,dynamic_lookup -Wl,-force_load,$(staticlib) -o $(dylib)

# We have to use nightly until the patch lands that stops the dylib
# from clobbering jemalloc symbols in the node executable.
$(staticlib): $(rustsrc) $(csrc)
	multirust run nightly cargo build --release

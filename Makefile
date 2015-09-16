rustsrc=$(wildcard **/build.rs) $(wildcard src/*.rs) $(wildcard crates/nanners/src/*.rs) $(wildcard crates/nanners-sys/src/*.rs)
csrc=$(wildcard crates/nanners-sys/src/*.h) $(wildcard crates/nanners-sys/src/*.cc)
staticlib=target/release/libfrom_scratch.a
dylib=target/release/libfrom_scratch.dylib

build/Release/from_scratch.node: $(dylib) from_scratch.cc
	npm install

$(dylib): $(staticlib)
	npm run build
	gcc -dynamiclib -Wl,-undefined,dynamic_lookup -Wl,-force_load,$(staticlib) -o $(dylib)

# We have to use nightly until https://github.com/rust-lang/rust/pull/27400 makes it
# through the trains to stable. That patch fixes the problem where jemalloc symbols
# in the node executable get clobbered by the dylib.
$(staticlib): $(rustsrc) $(csrc)
	multirust run nightly cargo build --release

staticlib=target/release/libfrom_scratch.a
dylib=target/release/libfrom_scratch.dylib

build/Release/from_scratch.node: $(dylib)
	npm install

$(dylib): $(staticlib)
	npm run build
	gcc -dynamiclib -Wl,-undefined,dynamic_lookup -Wl,-force_load,$(staticlib) -o $(dylib)

$(staticlib): src/lib.rs
	cargo build --release

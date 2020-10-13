.default_goal := all
SOURCES=$(sort $(wildcard ./rust/ffi/src/*.rs ./rust/**/*.rs))
all: ios android macos

macos: rust/target/release/liblexibook_ffi.a

rust/target/release/liblexibook_ffi.a: $(SOURCES)
	@if [ $$(uname) == "Darwin" ] ; then \
		cd rust/ffi; \
		cargo build --release; \
	else echo "Skipping iOS compilation on $$(uname)" ; \
	fi

ios: rust/target/universal/release/liblexibook_ffi.a

rust/target/universal/release/liblexibook_ffi.a: $(SOURCES)
	@if [ $$(uname) == "Darwin" ] ; then \
		cd rust/ffi; \
		cargo lipo --release; \
	else echo "Skipping iOS compilation on $$(uname)" ; \
	fi

android: rust/target/aarch64-linux-android/release/liblexibook_ffi.so rust/target/armv7-linux-androidabi/release/liblexibook_ffi.so rust/target/i686-linux-android/release/liblexibook_ffi.so

rust/target/aarch64-linux-android/release/liblexibook_ffi.so: $(SOURCES) 
	cd rust/ffi; \
	cargo build --target aarch64-linux-android --release

rust/target/armv7-linux-androidabi/release/liblexibook_ffi.so: $(SOURCES) 
	cd rust/ffi; \
	cargo build --target armv7-linux-androideabi --release


rust/target/i686-linux-android/release/liblexibook_ffi.so: $(SOURCES) 
	cd rust/ffi; \
	cargo build --target i686-linux-android --release

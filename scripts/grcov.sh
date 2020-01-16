#!/usr/bin/env bash
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads"
rm -rf target/debug/
rustup run nightly cargo test
zip -0 ccov.zip `find . -name "lexibook*.gc*" -print`
grcov ccov.zip --llvm --branch --ignore-not-existing --ignore "/*" -t html -o target/coverage
rm -f ccov.zip
open target/coverage/index.html


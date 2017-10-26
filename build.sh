#!/bin/sh
cargo build --release
strip --strip-all target/release/midigrep
mv ./target/release/midigrep ./bin 

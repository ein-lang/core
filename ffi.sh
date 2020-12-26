#!/bin/sh

set -e

cd ffi
cargo build --release
echo target/release/libprelude.a

#!/bin/bash

set -e -u -o pipefail

cd "$(dirname "$0")"

echo
echo
echo cleanup
cargo clean
make -C c-mylib clean

echo
echo
echo compiling C library
make -C c-mylib

echo
echo
echo compiling Rust libraries
cargo build

echo
echo
echo testing -sys package
cargo test --package mylib-sys

echo
echo
echo testing -rs package
cargo test --package mylib-rs

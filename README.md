# Rust wrapping C

This project demonstrates the recommended pattern of building Rust wrappers
around a C library:
- `c-mylib`: a C library that builds into a static archive at `c-mylib/out/libmylib.a`
- `mylib-sys`: a minimal Rust FFI crate to expose that C library as unsafe Rust
- `mylib-rs`: a safe Rust library that depends on `mylib-sys` and makes it usable by other Rust code


To see a basic build and tests passing:
```
./test.sh
```

But Link Time Optimization (LTO) of the C library breaks things in a strange way:
```
LDFLAGS=-flto ./test.sh
```
In this case the tests of `mylib-sys` still pass, but the tests of `mylib-rs`
fail to link.

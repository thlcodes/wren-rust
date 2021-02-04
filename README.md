# wren-rust 
[![Crates.io](https://img.shields.io/crates/v/wren.svg)](https://crates.io/crates/wren)
[![Documentation](https://docs.rs/wren/badge.svg)](https://docs.rs/wren)

Rust bindings to the [Wren scripting language](http://wren.io) API.

# Safety
Wren doesn't do any kind of validation outside of a few assertions in debug builds. This means it's very easy to get Undefined Behavior in release builds if you're not careful (especially when processing arbitrary scripts).

Some functions are still not checked regarding upgraded C API.

# Todo

* Support Wren standard lib (option headers)

## References

* Another [Wren lib in Rust](https://github.com/Laegluin/wren-sys)
* Wrap some unsafe functions so you don't hit by undefined behaviour.

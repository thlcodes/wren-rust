# wren-rust [![Crates.io](https://img.shields.io/crates/v/wren.svg)](https://crates.io/crates/wren) [![Documentation](https://docs.rs/wren/badge.svg)](https://docs.rs/wren)
Rust bindings to the [Wren scripting language](http://wren.io) API.

Crate documentation is somewhat lacking at the moment.
For complete documentation on each type and function, refer to `wren.h` in the [official Wren repository](http://github.com/munificent/wren).

Wren is still under heavy development. 
I'll do my best to keep these bindings up-to-date as new features are added.
If you notice a missing feature, feel free to create an issue or pull request.

# Safety
Wren doesn't do any kind of validation outside of a few assertions in debug builds. 
This means it's very easy to get Undefined Behavior in release builds if you're not careful (especially when processing arbitrary scripts).

Most functions in this crate include additional safety features to help avoid these problems. In particular:

1. Functions that retrieve slot values will perform type checking and return an Option.
2. `wrenEnsureSlots` is called automatically when setting slot values.
3. Most functions validate their parameters before calling Wren. 

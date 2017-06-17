# wren-rust
Rust bindings to the [Wren scripting language](http://wren.io) API.

For complete documentation on each type and function, refer to `wren.h` in the [official Wren repository](http://github.com/munificent/wren).

Wren is still under heavy development. 
I'll do my best to keep these bindings up-to-date as new features are added.
If you notice a missing feature, feel free to create an issue or pull request.

# Safety
There currently aren't any safeguards to protect you from doing something potentially unsafe. 

In debug builds, Wren makes assertions that prevent undefined behavior, but these are disabled in release builds.


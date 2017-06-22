//! Bindings to the Wren scripting language API.
//!
//! For complete documentation on each type and function, refer to `wren.h` in the official Wren repository.

extern crate libc;
extern crate wren_sys as ffi;

#[macro_use]
pub mod macros;
mod vm;

/// Typedef for a raw pointer.
pub type Pointer = *mut libc::c_void;

pub use ffi::WrenErrorType as ErrorType;
pub use ffi::WrenInterpretResult as InterpretResult;
pub use ffi::WrenType as Type;

pub use ffi::WrenReallocateFn as ReallocateFn;
pub use ffi::WrenForeignMethodFn as ForeignMethodFn;
pub use ffi::WrenFinalizerFn as FinalizerFn;
pub use ffi::WrenLoadModuleFn as LoadModuleFn;
pub use ffi::WrenBindForeignMethodFn as BindForeignMethodFn;
pub use ffi::WrenBindForeignClassFn as BindForeignClassFn;
pub use ffi::WrenWriteFn as WriteFn;
pub use ffi::WrenErrorFn as ErrorFn;

pub use self::vm::Configuration;
pub use self::vm::ForeignClassMethods;
pub use self::vm::Handle;
pub use self::vm::VM;

#[cfg(test)]
mod tests;

use ffi;
use libc::*;
use std::ffi::{CStr};
use std::mem;
use std::ptr;
use ErrorType;
use Pointer;
use VM;

/// Wrap a `Fn(Pointer, usize) -> Pointer` as an ffi-suitable `ReallocateFn`.
#[macro_export]
macro_rules! wren_reallocate_fn {
    ($f:path) => {
        $crate::macros::_wrap_reallocate_fn($f)
    };
}

/// Wrap a `Fn(&mut VM)` as an ffi-suitable `ForeignMethodFn`.
#[macro_export]
macro_rules! wren_foreign_method_fn {
    ($f:path) => {
        $crate::macros::_wrap_foreign_method_fn($f)
    };
}

/// Wrap a `Fn(Pointer)` as an ffi-suitable `FinalizerFn`.
#[macro_export]
macro_rules! wren_finalizer_fn {
    ($f:path) => {
        $crate::macros::_wrap_finalizer_fn($f)
    };
}

/// Wrap a `Fn(&mut VM, &str, &str, bool, &str) -> ForeignMethodFn` as an ffi-suitable `BindForeignMethodFn`.
#[macro_export]
macro_rules! wren_bind_foreign_method_fn {
    ($f:path) => {
        $crate::macros::_wrap_bind_foreign_method_fn($f)
    };
}

/// Wrap a `Fn(&mut VM, &str, &str) -> ForeignClassMethods` as an ffi-suitable `BindForeignClassFn`.
#[macro_export]
macro_rules! wren_bind_foreign_class_fn {
    ($f:path) => {
        $crate::macros::_wrap_bind_foreign_class_fn($f)
    };
}

/// Wrap a `Fn(&mut VM, &str)` as an ffi-suitable `WriteFn`.
#[macro_export]
macro_rules! wren_write_fn {
    ($f:path) => {
        $crate::macros::_wrap_write_fn($f)
    };
}

/// Wrap a `Fn(&mut VM, ErrorType, &str, i32, &str)` as an ffi-suitable `ErrorFn`.
#[macro_export]
macro_rules! wren_error_fn {
    ($f:path) => {
        $crate::macros::_wrap_error_fn($f)
    };
}

#[doc(hidden)]
#[inline]
pub fn _default_realloc(memory: *mut c_void, new_size: usize) -> *mut c_void {
    if new_size == 0 {
        unsafe { free(memory) };
        return ptr::null_mut();
    }
    unsafe { realloc(memory, new_size) }
}

#[doc(hidden)]
#[inline]
fn _assert_size<F>() {
    let size = mem::size_of::<F>();
    assert!(size == 0, "Wrapped closures must be zero-sized");
}

#[doc(hidden)]
#[inline]
pub fn _wrap_reallocate_fn<F: Fn(Pointer, usize) -> Pointer>(_: F) -> ::ReallocateFn {
    unsafe extern "C" fn f<F: Fn(Pointer, usize) -> Pointer>(
        _memory: *mut c_void,
        new_size: size_t,
        data: *mut c_void,
    ) -> *mut c_void {
        mem::transmute::<&(), &F>(&())(data, new_size)
    }
    _assert_size::<F>();
    Some(f::<F>)
}

#[doc(hidden)]
#[inline]
pub fn _wrap_foreign_method_fn<F: Fn(&mut VM)>(_: F) -> ::ForeignMethodFn {
    unsafe extern "C" fn f<F: Fn(&mut VM)>(vm: *mut ffi::WrenVM) {
        mem::transmute::<&(), &F>(&())(&mut VM::from_ptr(vm));
    }
    _assert_size::<F>();
    Some(f::<F>)
}

#[doc(hidden)]
#[inline]
pub fn _wrap_finalizer_fn<F: Fn(Pointer)>(_: F) -> ::FinalizerFn {
    unsafe extern "C" fn f<F: Fn(Pointer)>(data: *mut c_void) {
        mem::transmute::<&(), &F>(&())(data)
    }
    _assert_size::<F>();
    Some(f::<F>)
}

#[doc(hidden)]
#[inline]
pub fn _wrap_bind_foreign_method_fn<F: Fn(&mut VM, &str, &str, bool, &str) -> ::ForeignMethodFn>(
    _: F,
) -> ::BindForeignMethodFn {
    unsafe extern "C" fn f<F: Fn(&mut VM, &str, &str, bool, &str) -> ::ForeignMethodFn>(
        vm: *mut ffi::WrenVM,
        module: *const c_char,
        class_name: *const c_char,
        is_static: bool,
        signature: *const c_char,
    ) -> ::ForeignMethodFn {
        let mut vm = VM::from_ptr(vm);
        let module = CStr::from_ptr(module).to_str().unwrap();
        let class_name = CStr::from_ptr(class_name).to_str().unwrap();
        let signature = CStr::from_ptr(signature).to_str().unwrap();
        mem::transmute::<&(), &F>(&())(&mut vm, module, class_name, is_static, signature)
    }
    _assert_size::<F>();
    Some(f::<F>)
}

#[doc(hidden)]
#[inline]
pub fn _wrap_bind_foreign_class_fn<F: Fn(&mut VM, &str, &str) -> ::ForeignClassMethods>(
    _: F,
) -> ::BindForeignClassFn {
    unsafe extern "C" fn f<F: Fn(&mut VM, &str, &str) -> ::ForeignClassMethods>(
        vm: *mut ffi::WrenVM,
        module: *const c_char,
        class_name: *const c_char,
    ) -> ffi::WrenForeignClassMethods {
        let mut vm = VM::from_ptr(vm);
        let module = CStr::from_ptr(module).to_str().unwrap();
        let class_name = CStr::from_ptr(class_name).to_str().unwrap();
        mem::transmute::<&(), &F>(&())(&mut vm, module, class_name).get()
    }
    _assert_size::<F>();
    Some(f::<F>)
}

#[doc(hidden)]
#[inline]
pub fn _wrap_write_fn<F: Fn(&mut VM, &str)>(_: F) -> ::WriteFn {
    unsafe extern "C" fn f<F: Fn(&mut VM, &str)>(vm: *mut ffi::WrenVM, text: *const c_char) {
        mem::transmute::<&(), &F>(&())(
            &mut VM::from_ptr(vm),
            CStr::from_ptr(text).to_str().unwrap(),
        );
    }
    _assert_size::<F>();
    Some(f::<F>)
}

#[doc(hidden)]
#[inline]
pub fn _wrap_error_fn<F: Fn(&mut VM, ErrorType, &str, i32, &str)>(_: F) -> ::ErrorFn {
    unsafe extern "C" fn f<F: Fn(&mut VM, ErrorType, &str, i32, &str)>(
        vm: *mut ffi::WrenVM,
        _type: ffi::WrenErrorType,
        module: *const c_char,
        line: c_int,
        message: *const c_char,
    ) {
        let mut vm = VM::from_ptr(vm);
        let module = if module == ptr::null() {
            ""
        } else {
            CStr::from_ptr(module).to_str().unwrap()
        };
        let message = CStr::from_ptr(message).to_str().unwrap();
        mem::transmute::<&(), &F>(&())(&mut vm, _type, module, line, message);
    }
    _assert_size::<F>();
    Some(f::<F>)
}

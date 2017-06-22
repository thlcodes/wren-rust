use std::mem;
use std::ptr;
use std::ffi::{CStr, CString};
use libc::*;
use ffi;
use VM;
use Pointer;
use ErrorType;

/// Wrap a `Fn(Pointer, usize) -> Pointer` as an ffi-suitable `ReallocateFn`.
#[macro_export]
macro_rules! wren_reallocate_fn{ ($f:path) => { $crate::macros::_wrap_reallocate_fn($f) } }

/// Wrap a `Fn(&mut VM)` as an ffi-suitable `ForeignMethodFn`.
#[macro_export]
macro_rules! wren_foreign_method_fn{ ($f:path) => { $crate::macros::_wrap_foreign_method_fn($f) } }

/// Wrap a `Fn(Pointer)` as an ffi-suitable `FinalizerFn`.
#[macro_export]
macro_rules! wren_finalizer_fn{ ($f:path) => { $crate::macros::_wrap_finalizer_fn($f) } }

/// Wrap a `Fn(&mut VM, &str) -> Option<String>` as an ffi-suitable `LoadModuleFn`.
///
/// Note: If a custom allocator is being used, it must be passed to this macro as well.
/// This isn't required when using the default allocator.
#[macro_export]
macro_rules! wren_load_module_fn{
    ($f:path) => { $crate::macros::_wrap_load_module_fn($f, $crate::macros::_default_realloc) };
    ($f:path, $alloc:path) => { $crate::macros::_wrap_load_module_fn($f, $alloc) };
}

/// Wrap a `Fn(&mut VM, &str, &str, bool, &str) -> ForeignMethodFn` as an ffi-suitable `BindForeignMethodFn`.
#[macro_export]
macro_rules! wren_bind_foreign_method_fn{ ($f:path) => { $crate::macros::_wrap_bind_foreign_method_fn($f) } }

/// Wrap a `Fn(&mut VM, &str, &str) -> ForeignClassMethods` as an ffi-suitable `BindForeignClassFn`.
#[macro_export]
macro_rules! wren_bind_foreign_class_fn{ ($f:path) => { $crate::macros::_wrap_bind_foreign_class_fn($f) } }

/// Wrap a `Fn(&mut VM, &str)` as an ffi-suitable `WriteFn`.
#[macro_export]
macro_rules! wren_write_fn{ ($f:path) => { $crate::macros::_wrap_write_fn($f) } }

/// Wrap a `Fn(&mut VM, ErrorType, &str, i32, &str)` as an ffi-suitable `ErrorFn`.
#[macro_export]
macro_rules! wren_error_fn{ ($f:path) => { $crate::macros::_wrap_error_fn($f) } }



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
    unsafe extern "C" fn f<F: Fn(Pointer, usize) -> Pointer>(data: *mut c_void,
                                                             new_size: size_t)
                                                             -> *mut c_void {
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
pub fn _wrap_load_module_fn<F: Fn(&mut VM, &str) -> Option<String>,
                            Alloc: Fn(*mut c_void, size_t) -> *mut c_void>
    (_: F,
     _: Alloc)
     -> ::LoadModuleFn {
    unsafe extern "C" fn f<F: Fn(&mut VM, &str) -> Option<String>,
                           Alloc: Fn(*mut c_void, size_t) -> *mut c_void>
        (vm: *mut ffi::WrenVM,
         name: *const c_char)
         -> *mut c_char {

        let mut vm = VM::from_ptr(vm);
        let name = CStr::from_ptr(name).to_str().unwrap();
        let source = mem::transmute::<&(), &F>(&())(&mut vm, name);
        if let Some(source) = source {
            let len = source.len() + 1; // One extra byte for the null terminator.
            let source_cstr = CString::new(source).unwrap();
            let buffer = mem::transmute::<&(), &Alloc>(&())(ptr::null_mut(), len);
            memcpy(buffer, source_cstr.as_ptr() as *mut c_void, len);
            buffer as *mut c_char
        } else {
            ptr::null_mut()
        }
    }
    _assert_size::<F>();
    Some(f::<F, Alloc>)
}

#[doc(hidden)]
#[inline]
pub fn _wrap_bind_foreign_method_fn<F: Fn(&mut VM, &str, &str, bool, &str) -> ::ForeignMethodFn>
    (_: F)
     -> ::BindForeignMethodFn {
    unsafe extern "C" fn f<F: Fn(&mut VM, &str, &str, bool, &str) -> ::ForeignMethodFn>
        (vm: *mut ffi::WrenVM,
         module: *const c_char,
         class_name: *const c_char,
         is_static: c_int,
         signature: *const c_char)
         -> ::ForeignMethodFn {
        let mut vm = VM::from_ptr(vm);
        let module = CStr::from_ptr(module).to_str().unwrap();
        let class_name = CStr::from_ptr(class_name).to_str().unwrap();
        let is_static = is_static != 0;
        let signature = CStr::from_ptr(signature).to_str().unwrap();
        mem::transmute::<&(), &F>(&())(&mut vm, module, class_name, is_static, signature)
    }
    _assert_size::<F>();
    Some(f::<F>)
}

#[doc(hidden)]
#[inline]
pub fn _wrap_bind_foreign_class_fn<F: Fn(&mut VM, &str, &str) -> ::ForeignClassMethods>
    (_: F)
     -> ::BindForeignClassFn {
    unsafe extern "C" fn f<F: Fn(&mut VM, &str, &str) -> ::ForeignClassMethods>
        (vm: *mut ffi::WrenVM,
         module: *const c_char,
         class_name: *const c_char)
         -> ffi::WrenForeignClassMethods {
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
        mem::transmute::<&(), &F>(&())(&mut VM::from_ptr(vm),
                                       CStr::from_ptr(text).to_str().unwrap());
    }
    _assert_size::<F>();
    Some(f::<F>)
}

#[doc(hidden)]
#[inline]
pub fn _wrap_error_fn<F: Fn(&mut VM, ErrorType, &str, i32, &str)>(_: F) -> ::ErrorFn {
    unsafe extern "C" fn f<F: Fn(&mut VM, ErrorType, &str, i32, &str)>(vm: *mut ffi::WrenVM,
                                                                       _type: ffi::WrenErrorType,
                                                                       module: *const c_char,
                                                                       line: c_int,
                                                                       message: *const c_char) {
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

#![allow(non_camel_case_types, non_snake_case, dead_code)]
#![allow(improper_ctypes)]

extern crate libc;
use libc::{c_char, c_int, c_void, size_t};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WrenVM {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WrenHandle {}

pub type WrenReallocateFn = Option<
    unsafe extern "C" fn(
        memory: *mut c_void,
        newSize: size_t,
        userData: *mut c_void,
    ) -> *mut c_void,
>;
pub type WrenForeignMethodFn = Option<unsafe extern "C" fn(vm: *mut WrenVM)>;
pub type WrenFinalizerFn = Option<unsafe extern "C" fn(data: *mut c_void)>;
pub type WrenResolveModuleFn = Option<
    unsafe extern "C" fn(
        vm: *mut WrenVM,
        importer: *const c_char,
        name: *const c_char,
    ) -> *const c_char,
>;
pub type WrenLoadModuleCompleteFn = Option<
    unsafe extern "C" fn(vm: *mut WrenVM, name: *const c_char, result: WrenLoadModuleResult),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WrenLoadModuleResult {
    pub source: *const c_char,
    pub onComplete: WrenLoadModuleCompleteFn,
    pub userData: *mut c_void,
}
pub type WrenLoadModuleFn =
    Option<unsafe extern "C" fn(vm: *mut WrenVM, name: *const c_char) -> WrenLoadModuleResult>;

pub type WrenBindForeignMethodFn = Option<
    unsafe extern "C" fn(
        vm: *mut WrenVM,
        module: *const c_char,
        className: *const c_char,
        isStatic: bool,
        signature: *const c_char,
    ) -> WrenForeignMethodFn,
>;
pub type WrenWriteFn = Option<unsafe extern "C" fn(vm: *mut WrenVM, text: *const c_char)>;

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(C)]
pub enum WrenErrorType {
    Compile,
    Runtime,
    StackTrace,
}
pub type WrenErrorFn = Option<
    unsafe extern "C" fn(
        vm: *mut WrenVM,
        type_: WrenErrorType,
        module: *const c_char,
        line: c_int,
        message: *const c_char,
    ),
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WrenForeignClassMethods {
    pub allocate: WrenForeignMethodFn,
    pub finalize: WrenFinalizerFn,
}
pub type WrenBindForeignClassFn = Option<
    unsafe extern "C" fn(
        vm: *mut WrenVM,
        module: *const c_char,
        className: *const c_char,
    ) -> WrenForeignClassMethods,
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WrenConfiguration {
    pub reallocate_fn: WrenReallocateFn,
    pub resolve_module_fn: WrenResolveModuleFn,
    pub load_module_fn: WrenLoadModuleFn,
    pub bind_foreign_method_fn: WrenBindForeignMethodFn,
    pub bind_foreign_class_fn: WrenBindForeignClassFn,
    pub write_fn: WrenWriteFn,
    pub error_fn: WrenErrorFn,
    pub initial_heap_size: size_t,
    pub min_heap_size: size_t,
    pub heap_growth_percent: c_int,
    pub user_data: *mut c_void,
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(C)]
pub enum WrenInterpretResult {
    Success,
    CompileError,
    RuntimeError,
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(C)]
pub enum WrenType {
    Bool,
    Num,
    Foreign,
    List,
    Null,
    String,
    Unknown,
}

extern "C" {
    pub fn wrenInitConfiguration(configuration: *mut WrenConfiguration);
    pub fn wrenNewVM(configuration: *mut WrenConfiguration) -> *mut WrenVM;
    pub fn wrenFreeVM(vm: *mut WrenVM);
    pub fn wrenCollectGarbage(vm: *mut WrenVM);
    pub fn wrenInterpret(vm: *mut WrenVM, source: *const c_char) -> WrenInterpretResult;
    pub fn wrenInterpretInModule(
        vm: *mut WrenVM,
        module: *const c_char,
        source: *const c_char,
    ) -> WrenInterpretResult;
    pub fn wrenMakeCallHandle(vm: *mut WrenVM, signature: *const c_char) -> *mut WrenHandle;
    pub fn wrenCall(vm: *mut WrenVM, method: *mut WrenHandle) -> WrenInterpretResult;
    pub fn wrenReleaseHandle(vm: *mut WrenVM, handle: *mut WrenHandle);
    pub fn wrenGetSlotCount(vm: *mut WrenVM) -> c_int;
    pub fn wrenEnsureSlots(vm: *mut WrenVM, numSlots: c_int);
    pub fn wrenGetSlotType(vm: *mut WrenVM, slot: c_int) -> WrenType;
    pub fn wrenGetSlotBool(vm: *mut WrenVM, slot: c_int) -> bool;
    pub fn wrenGetSlotBytes(vm: *mut WrenVM, slot: c_int, length: *mut c_int) -> *const c_char;
    pub fn wrenGetSlotDouble(vm: *mut WrenVM, slot: c_int) -> f64;
    pub fn wrenGetSlotForeign(vm: *mut WrenVM, slot: c_int) -> *mut c_void;
    pub fn wrenGetSlotString(vm: *mut WrenVM, slot: c_int) -> *const c_char;
    pub fn wrenGetSlotHandle(vm: *mut WrenVM, slot: c_int) -> *mut WrenHandle;
    pub fn wrenSetSlotBool(vm: *mut WrenVM, slot: c_int, value: bool);
    pub fn wrenSetSlotBytes(vm: *mut WrenVM, slot: c_int, bytes: *const c_char, length: size_t);
    pub fn wrenSetSlotDouble(vm: *mut WrenVM, slot: c_int, value: f64);
    pub fn wrenSetSlotNewForeign(
        vm: *mut WrenVM,
        slot: c_int,
        classSlot: c_int,
        size: size_t,
    ) -> *mut c_void;
    pub fn wrenSetSlotNewList(vm: *mut WrenVM, slot: c_int);
    pub fn wrenSetSlotNewMap(vm: *mut WrenVM, slot: c_int);
    pub fn wrenSetSlotNull(vm: *mut WrenVM, slot: c_int);
    pub fn wrenSetSlotString(vm: *mut WrenVM, slot: c_int, text: *const c_char);
    pub fn wrenSetSlotHandle(vm: *mut WrenVM, slot: c_int, handle: *mut WrenHandle);
    pub fn wrenGetListCount(vm: *mut WrenVM, slot: c_int) -> c_int;
    pub fn wrenGetListElement(vm: *mut WrenVM, listSlot: c_int, index: c_int, elementSlot: c_int);
    pub fn wrenSetListElement(vm: *mut WrenVM, listSlot: c_int, index: c_int, elementSlot: c_int);
    pub fn wrenInsertInList(vm: *mut WrenVM, listSlot: c_int, index: c_int, elementSlot: c_int);
    pub fn wrenGetMapCount(vm: *mut WrenVM, slot: c_int) -> c_int;
    pub fn wrenGetMapContainsKey(vm: *mut WrenVM, mapSlot: c_int, keySlot: c_int) -> bool;
    pub fn wrenGetMapValue(vm: *mut WrenVM, mapSlot: c_int, keySlot: c_int, valueSlot: c_int);
    pub fn wrenSetMapValue(vm: *mut WrenVM, mapSlot: c_int, keySlot: c_int, valueSlot: c_int);
    pub fn wrenRemoveMapValue(
        vm: *mut WrenVM,
        mapSlot: c_int,
        keySlot: c_int,
        removedValueSlot: c_int,
    );
    pub fn wrenGetVariable(
        vm: *mut WrenVM,
        module: *const c_char,
        name: *const c_char,
        slot: c_int,
    );
    pub fn wrenHasVariable(vm: *mut WrenVM, module: *const c_char, name: *const c_char) -> bool;
    pub fn wrenHasModule(vm: *mut WrenVM, module: *const c_char) -> bool;
    pub fn wrenAbortFiber(vm: *mut WrenVM, slot: c_int);
    pub fn wrenGetUserData(vm: *mut WrenVM) -> *mut c_void;
    pub fn wrenSetUserData(vm: *mut WrenVM, userData: *mut c_void);
}

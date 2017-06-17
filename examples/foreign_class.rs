#[macro_use]
extern crate wren;
#[macro_use]
extern crate lazy_static;

mod vector;

use std::mem;
use std::ptr;
use std::collections::HashMap;
use vector::Vec3;
use wren::{Pointer, VM, Configuration, ForeignMethodFn, ForeignClassMethods};

lazy_static! {
    static ref FOREIGN_METHODS: HashMap<&'static str, ForeignMethodFn> = {
        let mut map = HashMap::new();
        map.insert("vectorVec3norm()", wren_foreign_method_fn!(vec3_norm));
        map.insert("vectorVec3dot(_)", wren_foreign_method_fn!(vec3_dot));
        map.insert("vectorVec3cross(_)", wren_foreign_method_fn!(vec3_cross));
        map.insert("vectorVec3x", wren_foreign_method_fn!(vec3_get_x));
        map.insert("vectorVec3x=(_)", wren_foreign_method_fn!(vec3_set_x));
        map.insert("vectorVec3y", wren_foreign_method_fn!(vec3_get_y));
        map.insert("vectorVec3y=(_)", wren_foreign_method_fn!(vec3_set_y));
        map.insert("vectorVec3z", wren_foreign_method_fn!(vec3_get_z));
        map.insert("vectorVec3z=(_)", wren_foreign_method_fn!(vec3_set_z));
        map
    };
}

lazy_static! {
    static ref FOREIGN_CLASSES: HashMap<&'static str, ForeignClassMethods> = {
        let mut map = HashMap::new();
        map.insert("vectorVec3", ForeignClassMethods::new(wren_foreign_method_fn!(vec3_allocate),
                                                          wren_finalizer_fn!(vec3_finalize)));
        map
    };
}

fn vec3_allocate(vm: &mut VM) {
    let ptr = vm.set_slot_new_foreign(0, 0, mem::size_of::<Vec3>()) as *mut Vec3;
    let vec = Vec3::new(vm.get_slot_double(1),
                        vm.get_slot_double(2),
                        vm.get_slot_double(3));
    unsafe { ptr::write(ptr, vec) };
}

fn vec3_finalize(_: Pointer) {
    // do nothing.
}

fn vec3_norm(vm: &mut VM) {
    let vec = vm.get_slot_foreign(0) as *mut Vec3;
    let result = unsafe { (*vec).norm() };
    vm.set_slot_double(0, result);
}

fn vec3_dot(vm: &mut VM) {
    let lhs = vm.get_slot_foreign(0) as *mut Vec3;
    let rhs = vm.get_slot_foreign(1) as *mut Vec3;
    let result = unsafe { (*lhs).dot(&*rhs) };
    vm.set_slot_double(0, result);
}

fn vec3_cross(vm: &mut VM) {
    let lhs = vm.get_slot_foreign(0) as *mut Vec3;
    let rhs = vm.get_slot_foreign(1) as *mut Vec3;
    let result = unsafe { (*lhs).cross(&*rhs) };

    // This currently causes Wren to fail an assertion check.
    // It seems like there's currently no way to create a foreign class object directly from the API.
    let result_ptr = vm.set_slot_new_foreign(0, 0, mem::size_of::<Vec3>()) as *mut Vec3;
    unsafe { ptr::write(result_ptr, result) };
}

fn vec3_get_x(vm: &mut VM) {
    let vec = vm.get_slot_foreign(0) as *mut Vec3;
    let x = unsafe { (*vec).x };
    vm.set_slot_double(0, x);
}

fn vec3_set_x(vm: &mut VM) {
    let vec = vm.get_slot_foreign(0) as *mut Vec3;
    let x = vm.get_slot_double(1);
    unsafe { (*vec).x = x };
}

fn vec3_get_y(vm: &mut VM) {
    let vec = vm.get_slot_foreign(0) as *mut Vec3;
    let y = unsafe { (*vec).y };
    vm.set_slot_double(0, y);
}

fn vec3_set_y(vm: &mut VM) {
    let vec = vm.get_slot_foreign(0) as *mut Vec3;
    let y = vm.get_slot_double(1);
    unsafe { (*vec).y = y };
}

fn vec3_get_z(vm: &mut VM) {
    let vec = vm.get_slot_foreign(0) as *mut Vec3;
    let z = unsafe { (*vec).z };
    vm.set_slot_double(0, z);
}

fn vec3_set_z(vm: &mut VM) {
    let vec = vm.get_slot_foreign(0) as *mut Vec3;
    let z = vm.get_slot_double(1);
    unsafe { (*vec).z = z };
}

fn bind_method(_: &mut VM,
               module: &str,
               class_name: &str,
               is_static: bool,
               signature: &str)
               -> ForeignMethodFn {
    let full_signature = format!("{}{}{}{}",
                                 module,
                                 class_name,
                                 signature,
                                 if is_static { "s" } else { "" });
    *FOREIGN_METHODS.get::<str>(&full_signature).unwrap_or(&None)
}

fn bind_class(_: &mut VM, module: &str, class_name: &str) -> ForeignClassMethods {
    let full_signature = format!("{}{}", module, class_name);
    let methods = FOREIGN_CLASSES.get::<str>(&full_signature);
    if let Some(methods) = methods {
        return *methods;
    }
    panic!("Failed to bind foreign class");
}

fn load_module(_: &mut VM, name: &str) -> Option<String> {
    use std::path::Path;
    use std::fs::File;
    use std::io::Read;

    let mut path = Path::new("examples").join(&name);
    path.set_extension("wren");
    let mut buffer = String::new();
    if File::open(path)
           .map(|mut f| f.read_to_string(&mut buffer))
           .is_ok() {
        Some(buffer)
    } else {
        None
    }
}

fn main() {
    let source = r#"
import "vector" for Vec3
var vec = Vec3.new(1.0, 2.0, 3.0)
var vec2 = Vec3.new(2.0, 4.0, 6.0)
System.print("norm = %(vec.norm())")
System.print("dot = %(vec.dot(vec2))")
"#;
    let mut cfg = Configuration::new();
    cfg.set_bind_foreign_method_fn(wren_bind_foreign_method_fn!(bind_method));
    cfg.set_bind_foreign_class_fn(wren_bind_foreign_class_fn!(bind_class));
    cfg.set_load_module_fn(wren_load_module_fn!(load_module));
    let mut vm = VM::new(cfg);
    vm.interpret(source);
}

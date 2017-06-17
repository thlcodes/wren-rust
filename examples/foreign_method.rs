#[macro_use]
extern crate wren_rust;
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use wren_rust::{VM, Configuration, ForeignMethodFn};

lazy_static!  {
    static ref FOREIGN_METHODS: HashMap<&'static str, ForeignMethodFn> = {
        let mut map = HashMap::new();
        map.insert("mainMathsin(_)s", wren_foreign_method_fn!(sin));
        map.insert("mainMathcos(_)s", wren_foreign_method_fn!(cos));
        map
    };
}

fn sin(vm: &mut VM) {
    let value = vm.get_slot_double(1);
    vm.set_slot_double(0, value.sin());
}

fn cos(vm: &mut VM) {
    let value = vm.get_slot_double(1);
    vm.set_slot_double(0, value.cos());
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

fn main() {
    let source = r#"
class Math {
    foreign static sin(num)
    foreign static cos(num)
}
System.print("sin(1rad) = %(Math.sin(1))")
System.print("cos(1rad) = %(Math.cos(1))")
"#;
    let mut cfg = Configuration::new();
    cfg.set_bind_foreign_method_fn(wren_bind_foreign_method_fn!(bind_method));
    let mut vm = VM::new(cfg);
    vm.interpret(source);
}

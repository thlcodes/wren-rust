extern crate wren;
use wren::{Configuration, VM};

fn main() {
    let cfg = Configuration::new();
    let mut vm = VM::new(cfg);

    vm.interpret_file("examples/scripts/test.wren").unwrap();

    vm.ensure_slots(1);
    vm.get_variable("main", "Test", 0);
    let class_handle = vm.get_slot_handle(0);

    let zero = vm.make_call_handle("zero()");
    let one = vm.make_call_handle("one(_)");
    let two = vm.make_call_handle("two(_,_)");

    vm.ensure_slots(1);
    vm.set_slot_handle(0, class_handle);
    vm.call(zero);

    vm.ensure_slots(2);
    vm.set_slot_handle(0, class_handle);
    vm.set_slot_double(1, 1.0);
    vm.call(one);

    vm.ensure_slots(3);
    vm.set_slot_handle(0, class_handle);
    vm.set_slot_double(1, 1.0);
    vm.set_slot_double(2, 2.0);
    vm.call(two);

    vm.release_handle(class_handle);
    vm.release_handle(zero);
    vm.release_handle(one);
    vm.release_handle(two);
}
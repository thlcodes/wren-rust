use {Configuration, VM};

#[test]
fn list() {
    let mut vm = VM::new(Configuration::new());
    vm.set_slot_new_list(0);
    for i in 1..4i32 {
        vm.set_slot_double(1, i as f64);
        vm.insert_in_list(0, -1, 1);
    }
    assert_eq!(vm.get_list_count(0), 3);

    vm.set_slot_double(1, 4.0);
    vm.insert_in_list(0, 3, 1);
    assert_eq!(vm.get_list_count(0), 4);

    vm.get_list_element(0, 2, 1);
    assert_eq!(vm.get_slot_double(1).unwrap(), 3.0);
}

#[test]
#[should_panic]
fn list_out_of_bounds() {
    let mut vm = VM::new(Configuration::new());
    vm.set_slot_new_list(0);
    vm.set_slot_double(1, 1.0);
    vm.insert_in_list(0, 1, 1);
}

#[test]
#[should_panic]
fn not_list() {
    let mut vm = VM::new(Configuration::new());
    vm.set_slot_double(0, 0.0);
    vm.set_slot_double(1, 1.0);
    vm.insert_in_list(0, -1, 1);
}

#[test]
#[should_panic]
fn list_no_element() {
    let mut vm = VM::new(Configuration::new());
    vm.set_slot_new_list(0);
    vm.insert_in_list(0, -1, 1);
}

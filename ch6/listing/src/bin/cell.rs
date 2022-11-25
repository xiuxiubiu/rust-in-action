use std::cell::Cell;

#[allow(dead_code)]
struct SomeStruct {
    regular_field: u8,
    special_filed: Cell<u8>,
}

fn main() {
    let my_struct = SomeStruct {
        regular_field: 0,
        special_filed: Cell::new(1),
    };

    let new_value = 100;

    my_struct.special_filed.set(new_value);
    assert_eq!(my_struct.special_filed.get(), new_value);
}

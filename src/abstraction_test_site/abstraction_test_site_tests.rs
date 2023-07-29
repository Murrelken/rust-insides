use std::fmt::Debug;
use super:: {one_struct::*, another_struct::*, struct_with_item_trait::*};

pub fn test_abstract_call() {
    let one_struct = OneStruct::new(5i32);
    let another_struct = AnotherStruct::new("25");

    println!("{:?}", one_struct.get_item());

    print_item_from_base_struct(&one_struct);
    print_item_from_base_struct(&another_struct);
}

fn print_item_from_base_struct<T: Copy + Debug>(struct_with_item: &(impl StructWithItem<T> + Debug)) {
    let item = struct_with_item.get_item();
    println!("{:?}", item);
    println!("{:?}", struct_with_item);
}
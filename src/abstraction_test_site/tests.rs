use super::{another_struct::*, one_struct::*, struct_with_item_trait::*};
use std::fmt::Debug;

#[test]
fn test_abstract_call() {
    let one_struct = OneStruct::new(5i32);
    let another_struct = AnotherStruct::new("25");

    let first_struct_item = get_item_from_base_struct(&one_struct);
    assert_eq!(first_struct_item, 5);

    let second_struct_item = get_item_from_base_struct(&another_struct);
    assert_eq!(second_struct_item, "25");
}

fn get_item_from_base_struct<T1: Copy>(struct_with_item: &(impl StructWithItem<T1> + Debug)) -> T1 {
    struct_with_item.get_item()
}

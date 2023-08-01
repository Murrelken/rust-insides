use super::struct_with_item_trait::*;

#[derive(Debug, Copy, Clone)]
pub struct AnotherStruct<T: Copy> {
    item: T,
}

impl<T: Copy> AnotherStruct<T> {
    pub fn new(item: T) -> AnotherStruct<T> {
        AnotherStruct { item }
    }
}

impl<T: Copy> StructWithItem<T> for AnotherStruct<T> {
    fn get_item(&self) -> T {
        self.item
    }
}

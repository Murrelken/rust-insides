use super::struct_with_item_trait::*;

#[derive(Debug, Copy, Clone)]
pub struct OneStruct<T: Copy> {
    item: T,
}

impl<T: Copy> OneStruct<T> {
    pub fn new(item: T) -> OneStruct<T> {
        OneStruct { item }
    }
}

impl<T: Copy> StructWithItem<T> for OneStruct<T> {
    fn get_item(&self) -> T {
        self.item
    }
}

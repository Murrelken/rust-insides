use super::struct_with_item_trait::*;

#[derive(Debug, Copy, Clone)]
pub struct OneStruct<T: Copy> {
    item: T
}

impl<T: Copy> OneStruct<T> {
    pub fn new(item: T) -> OneStruct<T> {
        OneStruct { item }
    }

    // pub fn get_item(&self) -> i32 {
    //     -1
    // }
}

impl<T: Copy> StructWithItem<T> for OneStruct<T> {
    fn get_item(&self) -> T {
        self.item
    }
}

impl OneStruct<i64> {
    pub fn get_item(&self) -> i64 {
        -64
    }
}

impl OneStruct<i32> {
    pub fn get_item(&self) -> i32 {
        -32
    }
}

// impl<OtherT: Copy> OneStruct<OtherT> {
//     fn get_item(&self) -> OtherT {
//         &self.item
//     }
// }

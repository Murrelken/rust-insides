
pub trait StructWithItem<T: Copy> {
    fn get_item(&self) -> T;
}

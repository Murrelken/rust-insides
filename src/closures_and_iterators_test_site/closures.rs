/// Actual return type is ```fn(&i32) -> bool```, and it implements ```Fn(&i32) -> bool```
///
/// Returning ```is_even``` in it's current implementation is equal to returning ```|x| x % 2 == 0```
pub fn get_test_is_even_fn() -> impl Fn(&i32) -> bool {
    is_even
}

fn is_even(x: &i32) -> bool {
    x % 2 == 0
}

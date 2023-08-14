/// Performs partial application to the ```is_modulo_zero```, and returns partial function.
pub fn get_test_is_even_fn() -> impl Fn(&i32) -> bool {
    get_is_modulo_zero_partial_by_divider(&2)
}

/// This function creates a new closure and returns it.
/// Therefore the return type is no longer ```fn(..) -> ..```, but ```Fn(..) -> ..```
///
/// Specifically, it allows to perform partial application to the ```is_modulo_zero``` function.
///
/// A closure is something that captures any value from outer scope.
/// For example, something like ```|x: &i32, d: &i32| x % d == 0```
/// isn't a closure to the compiler, but rather a function pointer - ```fn```
fn get_is_modulo_zero_partial_by_divider<'a>(divider: &'a i32) -> impl Fn(&i32) -> bool + 'a {
    move |x: &i32| is_modulo_zero(x, divider)
}

/// Type of this function is ```fn(&i32, &i32) -> bool```,
/// but it also implements ```Fn(&i32, &i32) -> bool```
fn is_modulo_zero(x: &i32, divider: &i32) -> bool {
    x % divider == 0
}

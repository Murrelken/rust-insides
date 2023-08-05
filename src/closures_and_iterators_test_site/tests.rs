use super::closures::get_test_is_even_fn;

#[test]
fn test_closure_with_iterator() {
    let is_even_closure = get_test_is_even_fn();
    let collection = (0..100).collect::<Vec<i32>>();
    let boolean_is_even_collection = collection.iter().map(is_even_closure).collect::<Vec<bool>>();

    assert_eq!(boolean_is_even_collection.len(), collection.len());

    for (val, original) in boolean_is_even_collection.iter().zip(collection) {
        assert_eq!(*val, original % 2 == 0);
    }
}

#[test]
fn test_closure_on_a_value() {
    let is_even_closure = get_test_is_even_fn();

    assert!(is_even_closure(&0));
    assert!(!is_even_closure(&1));
}

#[test]
fn test_for_each_mut() {
    // let int_collection = (0..10).collect::<Vec<i32>>();
    // let pixel = Pixel::new(0,127,255);
    //
    // let pixel = dbg!(pixel);
    //
    // for i in pixel.into_iter().collect::<PixelIntoIterator>().zip(int_collection) {
    //     dbg!(i);
    // }
    //
    // panic!();
}

#[test]
fn test_next() {

}

#[test]
fn test_nth() {

}

#[test]
fn test_for_each() {

}

#[test]
fn test_collect() {

}

#[test]
fn test_all() {

}

#[test]
fn test_any() {

}

#[test]
fn test_find() {

}

#[test]
fn test_find_map() {

}

#[test]
fn test_position() {

}

#[test]
fn test_max() {

}

#[test]
fn test_min() {

}

#[test]
fn test_sum() {

}

#[test]
fn test_eq() {

}

#[test]
fn test_() {

}

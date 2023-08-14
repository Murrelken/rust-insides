use super::closures::get_test_is_even_fn;

#[test]
fn test_closure_with_iterator() {
    let is_even_closure = get_test_is_even_fn();
    let collection = (0..100).collect::<Vec<i32>>();
    let boolean_is_even_collection = collection
        .iter()
        .map(is_even_closure)
        .collect::<Vec<bool>>();

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

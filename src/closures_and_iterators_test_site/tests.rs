use std::ops::AddAssign;
use crate::closures_and_iterators_test_site::pixel::Pixel;
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

const R: u8 = 8;
const G: u8 = 32;
const B: u8 = 128;

fn get_test_pixel() -> Pixel {
    Pixel::new(R, G, B)
}

#[test]
fn test_for_each_mut() {
    let pixel = get_test_pixel();

    for color in pixel.into_iter().for_each_mut(|x| { x.add_assign(255 - *x) }) {
        assert_eq!(color, 255);
    }
}

#[test]
fn test_next() {
    let pixel = get_test_pixel();
    let mut iter = pixel.into_iter();
    let mut prev = 2u8;

    while let Some(el) = iter.next() {
        prev *= 4;
        assert_eq!(el, prev);
    }
}

#[test]
fn test_nth() {
    let pixel = get_test_pixel();
    let mut iter = pixel.into_iter();

    assert_eq!(Some(R), iter.nth(0));
    assert_eq!(Some(G), iter.nth(1));
    assert_eq!(Some(B), iter.nth(2));
    assert_eq!(None, iter.nth(3));
}

#[test]
fn test_for_each() {
    let pixel = get_test_pixel();
    let mut prev = 2u8;
    let for_each_predicate = |el: u8| {
        prev *= 4;
        assert_eq!(el, prev);
    };

    pixel.into_iter().for_each(for_each_predicate);
}

#[test]
fn test_all() {
    let predicate = |el: u8| { el > 127 };
    let pixel_all_true = Pixel::new(128, 192, 255);
    let pixel_one_false = Pixel::new(127, 191, 255);
    let pixel_all_false = Pixel::new(0, 63, 127);

    let pixel_all_true_result = pixel_all_true.into_iter().all(predicate);
    assert!(pixel_all_true_result);
    let pixel_one_false_result = pixel_one_false.into_iter().all(predicate);
    assert!(!pixel_one_false_result);
    let pixel_all_false_result = pixel_all_false.into_iter().all(predicate);
    assert!(!pixel_all_false_result);
}

#[test]
fn test_any() {
    let predicate = |el: u8| { el > 127 };
    let pixel_all_true = Pixel::new(128, 192, 255);
    let pixel_one_false = Pixel::new(127, 191, 255);
    let pixel_all_false = Pixel::new(0, 63, 127);

    let pixel_all_true_result = pixel_all_true.into_iter().any(predicate);
    assert!(pixel_all_true_result);
    let pixel_one_false_result = pixel_one_false.into_iter().any(predicate);
    assert!(pixel_one_false_result);
    let pixel_all_false_result = pixel_all_false.into_iter().any(predicate);
    assert!(!pixel_all_false_result);
}

#[test]
fn test_find() {
    let pixel = get_test_pixel();
    let mut iter = pixel.into_iter();
    let find_result = iter.find(|x| *x == R);
    let not_found_result = iter.find(|x| *x == 0);
    assert_eq!(Some(R), find_result);
    assert_eq!(None, not_found_result);
}

#[test]
fn test_position() {
    let pixel = get_test_pixel();
    let mut iter = pixel.into_iter();
    let zero = iter.position(|x| x == R);
    let one = iter.position(|x| x == G);
    let two = iter.position(|x| x == B);
    let none = iter.position(|x| x == 0);
    assert_eq!(Some(0), zero);
    assert_eq!(Some(1), one);
    assert_eq!(Some(2), two);
    assert_eq!(None, none);
}

#[test]
fn test_max() {
    let pixel = get_test_pixel();
    let max = pixel.into_iter().max();
    assert_eq!(Some(B), max);
}

#[test]
fn test_min() {
    let pixel = get_test_pixel();
    let min = pixel.into_iter().min();
    assert_eq!(Some(R), min);
}

#[test]
fn test_sum() {
    let pixel = get_test_pixel();
    let sum = pixel.into_iter().sum::<u8>();
    assert_eq!(R + G + B, sum);
}

#[test]
fn test_eq() {
    let pixel_one = get_test_pixel();
    let pixel_two = get_test_pixel();
    assert!(pixel_one.into_iter().eq(pixel_two));
}

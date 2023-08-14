use std::fmt::Debug;
use std::iter::Sum;

#[derive(Debug, Copy, Clone)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Pixel {
        Pixel { r, g, b }
    }
}

impl IntoIterator for Pixel {
    type Item = u8;
    type IntoIter = PixelIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        PixelIntoIterator {
            item: self,
            index: 0,
        }
    }
}

impl FromIterator<u8> for PixelIntoIterator {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let mut into_iter = iter.into_iter();
        let r: Option<u8> = into_iter.next();
        let g: Option<u8> = into_iter.next();
        let b: Option<u8> = into_iter.next();
        PixelIntoIterator {
            item: Pixel::new(
                r.unwrap_or_else(|| 0),
                g.unwrap_or_else(|| 0),
                b.unwrap_or_else(|| 0),
            ),
            index: 0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PixelIntoIterator {
    item: Pixel,
    index: usize,
}

impl PixelIntoIterator {
    fn next_mut(&mut self) -> Option<&mut u8> {
        let index = self.index;
        self.index += 1;
        match index {
            0 => Some(&mut self.item.r),
            1 => Some(&mut self.item.g),
            2 => Some(&mut self.item.b),
            _ => return None,
        }
    }

    /// Executes a delegate on each element as mutable.
    ///
    /// # Examples
    /// ```
    ///    use std::ops::AddAssign;
    ///
    ///    let pixel = rust_insides::get_test_pixel();
    ///    for color in pixel.into_iter().for_each_mut(|x| { x.add_assign(255 - *x) }) {
    ///        assert_eq!(color, 255);
    ///    }
    /// ```
    pub fn for_each_mut<F>(mut self, mut f: F) -> Self
    where
        Self: Sized,
        F: FnMut(&mut u8),
    {
        while let Some(el) = self.next_mut() {
            f(el);
        }
        self.index = 0;
        self
    }
}

impl DoubleEndedIterator for PixelIntoIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => self.item.r,
            1 => self.item.g,
            2 => self.item.b,
            _ => return None,
        };
        self.index -= 1;
        Some(result)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let result = match self.index - (n - 1) {
            0 => self.item.r,
            1 => self.item.g,
            2 => self.item.b,
            _ => return None,
        };
        self.index -= n;
        Some(result)
    }
}

impl Iterator for PixelIntoIterator {
    type Item = u8;

    /// Returns iterator' current element, and increments the index.
    ///
    /// # Examples
    /// ```
    ///    use rust_insides::get_test_pixel;
    ///
    ///    let pixel = get_test_pixel();
    ///    let mut iter = pixel.into_iter();
    ///    let mut prev = 2u8;
    ///
    ///    while let Some(el) = iter.next() {
    ///        prev *= 4;
    ///        assert_eq!(el, prev);
    ///    }
    /// ```
    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => self.item.r,
            1 => self.item.g,
            2 => self.item.b,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }

    /// Actually count of values in a pixel is always 3.
    fn count(self) -> usize
    where
        Self: Sized,
    {
        3
    }

    /// Last of a pixel is always it's "b"
    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        Some(self.item.b)
    }

    /// For 0..2 returns Option of one of the "r", "g", and "b" respectively.
    /// Otherwise returns None.
    ///
    /// # Examples
    /// ```
    ///    use rust_insides::*;
    ///
    ///    let pixel = get_test_pixel();
    ///    let mut iter = pixel.into_iter();
    ///
    ///    assert_eq!(Some(R), iter.nth(0));
    ///    assert_eq!(Some(G), iter.nth(1));
    ///    assert_eq!(Some(B), iter.nth(2));
    ///    assert_eq!(None, iter.nth(3));
    /// ```
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let result = match n {
            0 => self.item.r,
            1 => self.item.g,
            2 => self.item.b,
            _ => return None,
        };
        Some(result)
    }

    /// Executes a delegate on each element as owned.
    ///
    /// # Examples
    /// ```
    ///    use rust_insides::get_test_pixel;
    ///
    ///    let pixel = get_test_pixel();
    ///    let mut prev = 2u8;
    ///    let for_each_predicate = |el: u8| {
    ///        prev *= 4;
    ///        assert_eq!(el, prev);
    ///    };
    ///
    ///    pixel.into_iter().for_each(for_each_predicate);
    /// ```
    fn for_each<F>(mut self, mut f: F)
    where
        Self: Sized,
        F: FnMut(Self::Item),
    {
        while let Some(el) = self.next() {
            f(el)
        }
    }

    /// Returns true if all "r", "g", and "b" match the predicate.
    ///
    /// # Examples
    /// ```
    ///    use rust_insides::Pixel;
    ///
    ///    let predicate = |el: u8| { el > 127 };
    ///    let pixel_all_true = Pixel::new(128, 192, 255);
    ///    let pixel_one_false = Pixel::new(127, 191, 255);
    ///    let pixel_all_false = Pixel::new(0, 63, 127);
    ///
    ///    let pixel_all_true_result = pixel_all_true.into_iter().all(predicate);
    ///    assert!(pixel_all_true_result);
    ///    let pixel_one_false_result = pixel_one_false.into_iter().all(predicate);
    ///    assert!(!pixel_one_false_result);
    ///    let pixel_all_false_result = pixel_all_false.into_iter().all(predicate);
    ///    assert!(!pixel_all_false_result);
    /// ```
    fn all<F>(&mut self, mut f: F) -> bool
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
    {
        f(self.item.r) && f(self.item.g) && f(self.item.b)
    }

    /// Returns true if any of "r", "g", and "b" matches a predicate.
    ///
    /// # Examples
    /// ```
    ///    use rust_insides::Pixel;
    ///
    ///    let predicate = |el: u8| { el > 127 };
    ///    let pixel_all_true = Pixel::new(128, 192, 255);
    ///    let pixel_one_false = Pixel::new(127, 191, 255);
    ///    let pixel_all_false = Pixel::new(0, 63, 127);
    ///
    ///    let pixel_all_true_result = pixel_all_true.into_iter().any(predicate);
    ///    assert!(pixel_all_true_result);
    ///    let pixel_one_false_result = pixel_one_false.into_iter().any(predicate);
    ///    assert!(pixel_one_false_result);
    ///    let pixel_all_false_result = pixel_all_false.into_iter().any(predicate);
    ///    assert!(!pixel_all_false_result);
    /// ```
    fn any<F>(&mut self, mut f: F) -> bool
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
    {
        f(self.item.r) || f(self.item.g) || f(self.item.b)
    }

    /// Returns Option of the first of "r", "g", and "b" that matches a predicate.
    /// If none matches, returns None.
    ///
    /// # Examples
    /// ```
    ///    use rust_insides::*;
    ///
    ///    let pixel = get_test_pixel();
    ///    let mut iter = pixel.into_iter();
    ///    let find_result = iter.find(|x| *x == R);
    ///    let not_found_result = iter.find(|x| *x == 0);
    ///    assert_eq!(Some(R), find_result);
    ///    assert_eq!(None, not_found_result);
    /// ```
    fn find<P>(&mut self, mut predicate: P) -> Option<Self::Item>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        if predicate(&self.item.r) {
            Some(self.item.r)
        } else if predicate(&self.item.g) {
            Some(self.item.r)
        } else if predicate(&self.item.b) {
            Some(self.item.b)
        } else {
            None
        }
    }

    /// Returns Option of index of the first of "r", "g", and "b" that matches a predicate.
    /// If none matches, returns None.
    ///
    /// # Examples
    /// ```
    ///    use rust_insides::*;
    ///
    ///    let pixel = get_test_pixel();
    ///    let mut iter = pixel.into_iter();
    ///    let zero = iter.position(|x| x == R);
    ///    let one = iter.position(|x| x == G);
    ///    let two = iter.position(|x| x == B);
    ///    let none = iter.position(|x| x == 0);
    ///    assert_eq!(Some(0), zero);
    ///    assert_eq!(Some(1), one);
    ///    assert_eq!(Some(2), two);
    ///    assert_eq!(None, none);
    /// ```
    fn position<P>(&mut self, mut predicate: P) -> Option<usize>
    where
        Self: Sized,
        P: FnMut(Self::Item) -> bool,
    {
        while let Some(el) = self.next() {
            if predicate(el) {
                return Some(self.index - 1);
            }
        }

        None
    }

    /// Returns max value of "r", "g", and "b".
    ///
    /// # Examples
    /// ```
    ///    use rust_insides::*;
    ///
    ///    let pixel = get_test_pixel();
    ///    let max = pixel.into_iter().max();
    ///    assert_eq!(Some(B), max);
    /// ```
    fn max(self) -> Option<Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        if self.item.r >= self.item.g && self.item.r >= self.item.b {
            Some(self.item.r)
        } else if self.item.g >= self.item.b {
            Some(self.item.g)
        } else {
            Some(self.item.b)
        }
    }

    /// Returns min value of "r", "g", and "b".
    ///
    /// # Examples
    /// ```
    ///    use rust_insides::*;
    ///
    ///    let pixel = get_test_pixel();
    ///    let min = pixel.into_iter().min();
    ///    assert_eq!(Some(R), min);
    /// ```
    fn min(self) -> Option<Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        if self.item.r <= self.item.g && self.item.r <= self.item.b {
            Some(self.item.r)
        } else if self.item.g <= self.item.b {
            Some(self.item.g)
        } else {
            Some(self.item.b)
        }
    }

    /// Returns sum of "r", "g", and "b".
    ///
    /// # Examples
    /// ```
    ///    use rust_insides::*;
    ///
    ///    let pixel = get_test_pixel();
    ///    let sum = pixel.into_iter().sum::<u8>();
    ///    assert_eq!(R + G + B, sum);
    /// ```
    fn sum<S>(self) -> S
    where
        Self: Sized,
        S: Sum<Self::Item>,
    {
        Sum::sum(self)
    }

    /// Check if two pixels are equal by each of "r", "g", and "b".
    ///
    /// # Examples
    /// ```
    ///    use rust_insides::get_test_pixel;
    ///
    ///    let pixel_one = get_test_pixel();
    ///    let pixel_two = get_test_pixel();
    ///    assert!(pixel_one.into_iter().eq(pixel_two));
    /// ```
    fn eq<I>(self, other: I) -> bool
    where
        I: IntoIterator,
        Self::Item: PartialEq<I::Item>,
        Self: Sized,
    {
        for (mine, other) in self.zip(other) {
            if !mine.eq(&other) {
                return false;
            }
        }
        true
    }

    /// Check if two pixels are not equal by each of "r", "g", and "b".
    ///
    /// # Examples
    /// ```
    ///    use rust_insides::get_test_pixel;
    ///
    ///    let pixel_one = get_test_pixel();
    ///    let pixel_two = get_test_pixel();
    ///    assert!(!pixel_one.into_iter().ne(pixel_two));
    /// ```
    fn ne<I>(self, other: I) -> bool
    where
        I: IntoIterator,
        Self::Item: PartialEq<I::Item>,
        Self: Sized,
    {
        !self.eq(other)
    }
}

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
    fn from_iter<T: IntoIterator<Item=u8>>(iter: T) -> Self {
        let mut into_iter = iter.into_iter();
        let r: Option<u8> = into_iter.next();
        let g: Option<u8> = into_iter.next();
        let b: Option<u8> = into_iter.next();
        PixelIntoIterator {
            item: Pixel::new(r.unwrap_or_else(|| 0), g.unwrap_or_else(|| 0), b.unwrap_or_else(|| 0)),
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

    pub fn for_each_mut<F>(mut self, mut f: F) -> Self where Self: Sized, F: FnMut(&mut u8) {
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

    fn count(self) -> usize where Self: Sized {
        3
    }

    fn last(self) -> Option<Self::Item> where Self: Sized {
        Some(self.item.b)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let result = match n {
            0 => self.item.r,
            1 => self.item.g,
            2 => self.item.b,
            _ => return None,
        };
        Some(result)
    }

    fn for_each<F>(mut self, mut f: F) where Self: Sized, F: FnMut(Self::Item) {
        while let Some(el) = self.next() {
            f(el)
        }
    }

    fn all<F>(&mut self, mut f: F) -> bool where Self: Sized, F: FnMut(Self::Item) -> bool {
        f(self.item.r) && f(self.item.g) && f(self.item.b)
    }

    fn any<F>(&mut self, mut f: F) -> bool where Self: Sized, F: FnMut(Self::Item) -> bool {
        f(self.item.r) || f(self.item.g) || f(self.item.b)
    }

    fn find<P>(&mut self, mut predicate: P) -> Option<Self::Item> where Self: Sized, P: FnMut(&Self::Item) -> bool {
        if predicate(&self.item.r) { Some(self.item.r) } else if predicate(&self.item.g) { Some(self.item.r) } else if predicate(&self.item.b) { Some(self.item.b) } else { None }
    }

    fn position<P>(&mut self, mut predicate: P) -> Option<usize> where Self: Sized, P: FnMut(Self::Item) -> bool {
        while let Some(el) = self.next() {
            if predicate(el) {
                return Some(self.index - 1);
            }
        }

        None
    }

    fn max(self) -> Option<Self::Item> where Self: Sized, Self::Item: Ord {
        if self.item.r >= self.item.g && self.item.r >= self.item.b { Some(self.item.r) } else if self.item.g >= self.item.b { Some(self.item.g) } else { Some(self.item.b) }
    }

    fn min(self) -> Option<Self::Item> where Self: Sized, Self::Item: Ord {
        if self.item.r <= self.item.g && self.item.r <= self.item.b { Some(self.item.r) } else if self.item.g <= self.item.b { Some(self.item.g) } else { Some(self.item.b) }
    }

    fn sum<S>(self) -> S where Self: Sized, S: Sum<Self::Item> {
        Sum::sum(self)
    }

    fn eq<I>(self, other: I) -> bool where I: IntoIterator, Self::Item: PartialEq<I::Item>, Self: Sized {
        for (mine, other) in self.zip(other) {
            if !mine.eq(&other) {
                return false;
            }
        }
        true
    }

    fn ne<I>(self, other: I) -> bool where I: IntoIterator, Self::Item: PartialEq<I::Item>, Self: Sized {
        !self.eq(other)
    }
}

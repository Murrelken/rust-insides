use std::iter::Sum;

#[derive(Debug)]
pub struct Pixel {
    r: i8,
    g: i8,
    b: i8,
}

pub struct PixelIntoIterator {
    item: Pixel,
    index: usize,
}

impl IntoIterator for Pixel {
    type Item = i8;
    type IntoIter = PixelIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        PixelIntoIterator {
            item: self,
            index: 0,
        }
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

impl PixelIntoIterator {
    fn next_mut(&mut self) -> Option<&mut i8> {
        let index = self.index;
        self.index += 1;
        match index {
            0 => Some(&mut self.item.r),
            1 => Some(&mut self.item.g),
            2 => Some(&mut self.item.b),
            _ => return None,
        }
    }

    pub fn for_each_mut<F>(mut self, mut f: F) where Self: Sized, F: FnMut(&mut i8) {
        while let Some(el) = self.next_mut() {
            f(el)
        }
    }
}

impl Iterator for PixelIntoIterator {
    type Item = i8;

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

    fn collect<B: FromIterator<Self::Item>>(self) -> B where Self: Sized {
        B::from_iter(self)
    }

    fn all<F>(&mut self, mut f: F) -> bool where Self: Sized, F: FnMut(Self::Item) -> bool {
        f(self.item.r) && f(self.item.g) && f(self.item.b)
    }

    fn any<F>(&mut self, mut f: F) -> bool where Self: Sized, F: FnMut(Self::Item) -> bool {
        f(self.item.r) || f(self.item.g) || f(self.item.b)
    }

    fn find<P>(&mut self, mut predicate: P) -> Option<Self::Item> where Self: Sized, P: FnMut(&Self::Item) -> bool {
        if predicate(&self.item.r) { Some(self.item.r) } else if predicate(&self.item.g) { Some(self.item.r) } else { Some(self.item.b) }
    }

    fn find_map<B, F>(&mut self, mut f: F) -> Option<B> where Self: Sized, F: FnMut(Self::Item) -> Option<B> {
        if let Some(r) = f(self.item.r) { Some(r) } else if let Some(g) = f(self.item.g) { Some(g) } else if let Some(b) = f(self.item.b) { Some(b) } else { None }
    }

    fn position<P>(&mut self, predicate: P) -> Option<usize> where Self: Sized, P: FnMut(Self::Item) -> bool {
        todo!()
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
        for (mine, other) in self.item.into_iter().zip(other) {
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

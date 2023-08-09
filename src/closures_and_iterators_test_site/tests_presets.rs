use crate::closures_and_iterators_test_site::pixel::Pixel;

pub const R: u8 = 8;
pub const G: u8 = 32;
pub const B: u8 = 128;

pub fn get_test_pixel() -> Pixel {
    Pixel::new(R, G, B)
}

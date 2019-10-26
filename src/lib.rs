pub mod alpha_cypher;
pub mod rgba_cypher;

use image::{ImageBuffer, Rgba};

pub fn load_img(path: &str) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    image::open(path).unwrap().to_rgba()
}

pub trait Cypher {
    fn encode(img: ImageBuffer<Rgba<u8>, Vec<u8>>, msg: &[u8]) -> ImageBuffer<Rgba<u8>, Vec<u8>>;
    fn decode(img: ImageBuffer<Rgba<u8>, Vec<u8>>) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;

    pub static BASE_IMG_PATH: &str = "example/test_img.jpg";

    pub fn load_test_img(path: &str) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        image::open(path).unwrap().to_rgba()
    }

    /// Tests that images are loaded as rgba without further transformation.
    #[test]
    fn test_load_img() {
        let expected = image::open(BASE_IMG_PATH).unwrap().to_rgba();
        let res = load_img(BASE_IMG_PATH);
        assert_eq!(res.to_vec(), expected.to_vec())
    }
}

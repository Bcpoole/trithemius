pub mod alpha_cypher;
pub mod rgba_cypher;

use image::{ImageBuffer, Rgba};

pub fn load_img(path: &str) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    image::open(path).unwrap().to_rgba()
}

fn get_start_idx(start: u32, width: u32) -> (u32, u32) {
    (start / width, start % width)
}

pub trait Cypher {
    fn encode(
        img: ImageBuffer<Rgba<u8>, Vec<u8>>,
        msg: &[u8],
        start_pixel: u32,
    ) -> ImageBuffer<Rgba<u8>, Vec<u8>>;
    fn decode(img: ImageBuffer<Rgba<u8>, Vec<u8>>, start_pixel: u32) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;

    pub static BASE_IMG_PATH: &str = "example/test_img.jpg";
    pub static TEST_MSG: &str = "test_msg";
    pub static TEST_START_INDEX: u32 = 15;

    pub fn load_test_img(path: &str) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        image::open(path).unwrap().to_rgba()
    }

    /// Tests that images are loaded as rgba without further transformation.
    #[test]
    fn test_load_img() {
        let expected = image::open(BASE_IMG_PATH).unwrap().to_rgba();
        let res = load_img(BASE_IMG_PATH);
        assert_eq!(res.to_vec(), expected.to_vec());
    }
}

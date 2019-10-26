use super::Cypher;

use image::{ImageBuffer, Rgba};

pub struct AlphaCypher {}

impl Cypher for AlphaCypher {
    /// Encode message in the alpha channel
    fn encode(img: ImageBuffer<Rgba<u8>, Vec<u8>>, msg: &[u8]) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let msg_len: u32 = msg.len() as u32;
        let (width, height) = img.dimensions();
        let bytes = width * height;

        if msg_len > bytes {
            panic!("Input is too large! {} bytes > {} bytes.", msg.len(), bytes)
        }

        let mut encoded_img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);

        for (x, y, pixel) in img.enumerate_pixels() {
            let idx = x + (y * width);
            if idx < msg_len {
                let mut encoded_pixel = pixel.clone();
                encoded_pixel[3] = msg[idx as usize];
                encoded_img.put_pixel(x, y, encoded_pixel);
            } else {
                encoded_img.put_pixel(x, y, pixel.clone());
            }
        }

        encoded_img
    }

    /// Decode message from alpha channel
    fn decode(img: ImageBuffer<Rgba<u8>, Vec<u8>>) -> String {
        let mut msg: Vec<u8> = Vec::new();

        for (_x, _y, pixel) in img.enumerate_pixels() {
            if pixel[3] != 255 {
                msg.push(pixel[3])
            }
        }

        String::from_utf8(msg).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::*;
    use super::*;

    /// Tests that imagebuffer no longer matches the input.
    #[test]
    fn test_encode() {
        let expected = load_test_img(BASE_IMG_PATH);
        let res = AlphaCypher::encode(load_test_img(BASE_IMG_PATH), "test".as_bytes());
        assert_ne!(res.to_vec(), expected.to_vec())
    }

    #[test]
    fn test_decode() {
        let expected = "hello";
        let res = AlphaCypher::decode(load_test_img("example/test_img_encoded_alpha.png"));
        assert_eq!(res, expected)
    }
}

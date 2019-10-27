use super::{get_start_idx, Cypher};

use image::{ImageBuffer, Rgba};
use std::cell::Cell;

pub struct AlphaCypher {}

impl Cypher for AlphaCypher {
    /// Encode message in the alpha channel
    fn encode(
        img: ImageBuffer<Rgba<u8>, Vec<u8>>,
        msg: &[u8],
        start_pixel: u32,
    ) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let msg_len: u32 = msg.len() as u32;
        let (width, height) = img.dimensions();
        let bytes = width * height;

        if msg_len > bytes {
            panic!("Input is too large! {} bytes > {} bytes.", msg.len(), bytes)
        } else if start_pixel > width * height {
            panic!("start_pixel is out of range!")
        }

        let mut encoded_img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);
        let (start_row, start_col) = get_start_idx(start_pixel, width);
        let mut inserted_pixels: u32 = 0; // how many

        let mut encode_pixel = |x: u32, y: u32| {
            let pixel = img.get_pixel(x, y).clone();
            if inserted_pixels < msg_len {
                let mut encoded_pixel = pixel.clone();
                encoded_pixel[3] = msg[inserted_pixels as usize];
                encoded_img.put_pixel(x, y, encoded_pixel);
                inserted_pixels += 1;
            } else {
                encoded_img.put_pixel(x, y, pixel.clone());
            }
        };

        for x in start_row..width {
            for y in 0..height {
                if x == start_row && y < start_col {
                    continue;
                } else {
                    encode_pixel(x, y);
                }
            }
        }

        for x in 0..start_row {
            for y in 0..height {
                if x == start_row && y >= start_col {
                    continue;
                } else {
                    encode_pixel(x, y);
                }
            }
        }

        encoded_img
    }

    /// Decode message from alpha channel
    fn decode(img: ImageBuffer<Rgba<u8>, Vec<u8>>, start_pixel: u32) -> String {
        let mut msg: Vec<u8> = Vec::new();
        let (width, height) = img.dimensions();
        let (start_row, start_col) = (start_pixel / width, start_pixel % width);

        let end_of_msg = Cell::new(false);

        let mut decode_pixel = |x: u32, y: u32| {
            let pixel = img.get_pixel(x, y);
            if pixel[3] != 255 {
                msg.push(pixel[3])
            } else {
                end_of_msg.set(true)
            }
        };

        for x in start_row..width {
            if end_of_msg.get() {
                break;
            }
            for y in 0..height {
                if end_of_msg.get() {
                    break;
                } else if x == start_row && y < start_col {
                    continue;
                } else {
                    decode_pixel(x, y);
                }
            }
        }

        for x in 0..start_row {
            if end_of_msg.get() {
                break;
            }
            for y in 0..height {
                if end_of_msg.get() {
                    break;
                } else if x == start_row && y >= start_col {
                    continue;
                } else {
                    decode_pixel(x, y);
                }
            }
        }

        String::from_utf8(msg).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::*;
    use super::*;

    /// should panic if message is too long to fit in image.
    #[test]
    #[should_panic(expected = "Input is too large! 8 bytes > 1 bytes.")]
    fn test_rgba_encode_msg_too_long() {
        AlphaCypher::encode(ImageBuffer::new(1, 1), TEST_MSG.as_bytes(), 0);
    }

    /// should panic if start_pixel is out of bounds.
    #[test]
    #[should_panic(expected = "start_pixel is out of range!")]
    fn test_rgba_encode_start_pixel_out_of_bounds() {
        AlphaCypher::encode(ImageBuffer::new(1, 1), "a".as_bytes(), 1000);
    }

    /// Tests that TEST_START_INDEX contains the start of TEST_MSG in the output but not the input.
    #[test]
    fn test_alpha_encode_encodes_msg_in_image_buffer() {
        let expected = load_test_img(BASE_IMG_PATH);
        let res = AlphaCypher::encode(
            load_test_img(BASE_IMG_PATH),
            TEST_MSG.as_bytes(),
            TEST_START_INDEX,
        );
        assert_ne!(
            res.get_pixel(0, TEST_START_INDEX),
            expected.get_pixel(0, TEST_START_INDEX)
        );

        for i in 0..=3 {
            assert_eq!(
                res.get_pixel(0, TEST_START_INDEX + i)[3],
                TEST_MSG.as_bytes()[i as usize]
            );
        }
    }

    #[test]
    fn test_alpha_decode_decodes_msg_from_image_buffe() {
        let res = AlphaCypher::decode(
            load_test_img("example/test_img_encoded_alpha.png"),
            TEST_START_INDEX,
        );
        assert_eq!(res, TEST_MSG);
    }
}

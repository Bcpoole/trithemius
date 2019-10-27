use super::{get_start_idx, Cypher};

use image::{ImageBuffer, Rgba};
use std::cell::Cell;

pub struct RgbaCypher {}

impl Cypher for RgbaCypher {
    /// Encode message in the rgba marking the end with three 0 bytes or whole image size, whichever is smaller.
    fn encode(
        img: ImageBuffer<Rgba<u8>, Vec<u8>>,
        msg: &[u8],
        start_pixel: u32,
    ) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let msg_len: u32 = msg.len() as u32;
        let (width, height) = img.dimensions();
        let bytes = width * height * 4;

        if msg_len > bytes {
            panic!("Input is too large! {} bytes > {} bytes.", msg.len(), bytes)
        } else if start_pixel > width * height {
            panic!("start_pixel is out of range!")
        }

        let mut encoded_img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);
        let (start_row, start_col) = get_start_idx(start_pixel, width);
        let mut inserted_pixels: u32 = 0;
        let mut add_end_of_msg = false;

        let mut encode_pixel = |x: u32, y: u32| {
            let mut pixel = img.get_pixel(x, y).clone();
            for i in 0..=3 {
                if inserted_pixels < msg_len {
                    pixel[i as usize] = msg[inserted_pixels as usize];
                    inserted_pixels += 1;
                    if inserted_pixels == msg_len {
                        add_end_of_msg = true;
                    }
                } else if add_end_of_msg {
                    pixel[i as usize] = 0;
                }
            }
            encoded_img.put_pixel(x, y, pixel);
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

    /// Decode message from rgba
    fn decode(img: ImageBuffer<Rgba<u8>, Vec<u8>>, start_pixel: u32) -> String {
        let mut msg: Vec<u8> = Vec::new();

        let end_of_msg = Cell::new(false);

        let (width, height) = img.dimensions();
        let (start_row, start_col) = get_start_idx(start_pixel, width);

        let mut decode_pixel = |x: u32, y: u32| {
            let pixel = img.get_pixel(x, y);
            for i in 0..=3 {
                if pixel[i] == 0 && !msg.is_empty() {
                    end_of_msg.set(true);
                } else if end_of_msg.get() {
                    break;
                } else {
                    msg.push(pixel[i]);
                }
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
    #[should_panic(expected = "Input is too large! 8 bytes > 4 bytes.")]
    fn test_rgba_encode_msg_too_long() {
        RgbaCypher::encode(ImageBuffer::new(1, 1), TEST_MSG.as_bytes(), 0);
    }

    /// should panic if start_pixel is out of bounds.
    #[test]
    #[should_panic(expected = "start_pixel is out of range!")]
    fn test_rgba_encode_start_pixel_out_of_bounds() {
        RgbaCypher::encode(ImageBuffer::new(1, 1), "a".as_bytes(), 1000);
    }

    /// Tests that TEST_START_INDEX contains the start of TEST_MSG in the output but not the input.
    #[test]
    fn test_rgba_encode_encodes_msg_in_image_buffer() {
        let expected = load_test_img(BASE_IMG_PATH);

        let res = RgbaCypher::encode(
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
                res.get_pixel(0, TEST_START_INDEX)[i],
                TEST_MSG.as_bytes()[i]
            );
        }
    }

    #[test]
    fn test_rgba_decode_decodes_msg_from_image_buffer() {
        let res = RgbaCypher::decode(
            load_test_img("example/test_img_encoded_rgba.png"),
            TEST_START_INDEX,
        );
        assert_eq!(res, TEST_MSG);
    }
}

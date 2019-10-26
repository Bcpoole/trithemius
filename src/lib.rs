use image::{ImageBuffer, Rgba};

pub fn load_img(path: &str) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    image::open(path).unwrap().to_rgba()
}

/// Encode message in the alpha channel
pub fn encode_msg_alpha(
    img: ImageBuffer<Rgba<u8>, Vec<u8>>,
    msg: &[u8],
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
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
pub fn decode_msg_alpha(img: ImageBuffer<Rgba<u8>, Vec<u8>>) -> String {
    let mut msg: Vec<u8> = Vec::new();

    for (_x, _y, pixel) in img.enumerate_pixels() {
        if pixel[3] != 255 {
            msg.push(pixel[3])
        }
    }

    String::from_utf8(msg).unwrap()
}

/// Encode message in the rgba marking the end with three 0 bytes or whole image size, whichever is smaller.
pub fn encode_msg_rgba(
    img: ImageBuffer<Rgba<u8>, Vec<u8>>,
    msg: &[u8],
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let msg_len: u32 = msg.len() as u32;
    let (width, height) = img.dimensions();
    let bytes = width * height * 4;

    if msg_len > bytes {
        panic!("Input is too large! {} bytes > {} bytes.", msg.len(), bytes)
    }

    let mut encoded_img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);

    let mut msg_end = 3;
    for (x, y, pixel) in img.enumerate_pixels() {
        let mut encoded_pixel = pixel.clone();
        for i in 0..=3 {
            let idx = (x + (y * width)) * 4 + i;
            if idx < msg_len {
                encoded_pixel[i as usize] = msg[idx as usize];
            } else if msg_end > 0 {
                encoded_pixel[i as usize] = 0;
                msg_end -= 1;
            }
        }
        encoded_img.put_pixel(x, y, encoded_pixel);
    }

    encoded_img
}

/// Decode message from rgba
pub fn decode_msg_rgba(img: ImageBuffer<Rgba<u8>, Vec<u8>>) -> String {
    let mut msg: Vec<u8> = Vec::new();

    let mut msg_end_counter = 0; // look for three 0 bytes
    let mut end_of_msg = false;
    for (_x, _y, pixel) in img.enumerate_pixels() {
        if end_of_msg {
            break;
        }
        for i in 0..=3 {
            msg.push(pixel[i]);

            if pixel[i] == 0 {
                msg_end_counter += 1;
                if msg_end_counter == 3 {
                    end_of_msg = true;
                }
            } else {
                msg_end_counter = 0;
            }
        }
    }

    let msg = String::from_utf8(msg).unwrap();
    return msg[..(msg.len() - msg_end_counter)].to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load_img() {
        let path = "example/base_img.jpg";
        let expected = image::open(path).unwrap().to_rgba();
        let res = load_img(path);
        assert_eq!(res.to_vec(), expected.to_vec())
    }
}

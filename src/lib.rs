use image::{DynamicImage, ImageBuffer, Rgba};

pub fn encode_msg(
    img: ImageBuffer<Rgba<u8>, Vec<u8>>,
    msg: &[u8],
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let msg_len: u32 = msg.len() as u32;
    let (width, height) = img.dimensions();
    let bytes = width * height;

    if msg_len > height {
        panic!(
            "Input is too large! {} bytes > {} bytes.",
            msg.len(),
            height
        )
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

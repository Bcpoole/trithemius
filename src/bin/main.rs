use steganography;

fn main() {
    let img = image::open("example/base_img.jpg").unwrap().to_rgba();
    let encoded_img = steganography::encode_msg(img, "hello".as_bytes());
    encoded_img.save("example/encoded_img.jpg").unwrap();
}

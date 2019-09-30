use steganography;

fn main() {
    let img = image::open("example/base_img.jpg").unwrap().to_rgba();
    let in_msg = "hello";

    let encoded_img = steganography::encode_msg(img, in_msg.as_bytes());

    let out_msg = steganography::decode_msg(encoded_img.clone());
    assert!(in_msg == out_msg);

    encoded_img.save("example/encoded_img.jpg").unwrap();
}

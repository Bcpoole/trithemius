#[test]
fn test_alpha_encode_decode() {
    let img = image::open("example/base_img.jpg").unwrap().to_rgba();
    let in_msg = "hello";

    let alpha_encoded_img = steganography::encode_msg_alpha(img, in_msg.as_bytes());

    let alpha_out_msg = steganography::decode_msg_alpha(alpha_encoded_img.clone());
    assert!(in_msg == alpha_out_msg);
}

#[test]
fn test_rgba_encode_decode() {
    let img = image::open("example/base_img.jpg").unwrap().to_rgba();
    let in_msg = "hello";
    let rgba_encoded_img = steganography::encode_msg_rgba(img, in_msg.as_bytes());

    let rgba_out_msg = steganography::decode_msg_rgba(rgba_encoded_img.clone());
    assert!(in_msg == rgba_out_msg);
}

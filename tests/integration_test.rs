use steganography::*;

static IMG_PATH: &str = "example/test_img.jpg";

#[test]
fn test_alpha_encode_decode() {
    let img = load_img(IMG_PATH);
    let in_msg = "hello";

    let alpha_encoded_img = alpha_cypher::AlphaCypher::encode(img, in_msg.as_bytes());

    let alpha_out_msg = alpha_cypher::AlphaCypher::decode(alpha_encoded_img.clone());
    assert!(in_msg == alpha_out_msg);
}

#[test]
fn test_rgba_encode_decode() {
    let img = load_img(IMG_PATH);
    let in_msg = "hello";

    let rgba_encoded_img = rgba_cypher::RgbaCypher::encode(img, in_msg.as_bytes());

    let rgba_out_msg = rgba_cypher::RgbaCypher::decode(rgba_encoded_img.clone());
    assert!(in_msg == rgba_out_msg);
}

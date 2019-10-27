use steganography::*;

static IMG_PATH: &str = "example/test_img.jpg";
static TEST_MSG: &str = "test_msg";
static TEST_START_INDEX: u32 = 15;

#[test]
fn test_alpha_encode_decode() {
    let img = load_img(IMG_PATH);

    let alpha_encoded_img =
        alpha_cypher::AlphaCypher::encode(img, TEST_MSG.as_bytes(), TEST_START_INDEX);
    alpha_encoded_img
        .save("example/test_img_encoded_alpha.png")
        .unwrap();

    let alpha_out_msg =
        alpha_cypher::AlphaCypher::decode(alpha_encoded_img.clone(), TEST_START_INDEX);
    assert!(alpha_out_msg == TEST_MSG);
}

#[test]
fn test_rgba_encode_decode() {
    let img = load_img(IMG_PATH);

    let rgba_encoded_img =
        rgba_cypher::RgbaCypher::encode(img, TEST_MSG.as_bytes(), TEST_START_INDEX);
    rgba_encoded_img
        .save("example/test_img_encoded_rgba.png")
        .unwrap();

    let rgba_out_msg = rgba_cypher::RgbaCypher::decode(rgba_encoded_img.clone(), TEST_START_INDEX);
    assert!(rgba_out_msg == TEST_MSG);
}

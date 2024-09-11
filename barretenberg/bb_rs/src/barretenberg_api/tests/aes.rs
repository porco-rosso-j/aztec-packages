use crate::barretenberg_api::aes;

//  cargo test test_aes_encrypt_buffer_cbc -- --nocapture
#[test]
fn test_aes_encrypt_buffer_cbc() {
    let message = [
        6, 196, 4, 126, 220, 48, 240, 65, 72, 173, 40, 101, 187, 150, 245, 115, 253, 193, 91, 5,
        45, 148, 91, 74, 184, 111, 200, 144, 36, 203, 76, 229,
    ];
    let iv = [
        211, 212, 43, 21, 246, 77, 247, 93, 115, 207, 52, 165, 216, 81, 100, 132,
    ];
    let key = [
        225, 168, 31, 197, 3, 144, 253, 98, 206, 169, 154, 199, 178, 43, 32, 7,
    ];

    let length = 32u32;
    let encrypted_data = unsafe { aes::aes_encrypt_buffer_cbc(&message, &iv, &key, &length) };
    println!("{:?}", encrypted_data);
}

//  cargo test test_aes_decrypt_buffer_cbc -- --nocapture
#[test]
fn test_aes_decrypt_buffer_cbcc() {
    let data = [
        184, 87, 245, 35, 196, 181, 226, 98, 168, 220, 71, 210, 211, 146, 221, 241, 36, 170, 216,
        19, 15, 49, 35, 173, 112, 0, 32, 160, 1, 82, 240, 27, 238, 85, 228, 133, 206, 185, 5, 66,
        171, 163, 85, 43, 99, 84, 129, 79,
    ];
    let iv = [
        211, 212, 43, 21, 246, 77, 247, 93, 115, 207, 52, 165, 216, 81, 100, 132,
    ];
    let key = [
        225, 168, 31, 197, 3, 144, 253, 98, 206, 169, 154, 199, 178, 43, 32, 7,
    ];

    let length = 48u32;
    let decrypted_data = unsafe { aes::aes_decrypt_buffer_cbc(&data, &iv, &key, &length) };
    println!("{:?}", decrypted_data);
}

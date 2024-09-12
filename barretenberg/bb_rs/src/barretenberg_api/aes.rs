use super::{
    bindgen,
    traits::{DeserializeBuffer, SerializeBuffer},
};

// TODO: doesn't work somehow. getting stuck.
pub unsafe fn aes_encrypt_buffer_cbc(
    input: &[u8], // Input data
    iv: &[u8],    // Initialization Vector
    key: &[u8],   // Encryption Key
    length: &u32, // Length of the input data
) -> [u8; 32] {
    // Result buffer (32 bytes)
    let mut result = [0u8; 32]; // Output buffer, assuming a fixed size of 32 bytes

    bindgen::aes_encrypt_buffer_cbc(
        input.as_ptr(),           // Input buffer pointer
        iv.as_ptr(),              // IV buffer pointer
        key.as_ptr(),             // Key buffer pointer
        length,                   // Length pointer (32-bit integer)
        &mut result.as_mut_ptr(), // Result buffer pointer
    );

    result
}

// TODO: doesn't work somehow. getting stuck.
pub unsafe fn aes_decrypt_buffer_cbc(
    input: &[u8], // Encrypted input data
    iv: &[u8],    // Initialization Vector
    key: &[u8],   // Encryption Key
    length: &u32, // Length of the input data
) -> [u8; 32] {
    // Result buffer (32 bytes)
    let mut result = [0u8; 32]; // Output buffer, assuming a fixed size of 32 bytes

    bindgen::aes_decrypt_buffer_cbc(
        input.as_ptr(),           // Input buffer pointer
        iv.as_ptr(),              // IV buffer pointer
        key.as_ptr(),             // Key buffer pointer
        length,                   // Length pointer (32-bit integer)
        &mut result.as_mut_ptr(), // Result buffer pointer
    );

    result
}

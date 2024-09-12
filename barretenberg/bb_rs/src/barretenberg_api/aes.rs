use super::{
    bindgen,
    traits::{DeserializeBuffer, SerializeBuffer},
};

// TODO: doesn't work somehow. getting stuck.
pub unsafe fn aes_encrypt_buffer_cbc(
    input: &[u8],
    iv: &[u8],
    key: &[u8],
    length: &u32,
) -> [u8; 32] {
    let mut result = [0u8; 32];

    bindgen::aes_encrypt_buffer_cbc(
        input.as_ptr(),
        iv.as_ptr(),
        key.as_ptr(),
        length,
        &mut result.as_mut_ptr(),
    );

    result
}

// TODO: doesn't work somehow. getting stuck.
pub unsafe fn aes_decrypt_buffer_cbc(
    input: &[u8],
    iv: &[u8],
    key: &[u8],
    length: &u32,
) -> [u8; 32] {
    let mut result = [0u8; 32];

    bindgen::aes_decrypt_buffer_cbc(
        input.as_ptr(),
        iv.as_ptr(),
        key.as_ptr(),
        length,
        &mut result.as_mut_ptr(),
    );

    result
}

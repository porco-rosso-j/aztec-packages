use super::bindgen;

pub unsafe fn ecc_grumpkin__mul(point_buf: &[u8], scalar_buf: &[u8]) -> [u8; 64] {
    let mut result = [0u8; 64];

    bindgen::ecc_grumpkin__mul(point_buf.as_ptr(), scalar_buf.as_ptr(), result.as_mut_ptr());

    result
}

/// Adds two points using the Grumpkin curve.
pub unsafe fn ecc_grumpkin__add(point_a_buf: &[u8], point_b_buf: &[u8]) -> [u8; 64] {
    let mut result = [0u8; 64]; // Buffer to store the result

    // Call the C++ function
    bindgen::ecc_grumpkin__add(
        point_a_buf.as_ptr(),
        point_b_buf.as_ptr(),
        result.as_mut_ptr(),
    );

    result
}

pub unsafe fn ecc_grumpkin__batch_mul(
    point_buf: &[u8],
    scalar_buf: &[u8],
    num_points: &u32,
) -> [u8; 64] {
    let mut result = [0u8; 64];

    bindgen::ecc_grumpkin__batch_mul(
        point_buf.as_ptr(),
        scalar_buf.as_ptr(),
        *num_points,
        result.as_mut_ptr(),
    );

    result
}

pub unsafe fn ecc_grumpkin__get_random_scalar_mod_circuit_modulus() -> [u8; 32] {
    let mut result = [0u8; 32];

    bindgen::ecc_grumpkin__get_random_scalar_mod_circuit_modulus(result.as_mut_ptr());

    result
}

pub unsafe fn ecc_grumpkin__reduce512_buffer_mod_circuit_modulus(input: &mut [u8]) -> [u8; 32] {
    let mut result = [0u8; 32];

    bindgen::ecc_grumpkin__reduce512_buffer_mod_circuit_modulus(
        input.as_mut_ptr(),
        result.as_mut_ptr(),
    );

    result
}

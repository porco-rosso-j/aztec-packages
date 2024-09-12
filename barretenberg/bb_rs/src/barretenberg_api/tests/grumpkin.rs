use crate::barretenberg_api::grumpkin;
use crate::barretenberg_api::models::{Fr, Point};
use crate::barretenberg_api::traits::{DeserializeBuffer, SerializeBuffer};
// cargo test test_compute_with_grumpkin -- --nocapture

#[test]
fn test_compute_with_grumpkin() {
    let value: [u8; 32] = [
        24, 137, 63, 144, 3, 163, 23, 154, 31, 36, 82, 59, 156, 27, 132, 51, 75, 95, 230, 238, 96,
        136, 200, 140, 7, 69, 145, 219, 14, 102, 20, 56,
    ];

    let gen_x: [u8; 32] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1,
    ];

    let gen_y: [u8; 32] = [
        0, 0, 0, 0, 0, 0, 0, 2, 207, 19, 94, 117, 6, 164, 93, 99, 45, 39, 13, 69, 241, 24, 18, 148,
        131, 63, 196, 141, 130, 63, 39, 44,
    ];
    let generator = Point {
        x: Fr { data: gen_x },
        y: Fr { data: gen_y },
    };

    let product_buffer = unsafe { grumpkin::ecc_grumpkin__mul(&generator.to_buffer(), &value) };
    // println!("product buffer {:?}", product_buffer);

    let sum_buffer =
        unsafe { grumpkin::ecc_grumpkin__add(&generator.to_buffer(), &product_buffer) };
    // println!("sum buffer {:?}", sum_buffer);

    let generator_buffer = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 207, 19, 94, 117, 6, 164, 93, 99, 45, 39, 13, 69, 241, 24,
        18, 148, 131, 63, 196, 141, 130, 63, 39, 44,
    ];

    let scalar_buffer = [
        40, 166, 91, 136, 58, 67, 179, 102, 120, 39, 27, 74, 142, 217, 28, 241, 196, 127, 153, 52,
        123, 67, 161, 42, 159, 47, 219, 30, 240, 253, 173, 132, 36, 21, 84, 100, 238, 216, 235, 67,
        142, 84, 31, 43, 255, 144, 231, 7, 23, 254, 138, 250, 23, 105, 219, 231, 59, 170, 85, 113,
        141, 154, 32, 101,
    ];

    let val: u32 = 256;

    let batch_product_buffer =
        unsafe { grumpkin::ecc_grumpkin__batch_mul(&generator_buffer, &scalar_buffer, val) };
    println!("batch product buffer {:?}", batch_product_buffer);

    let get_random_scalar_mod_circuit_modulus_buffer =
        unsafe { grumpkin::ecc_grumpkin__get_random_scalar_mod_circuit_modulus() };
    println!(
        "get_random_scalar_mod_circuit_modulus_buffer {:?}",
        get_random_scalar_mod_circuit_modulus_buffer
    );

    let mut val = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 27,
    ];

    let reduce512_buffer_mod_circuit_modulus_buffer =
        unsafe { grumpkin::ecc_grumpkin__reduce512_buffer_mod_circuit_modulus(&mut val) };

    println!(
        "reduce512_buffer_mod_circuit_modulus_buffer {:?}",
        reduce512_buffer_mod_circuit_modulus_buffer
    );

    // array -> Fr -> Fr array
}

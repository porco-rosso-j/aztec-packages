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
    println!("product buffer {:?}", product_buffer);

    let sum_buffer =
        unsafe { grumpkin::ecc_grumpkin__add(&generator.to_buffer(), &product_buffer) };
    println!("sum buffer {:?}", sum_buffer);

    // let batch_product_buffer = unsafe {
    //     grumpkin::ecc_grumpkin__batch_mul(&generator.to_buffer(), &product_buffer, &12u32)
    // };
    // println!("batch product buffer {:?}", batch_product_buffer);

    let get_random_scalar_mod_circuit_modulus_buffer =
        unsafe { grumpkin::ecc_grumpkin__get_random_scalar_mod_circuit_modulus() };

    let mut val = [27];
    let reduce512_buffer_mod_circuit_modulus_buffer =
        unsafe { grumpkin::ecc_grumpkin__reduce512_buffer_mod_circuit_modulus(&mut val) };

    println!(
        "reduce512_buffer_mod_circuit_modulus_buffer {:?}",
        reduce512_buffer_mod_circuit_modulus_buffer
    );

    // array -> Fr -> Fr array
}

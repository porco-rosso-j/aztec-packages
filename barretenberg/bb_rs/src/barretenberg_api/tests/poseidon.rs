use crate::barretenberg_api::models::Fr;
use crate::barretenberg_api::poseidon2;

// cargo test test_poseidon2_hash -- --nocapture
#[test]
fn test_poseidon2_hash() {
    let value = [
        6, 196, 4, 126, 220, 48, 240, 65, 72, 173, 40, 101, 187, 150, 245, 115, 253, 193, 91, 5,
        45, 148, 91, 74, 184, 111, 200, 144, 36, 203, 76, 229,
    ];

    let result = unsafe { poseidon2::poseidon2_hash(&[Fr { data: value }]) };
    println!("{:?}", result);
}

// cargo test test_poseidon2_hash -- --nocapture
#[test]
fn test_poseidon2_hashes() {
    let value = [
        6, 196, 4, 126, 220, 48, 240, 65, 72, 173, 40, 101, 187, 150, 245, 115, 253, 193, 91, 5,
        45, 148, 91, 74, 184, 111, 200, 144, 36, 203, 76, 229,
    ];

    let result = unsafe { poseidon2::poseidon2_hashes(&[Fr { data: value }]) };
    println!("{:?}", result);
}

// cargo test poseidon2_permutation -- --nocapture
#[test]
fn test_poseidon2_permutation() {
    let value = [
        6, 196, 4, 126, 220, 48, 240, 65, 72, 173, 40, 101, 187, 150, 245, 115, 253, 193, 91, 5,
        45, 148, 91, 74, 184, 111, 200, 144, 36, 203, 76, 229,
    ];

    let result = unsafe { poseidon2::poseidon2_permutation(&[Fr { data: value }]) };
    println!("{:?}", result);
}

use crate::barretenberg_api::models::Fr;
use crate::barretenberg_api::pedersen;

// cargo test test_pedersen_hash -- --nocapture

#[test]
fn test_pedersen_hash() {
    let value = [
        6, 196, 4, 126, 220, 48, 240, 65, 72, 173, 40, 101, 187, 150, 245, 115, 253, 193, 91, 5,
        45, 148, 91, 74, 184, 111, 200, 144, 36, 203, 76, 229,
    ];
    let result = unsafe { pedersen::pedersen_hash(&[Fr { data: value }], 0) };
    println!("{:?}", result);
}

// cargo test test_pedersen_commit -- --nocapture

#[test]
fn test_pedersen_commit() {
    let value = [
        6, 196, 4, 126, 220, 48, 240, 65, 72, 173, 40, 101, 187, 150, 245, 115, 253, 193, 91, 5,
        45, 148, 91, 74, 184, 111, 200, 144, 36, 203, 76, 229,
    ];
    let result = unsafe { pedersen::pedersen_commit(&[Fr { data: value }]) };
    println!("{:?}", result);
}

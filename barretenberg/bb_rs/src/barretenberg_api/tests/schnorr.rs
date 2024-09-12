use crate::barretenberg_api::models::Fr;
use crate::barretenberg_api::schnorr;
use crate::barretenberg_api::traits::{DeserializeBuffer, SerializeBuffer};

//  cargo test test_schnorr_compute_public_key -- --nocapture
#[test]
fn test_schnorr_compute_public_key() {
    let private_key = [
        6, 196, 4, 126, 220, 48, 240, 65, 72, 173, 40, 101, 187, 150, 245, 115, 253, 193, 91, 5,
        45, 148, 91, 74, 184, 111, 200, 144, 36, 203, 76, 229,
    ];
    let pubkey = unsafe { schnorr::schnorr_compute_public_key(&private_key) };
    println!("{:?}", pubkey);
}

// cargo test test_schnorr_construct_signature -- --nocapture
#[test]
fn test_schnorr_construct_signature() {
    let hashed_message = [
        162, 11, 221, 82, 101, 38, 145, 153, 143, 140, 164, 97, 148, 164, 55, 133, 17, 93, 198, 63,
        125, 169, 147, 53, 229, 221, 91, 27, 2, 153, 104, 166,
    ];

    let private_key = [
        1, 124, 129, 11, 95, 129, 118, 34, 63, 106, 20, 43, 110, 152, 20, 47, 212, 57, 25, 192,
        123, 93, 100, 106, 31, 81, 17, 36, 29, 118, 201, 86,
    ];

    let (s, e) = unsafe { schnorr::schnorr_construct_signature(&hashed_message, &private_key) };

    println!("s: {:?}", s);
    println!("e: {:?}", e);
}

// cargo test test_schnorr_verify_signature -- --nocapture
#[test]
fn test_schnorr_verify_signature() {
    let hashed_message = [
        162, 11, 221, 82, 101, 38, 145, 153, 143, 140, 164, 97, 148, 164, 55, 133, 17, 93, 198, 63,
        125, 169, 147, 53, 229, 221, 91, 27, 2, 153, 104, 166,
    ];

    let pubkey = [
        33, 19, 143, 189, 232, 117, 66, 250, 74, 2, 28, 58, 160, 8, 157, 74, 211, 223, 158, 178,
        86, 90, 53, 143, 235, 187, 99, 187, 12, 126, 80, 46, 8, 8, 186, 201, 100, 142, 247, 156,
        228, 231, 24, 1, 125, 52, 145, 50, 120, 135, 25, 141, 249, 164, 114, 94, 201, 103, 165,
        147, 178, 70, 92, 182,
    ];

    let s = [
        40, 175, 118, 99, 128, 220, 32, 85, 180, 238, 212, 101, 110, 113, 96, 63, 138, 65, 204,
        153, 96, 217, 167, 61, 38, 115, 104, 142, 197, 228, 189, 111,
    ];

    let e = [
        144, 219, 152, 86, 59, 253, 241, 118, 138, 131, 106, 147, 231, 200, 206, 131, 195, 36, 107,
        73, 147, 34, 143, 75, 191, 144, 247, 170, 101, 53, 148, 223,
    ];
    let result = unsafe { schnorr::schnorr_verify_signature(&hashed_message, &pubkey, &s, &e) };
    println!("{:?}", result);
}

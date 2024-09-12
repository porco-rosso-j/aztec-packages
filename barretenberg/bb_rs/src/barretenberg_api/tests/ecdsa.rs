use crate::barretenberg_api::ecdsa;
use crate::barretenberg_api::models::Fr;
use crate::barretenberg_api::traits::{DeserializeBuffer, SerializeBuffer};

//  cargo test test_ecdsa_compute_public_key -- --nocapture
#[test]
fn test_ecdsa_compute_public_key() {
    let private_key = [
        6, 196, 4, 126, 220, 48, 240, 65, 72, 173, 40, 101, 187, 150, 245, 115, 253, 193, 91, 5,
        45, 148, 91, 74, 184, 111, 200, 144, 36, 203, 76, 229,
    ];
    let pubkey = unsafe { ecdsa::ecdsa__compute_public_key(&private_key) };
    println!("{:?}", pubkey);

    // let expected_pub_key = [
    //     30, 208, 241, 215, 103, 94, 254, 117, 70, 152, 81, 205, 82, 148, 1, 169, 164, 67, 60, 5,
    //     20, 174, 250, 221, 242, 79, 1, 65, 210, 85, 102, 168, 120, 138, 73, 199, 27, 230, 199, 139,
    //     88, 30, 223, 222, 101, 80, 215, 183, 206, 127, 56, 111, 54, 82, 44, 173, 86, 64, 143, 202,
    //     3, 123, 35, 7,
    // ];
}

// cargo test test_ecdsa__construct_signature -- --nocapture
#[test]
fn test_ecdsa__construct_signature() {
    let hashed_message = [
        162, 11, 221, 82, 101, 38, 145, 153, 143, 140, 164, 97, 148, 164, 55, 133, 17, 93, 198, 63,
        125, 169, 147, 53, 229, 221, 91, 27, 2, 153, 104, 166,
    ];

    let private_key = [
        171, 34, 16, 89, 133, 250, 122, 117, 28, 144, 207, 224, 134, 19, 41, 15, 31, 179, 180, 49,
        11, 48, 154, 79, 81, 240, 167, 249, 91, 136, 85, 165,
    ];

    let (sig_r, sig_s, sig_v) = unsafe {
        ecdsa::ecdsa__construct_signature(&hashed_message, hashed_message.len(), &private_key)
    };

    println!("{:?}", sig_r);
    println!("{:?}", sig_s);
    println!("{:?}", sig_v);

    // let expected_sig_r = [
    //     1, 188, 236, 84, 137, 101, 248, 138, 176, 59, 19, 252, 26, 182, 25, 146, 14, 133, 122, 82,
    //     142, 190, 176, 190, 72, 151, 136, 152, 135, 11, 96, 156,
    // ];

    // let expected_sig_s = [
    //     25, 90, 155, 18, 200, 150, 103, 84, 138, 244, 113, 135, 119, 90, 175, 219, 203, 84, 2, 116,
    //     181, 185, 38, 112, 213, 116, 120, 16, 112, 254, 241, 66,
    // ];

    // let expected_sig_v = 27u8;
}

// cargo test test_ecdsa__recover_public_key_from_signature -- --nocapture
#[test]
fn test_ecdsa__recover_public_key_from_signature() {
    let hashed_message = [
        162, 11, 221, 82, 101, 38, 145, 153, 143, 140, 164, 97, 148, 164, 55, 133, 17, 93, 198, 63,
        125, 169, 147, 53, 229, 221, 91, 27, 2, 153, 104, 166,
    ];

    let sig_r = [
        1, 188, 236, 84, 137, 101, 248, 138, 176, 59, 19, 252, 26, 182, 25, 146, 14, 133, 122, 82,
        142, 190, 176, 190, 72, 151, 136, 152, 135, 11, 96, 156,
    ];

    let sig_s = [
        25, 90, 155, 18, 200, 150, 103, 84, 138, 244, 113, 135, 119, 90, 175, 219, 203, 84, 2, 116,
        181, 185, 38, 112, 213, 116, 120, 16, 112, 254, 241, 66,
    ];
    let mut sig_v = 27u8;

    let recovered_pub_key = unsafe {
        ecdsa::ecdsa__recover_public_key_from_signature(
            &hashed_message,
            hashed_message.len(),
            &sig_r,
            &sig_s,
            &mut sig_v,
        )
    };

    println!("{:?}", recovered_pub_key);

    // let expected_pub_key = [
    //     30, 208, 241, 215, 103, 94, 254, 117, 70, 152, 81, 205, 82, 148, 1, 169, 164, 67, 60, 5,
    //     20, 174, 250, 221, 242, 79, 1, 65, 210, 85, 102, 168, 120, 138, 73, 199, 27, 230, 199, 139,
    //     88, 30, 223, 222, 101, 80, 215, 183, 206, 127, 56, 111, 54, 82, 44, 173, 86, 64, 143, 202,
    //     3, 123, 35, 7,
    // ];
}

// cargo test test_ecdsa__verify_signature -- --nocapture
#[test]
fn test_ecdsa__verify_signature() {
    let hashed_message = [
        162, 11, 221, 82, 101, 38, 145, 153, 143, 140, 164, 97, 148, 164, 55, 133, 17, 93, 198, 63,
        125, 169, 147, 53, 229, 221, 91, 27, 2, 153, 104, 166,
    ];

    let pubkey = [
        30, 208, 241, 215, 103, 94, 254, 117, 70, 152, 81, 205, 82, 148, 1, 169, 164, 67, 60, 5,
        20, 174, 250, 221, 242, 79, 1, 65, 210, 85, 102, 168, 120, 138, 73, 199, 27, 230, 199, 139,
        88, 30, 223, 222, 101, 80, 215, 183, 206, 127, 56, 111, 54, 82, 44, 173, 86, 64, 143, 202,
        3, 123, 35, 7,
    ];

    let sig_r = [
        1, 188, 236, 84, 137, 101, 248, 138, 176, 59, 19, 252, 26, 182, 25, 146, 14, 133, 122, 82,
        142, 190, 176, 190, 72, 151, 136, 152, 135, 11, 96, 156,
    ];

    let sig_s = [
        25, 90, 155, 18, 200, 150, 103, 84, 138, 244, 113, 135, 119, 90, 175, 219, 203, 84, 2, 116,
        181, 185, 38, 112, 213, 116, 120, 16, 112, 254, 241, 66,
    ];

    let mut sig_v = 27u8;

    let result = unsafe {
        ecdsa::ecdsa__verify_signature(
            &hashed_message,
            hashed_message.len(),
            &pubkey,
            &sig_r,
            &sig_s,
            sig_v,
        )
    };
    println!("{:?}", result); // expect () == true
}

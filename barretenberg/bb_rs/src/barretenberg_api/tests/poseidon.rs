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
    // let result = unsafe { poseidon2::poseidon2_hash(&value) };
    println!("{:?}", result);

    /*
    Fr {
       data: [30, 214, 137, 169, 70, 110, 73, 191, 253, 150, 74, 33,
              179, 123, 61, 16, 156, 153, 106, 203, 195, 50, 131, 117, 96,
              126, 120, 145, 153, 133, 105, 204]
        }
     */
}

// cargo test test_poseidon2_permutation -- --nocapture
#[test]
fn test_poseidon2_permutation() {
    let value0 = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1,
    ];

    let value1 = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 2,
    ];

    let value2 = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 3,
    ];

    let values: [Fr; 3] = [
        Fr { data: value0 },
        Fr { data: value1 },
        Fr { data: value2 },
    ];

    //let result = unsafe { poseidon2::poseidon2_permutation(&[Fr { data: value }]) };
    let result = unsafe { poseidon2::poseidon2_permutation(&values) };
    //  println!("result: {:?}", result);
}

// cargo test poseidon2_permutation -- --nocapture
// #[test]
// fn test_poseidon2_permutation() {
//     let value = [
//         6, 196, 4, 126, 220, 48, 240, 65, 72, 173, 40, 101, 187, 150, 245, 115, 253, 193, 91, 5,
//         45, 148, 91, 74, 184, 111, 200, 144, 36, 203, 76, 229,
//     ];

//     // let result = unsafe { poseidon2::poseidon2_permutation(&[Fr { data: value }]) };
//     let result = unsafe { poseidon2::poseidon2_permutation(&[value]) };
//     println!("{:?}", result);
// }

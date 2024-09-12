use super::{
    bindgen,
    models::Fr,
    traits::{DeserializeBuffer, SerializeBuffer},
    Buffer,
};
use std::ptr;

pub unsafe fn poseidon2_hash(inputs: &[Fr]) -> Fr {
    let mut output: <Fr as DeserializeBuffer>::Slice = [0; 32];
    bindgen::poseidon2_hash(inputs.to_buffer().as_slice().as_ptr(), output.as_mut_ptr());
    Fr::from_buffer(output)
}

// pub unsafe fn poseidon2_hashes(inputs: &[Fr]) -> Vec<Fr> {
//     let num_hashes = inputs.len() / 2;
//     let mut output: Vec<u8> = vec![0; num_hashes * 32];
//     bindgen::poseidon2_hashes(inputs.to_buffer().as_slice().as_ptr(), output.as_mut_ptr());

//     output
//         .chunks(32)
//         .map(|chunk| Fr::from_buffer(chunk.try_into().unwrap()))
//         .collect()
// }

// TODO: unable to get the same output as @aztec/bb.js
// output result always changes...
pub unsafe fn poseidon2_permutation(inputs: &[Fr]) -> Vec<Fr> {
    let mut output_ptr = ptr::null_mut();
    bindgen::poseidon2_permutation(inputs.to_buffer().as_slice().as_ptr(), &mut output_ptr);

    // let output_buffer = Buffer::from_ptr(output_ptr).unwrap();
    let output_buffer = Buffer::from_ptr(output_ptr).unwrap();

    println!("output_buffer: {:?}", output_buffer);

    // let output_buffer_parsed = &output_buffer.data[4..132];
    // println!("output_buffer_parsed: {:?}", output_buffer_parsed);

    // output_buffer_parsed
    //     .to_vec()
    //     .chunks(32)
    //     .map(|chunk| Fr::from_buffer(chunk.try_into().unwrap()))
    //     .collect()

    let asa: Vec<Fr> = Vec::new();
    asa
}

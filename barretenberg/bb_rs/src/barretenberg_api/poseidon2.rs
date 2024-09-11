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

pub unsafe fn poseidon2_hashes(inputs: &[Fr]) -> Fr {
    let mut output: <Fr as DeserializeBuffer>::Slice = [0; 32];
    bindgen::poseidon2_hashes(inputs.to_buffer().as_slice().as_ptr(), output.as_mut_ptr());
    Fr::from_buffer(output)
}

pub unsafe fn poseidon2_permutation(inputs: &[Fr]) -> Vec<u8> {
    // let mut output: <Fr as DeserializeBuffer>::Slice = [0; 32];
    let mut output = ptr::null_mut();
    bindgen::poseidon2_permutation(inputs.to_buffer().as_slice().as_ptr(), &mut output);
    Buffer::from_ptr(output).unwrap().to_vec()
}

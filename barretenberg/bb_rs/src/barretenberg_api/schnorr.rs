use super::{
    bindgen,
    models::{Fq, Fr, Point},
    traits::{DeserializeBuffer, SerializeBuffer},
};

pub unsafe fn schnorr_compute_public_key(private_key: &[u8; 32]) -> [u8; 64] {
    let mut public_key_buf = [0u8; 64];

    bindgen::schnorr_compute_public_key(private_key.as_ptr(), public_key_buf.as_mut_ptr());

    public_key_buf
}

// TODO: doesnt work somehow. getting stuck.
pub unsafe fn schnorr_construct_signature(
    message: &[u8],
    private_key: &[u8], // not &[u8]??
) -> ([u8; 32], [u8; 32]) {
    let mut s = [0; 32];
    let mut e = [0; 32];
    bindgen::schnorr_construct_signature(
        message.as_ptr(),
        private_key.as_ptr(),
        s.as_mut_ptr(),
        e.as_mut_ptr(),
    );
    (s, e)
}

// TODO: doesnt work somehow too. getting stuck.
pub unsafe fn schnorr_verify_signature(
    message: &[u8],
    public_key: &[u8],
    sig_s: &[u8],
    sig_e: &[u8],
) -> bool {
    let mut result = false;
    bindgen::schnorr_verify_signature(
        message.as_ptr(),
        public_key.as_ptr(),
        sig_s.as_ptr(),
        sig_e.as_ptr(),
        &mut result,
    );
    result
}

// TOOD: haven't tried
pub unsafe fn schnorr_multisig_create_multisig_public_key(public_key: &Fq) -> [u8; 128] {
    let mut result = [0; 128];
    bindgen::schnorr_multisig_create_multisig_public_key(
        public_key.to_buffer().as_slice().as_ptr(),
        result.as_mut_ptr(),
    );
    result
}

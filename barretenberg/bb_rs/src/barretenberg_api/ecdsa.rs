use super::{
    bindgen,
    models::{Fr, Point},
    traits::{DeserializeBuffer, SerializeBuffer},
};

pub unsafe fn ecdsa__compute_public_key(private_key: &[u8]) -> [u8; 64] {
    let mut public_key_buf = [0; 64];

    bindgen::ecdsa__compute_public_key(private_key.as_ptr(), public_key_buf.as_mut_ptr());

    public_key_buf
}

pub unsafe fn ecdsa__construct_signature(
    message: &[u8],
    msg_len: usize,
    private_key: &[u8],
) -> ([u8; 32], [u8; 32], u8) {
    let mut output_sig_r = [0; 32];
    let mut output_sig_s = [0; 32];
    let mut output_sig_v = 0u8;
    bindgen::ecdsa__construct_signature(
        message.as_ptr(),
        msg_len,
        private_key.as_ptr(),
        output_sig_r.as_mut_ptr(),
        output_sig_s.as_mut_ptr(),
        &mut output_sig_v,
    );

    (output_sig_r, output_sig_s, output_sig_v)
}

pub unsafe fn ecdsa__recover_public_key_from_signature(
    message: &[u8],
    msg_len: usize,
    sig_r: &[u8],
    sig_s: &[u8],
    sig_v: &mut u8,
) -> [u8; 64] {
    let mut output_pub_key = [0; 64];
    bindgen::ecdsa__recover_public_key_from_signature(
        message.as_ptr(),
        msg_len,
        sig_r.as_ptr(),
        sig_s.as_ptr(),
        &mut *sig_v,
        output_pub_key.as_mut_ptr(),
    );

    output_pub_key
}

pub unsafe fn ecdsa__verify_signature(
    message: &[u8],
    msg_len: usize,
    public_key: &[u8],
    sig_r: &[u8],
    sig_s: &[u8],
    sig_v: u8,
) {
    let mut _sig_v = sig_v;
    bindgen::ecdsa__verify_signature(
        message.as_ptr(),
        msg_len,
        public_key.as_ptr(),
        sig_r.as_ptr(),
        sig_s.as_ptr(),
        &mut _sig_v,
    );
}

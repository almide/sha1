use sha1_crate::{Sha1, Digest};

pub fn native_sha1_hash(data: &Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha1::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

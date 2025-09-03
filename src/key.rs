use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct Key {
    pub id: [u8; 32]
}

impl Key {
    pub fn new(seed: [u8; 32]) -> Self {
        let hashed = Sha256::digest(seed);

        Key {
            id: hashed.into()
        }
    }
}


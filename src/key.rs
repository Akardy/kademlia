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

    pub fn distance(k1: &Self, k2: &Key) -> [u8; 32] {
        let mut d = [0u8; 32];

        for i in 0..32 {
            d[i] = k1.id[i] ^ k2.id[i];
        }

        d
    }
}


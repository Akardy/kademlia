use std::fs;
use std::io::Write;
use rand::rngs::OsRng;
use rand::TryRngCore;

use crate::key::Key;

pub fn import_or_generate_seed(path: String) -> [u8; 32] {
    match fs::read(&path) {
        Ok(seed_vec) if seed_vec.len() == 32 => {
            let mut seed = [0u8; 32];
            seed.copy_from_slice(&seed_vec);
            println!("Seed is imported!");
            seed
        }
        _ => {
            let mut seed = [0u8; 32];
            OsRng.try_fill_bytes(&mut seed).unwrap();
            save_seed(seed, path);
            seed
        }
    }
}

fn save_seed(seed: [u8; 32], path: String) {
    let mut file = fs::File::create(path).expect("[ERROR] Failed to create a seed file.");
    file.write_all(&seed).expect("[ERROR] Failed to save the seed locally.");
}

pub fn distance(k1: &Key, k2: &Key) -> [u8; 32] {
    let mut d = [0u8; 32];

    for i in 0..32 {
        d[i] = k1.id[i] ^ k2.id[i];
    }

    d
}
use std::fs;
use std::io::Write;
use rand::rngs::OsRng;
use rand::TryRngCore;

pub fn import_or_generate_seed() -> [u8; 32] {
    match fs::read("seed.bin") {
        Ok(seed_vec) if seed_vec.len() == 32 => {
            let mut seed = [0u8; 32];
            seed.copy_from_slice(&seed_vec);
            println!("Seed is imported!");
            seed
        }
        _ => {
            let mut seed = [0u8; 32];
            OsRng.try_fill_bytes(&mut seed).unwrap();
            save_seed(seed);
            seed
        }
    }
}

fn save_seed(seed: [u8; 32]) {
    let mut file = fs::File::create("seed.bin").expect("[ERROR] Failed to create a seed file.");
    file.write_all(&seed).expect("[ERROR] Failed to save the seed locally.");
}
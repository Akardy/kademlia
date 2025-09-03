use crate::key::Key;
use crate::utils;

pub struct Node {
    pub id: Key,
}

impl Node {
    pub fn new() -> Self {
        let seed = utils::import_or_generate_seed();
        Node {
            id: Key::new(seed)
        }
    }
}



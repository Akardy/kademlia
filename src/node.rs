use crate::key::Key;
use crate::utils;

pub struct Node {
    pub id: Key,
}

impl Node {
    pub fn new(path: Option<String>) -> Self {

        let seed = utils::import_or_generate_seed(path);
        Node {
            id: Key::new(seed)
        }
    }
}



use crate::key::Key;
pub struct Node {
    id: Key,
}

impl Node {
    pub fn new() -> Self {
        Node {
            id: Key::new()
        }
    }
}



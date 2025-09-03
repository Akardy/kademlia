mod node; 
mod key;
mod utils;

use node::Node;

fn main() {
    let node = Node::new();
    println!("Node ID: {:?}", node.id) 
}

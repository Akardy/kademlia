mod node; 
mod key;
mod utils;

use node::Node;

fn main() {
    let node = Node::new("seed.bin".to_string());
    let other = Node::new("seed2.bin".to_string());

    println!("Node ID: {:?}", node.id);
    println!("Node ID: {:?}", other.id);  
    let distance = utils::distance(&node.id, &other.id);

    println!("Distance: {:?}", distance);
}
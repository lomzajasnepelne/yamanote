mod dummy_l1;
mod node;
mod sim;
mod transparent_aether;

use sim::nodelike;

fn main() {
    println!("Creating the simulation");
    let nodes_ = vec![
        node::Node::new(dummy_l1::DummyL1::new()),
        node::Node::new(dummy_l1::DummyL1::new()),
    ];
    let nodes: Vec<&dyn nodelike::Nodelike> = nodes_
        .iter()
        .map(|n| n as &dyn nodelike::Nodelike)
        .collect();
}

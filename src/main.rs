mod node;
mod nodelike;
mod dummy_l1;

trait Aether<'a> {
    fn new(nodes: &[&'a dyn nodelike::Nodelike]) -> Self;
    fn step(&self);
}

struct TransparentAether<'a> {
    nodes: Vec<&'a dyn nodelike::Nodelike>,
}

impl<'a> Aether<'a> for TransparentAether<'a> {
    fn new(nodes: &[&'a dyn nodelike::Nodelike]) -> Self {
        TransparentAether {
            nodes: nodes.to_vec(),
        }
    }

    fn step(&self) {
        ()
    }
}

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
    let sim = TransparentAether::new(&nodes);
    println!("Advancing the simulation by 1 step");
    sim.step();
}

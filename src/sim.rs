pub mod nodelike;

struct Sim<T> {
    nodes: Vec<Box<dyn nodelike::Nodelike>>,
    aeth: T,
}

impl<T> Sim<T> {
    fn new(nodes: Vec<Box<dyn nodelike::Nodelike>>, aeth: T) -> Self {
        Sim { nodes, aeth }
    }
}

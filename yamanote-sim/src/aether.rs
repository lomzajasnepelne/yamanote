use yamanote_node::node;

pub enum AetherError {
    NodeNotInAether,
}

pub trait Aether {
    fn add(&self, n: node::Node);
    fn remove(&self, n: &node::Node) -> Result<(), AetherError>;
    fn transmit(&self, n: &node::Node) -> Result<(), AetherError>;
}

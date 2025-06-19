use crate::node::{NodeId, NodeReceiveError, NodeSendError};

pub trait Nodelike {
    fn get_connected_nodes(&self) -> Vec<NodeId>;

    fn send_to(&self, id: NodeId) -> Result<(), NodeSendError>;

    fn receive_from(&self, id: NodeId) -> Result<Vec<u8>, NodeReceiveError>;
}

use crate::node;

pub trait Nodelike {
    fn get_connected_nodes<'a>(
        &self,
        node_ids_buf: &'a mut [node::NodeId],
    ) -> &'a [node::NodeId];

    fn send_to(
        &self,
        b: &[u8],
        id: node::NodeId,
    ) -> Result<(), node::NodeSendError>;

    fn receive_from<'a>(
        &self,
        b: &'a mut [u8],
        id: node::NodeId,
    ) -> Result<&'a [u8], node::NodeReceiveError>;
}

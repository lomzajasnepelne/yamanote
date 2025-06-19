use crate::node;
use crate::nodelike;

pub struct DummyL1 {
    tx: Vec<u8>,
    rx: Vec<u8>,
}

impl DummyL1 {
    pub fn new() -> Self {
        Self {
            tx: Vec::new(),
            rx: Vec::new(),
        }
    }

    fn get_tx(&mut self) -> Vec<u8> {
        let ret = self.tx.clone();
        self.tx.clear();
        ret
    }

    fn set_rx(&mut self, b: &[u8]) {
        self.rx.extend_from_slice(b);
    }
}

impl node::l1::L1 for DummyL1 {
    fn send_to_l1(&mut self, b: &[u8]) -> Result<(), node::l1::SendToL1Error> {
        self.rx.extend_from_slice(b);
        Ok(())
    }

    fn receive_from_l1<'a>(
        &mut self,
        b: &'a mut [u8],
    ) -> Result<&'a [u8], node::l1::ReceiveFromL1Error> {
        let to_copy = std::cmp::min(b.len(), self.rx.len());
        let target = &mut b[..to_copy];
        target.copy_from_slice(&self.rx[..to_copy]);
        self.rx.drain(..to_copy);
        Ok(target)
    }
}

impl nodelike::Nodelike for node::Node<DummyL1> {
    fn get_connected_nodes<'a>(
        &self,
        node_ids_buf: &'a mut [node::NodeId],
    ) -> &'a [node::NodeId] {
        node_ids_buf
    }

    fn send_to(
        &self,
        b: &[u8],
        id: node::NodeId,
    ) -> Result<(), node::NodeSendError> {
        Ok(())
    }

    fn receive_from<'a>(
        &self,
        b: &'a mut [u8],
        id: node::NodeId,
    ) -> Result<&'a [u8], node::NodeReceiveError> {
        Ok(b)
    }
}

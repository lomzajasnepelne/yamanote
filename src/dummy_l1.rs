use crate::node::{Node, NodeId, NodeReceiveError, NodeSendError, l1};
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

impl l1::L1 for DummyL1 {
    fn send_to_l1(&mut self, b: &[u8]) -> Result<(), l1::SendToL1Error> {
        self.rx.extend_from_slice(b);
        Ok(())
    }

    fn receive_from_l1<'a>(
        &mut self,
        b: &'a mut [u8],
    ) -> Result<&'a [u8], l1::ReceiveFromL1Error> {
        let to_copy = std::cmp::min(b.len(), self.rx.len());
        let target = &mut b[..to_copy];
        target.copy_from_slice(&self.rx[..to_copy]);
        self.rx.drain(..to_copy);
        Ok(target)
    }
}

impl nodelike::Nodelike for Node<DummyL1> {
    fn get_connected_nodes(&self) -> Vec<NodeId> {
        vec![]
    }

    fn send_to(&self, id: NodeId) -> Result<(), NodeSendError> {
        Ok(())
    }

    fn receive_from(&self, id: NodeId) -> Result<Vec<u8>, NodeReceiveError> {
        Ok(vec![])
    }
}

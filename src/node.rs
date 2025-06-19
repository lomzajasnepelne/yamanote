pub mod l1;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct NodeId(pub u64);

pub enum NodeSendError {
    TargetNodeIdUnknown,
    SendFailed,
}

pub enum NodeReceiveError {
    TargetNodeIdUnknown,
    ReceiveFailed,
    NoData,
}

pub struct Node<T: l1::L1> {
    layer1: T,
}

impl<T: l1::L1> Node<T> {
    pub fn new(layer1: T) -> Self {
        Node { layer1 }
    }
}

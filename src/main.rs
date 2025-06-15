enum SendToL1Error {
    BufferFull,
}

enum ReceiveFromL1Error {
    BufferEmpty,
}

trait L1 {
    fn send_to_l1(&mut self, b: &[u8]) -> Result<(), SendToL1Error>;
    fn receive_from_l1<'a>(
        &mut self,
        b: &'a mut [u8],
    ) -> Result<&'a [u8], ReceiveFromL1Error>;
}

enum NodeSendError {
    TargetNodeIdUnknown,
    SendFailed,
}

enum NodeReceiveError {
    TargetNodeIdUnknown,
    ReceiveFailed,
    NoData,
}

struct Node<T: L1> {
    l1: T,
}

impl<T: L1> Node<T> {
    pub fn new(l1: T) -> Self {
        Node { l1 }
    }
}

struct NodeId(u64);

trait Nodelike {
    fn get_connected_nodes(&self) -> [NodeId];
    fn send_to(&self, b: &[u8], id: NodeId) -> Result<(), NodeSendError>;
    fn receive_from(
        &self,
        b: &mut [u8],
        id: NodeId,
    ) -> Result<&[u8], NodeReceiveError>;
}

struct DummyL1 {
    tx: Vec<u8>,
    rx: Vec<u8>,
}

impl DummyL1 {
    fn get_tx(&mut self) -> Vec<u8> {
        let ret = self.tx.clone();
        self.tx.clear();
        ret
    }

    fn set_rx(&mut self, b: &[u8]) {
        self.rx.extend_from_slice(b);
    }
}

impl L1 for DummyL1 {
    fn send_to_l1(&mut self, b: &[u8]) -> Result<(), SendToL1Error> {
        self.rx.extend_from_slice(b);
        Ok(())
    }

    fn receive_from_l1<'a>(
        &mut self,
        b: &'a mut [u8],
    ) -> Result<&'a [u8], ReceiveFromL1Error> {
        let to_copy = std::cmp::min(b.len(), self.rx.len());
        let target = &mut b[..to_copy];
        target.copy_from_slice(&self.rx[..to_copy]);
        self.rx.drain(..to_copy);
        Ok(target)
    }
}

trait Aether<T: L1> {
    fn new(nodes: &[&dyn Nodelike]);
}

struct Sim {}

impl Sim {
    pub fn new() -> Self {
        Sim {}
    }

    pub fn advance_by(&self, steps: u32) {
        _ = steps
    }
}

fn main() {
    println!("Creating the simulation");
    let sim = Sim::new();
    println!("Advancing the simulation by 10 steps");
    sim.advance_by(10);
}

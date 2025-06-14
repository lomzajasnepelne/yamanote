use yamanote_node::node;

struct L1Stub {}

impl node::l1::L1 for L1Stub {
    fn recv_from_l1(&self, b: &mut [u8]) -> Result<(), node::l1::RecvFromL1Error> {
        _ = b;
        Ok(())
    }

    fn send_to_l1(&self, b: &[u8]) -> Result<(), node::l1::SendToL1Error> {
        _ = b;
        Ok(())
    }
}

#[test]
fn test_add() {
    let l1_stub = L1Stub {};
    let n = node::IdleNode::new(&l1_stub);
    let n = n.start();
    let n = n.stop().unwrap();
}

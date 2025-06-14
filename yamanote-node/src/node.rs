pub mod l1;

pub struct IdleNode<'a> {
    l1_if: &'a (dyn l1::L1 + 'a),
}

pub struct ActiveNode<'a> {
    l1_if: &'a (dyn l1::L1 + 'a),
}

#[derive(Debug)]
pub enum NodeStopError {
    StopTimeout,
}

impl<'a> IdleNode<'a> {
    pub fn new(l1_if: &'a dyn l1::L1) -> Self {
        IdleNode { l1_if }
    }

    pub fn start(self) -> ActiveNode<'a> {
        ActiveNode { l1_if: self.l1_if }
    }
}

impl<'a> ActiveNode<'a> {
    pub fn stop(self) -> Result<IdleNode<'a>, NodeStopError> {
        Ok(IdleNode { l1_if: self.l1_if })
    }
}

pub enum Node<'a> {
    Idle(IdleNode<'a>),
    Active(ActiveNode<'a>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        struct L1Stub {}

        impl l1::L1 for L1Stub {
            fn recv_from_l1(&self, b: &mut [u8]) -> Result<(), l1::RecvFromL1Error> {
                _ = b;
                Ok(())
            }

            fn send_to_l1(&self, b: &[u8]) -> Result<(), l1::SendToL1Error> {
                _ = b;
                Ok(())
            }
        }

        let l1_stub = L1Stub {};
        let n = IdleNode::new(&l1_stub);
        let n = n.start();
        let n = n.stop().unwrap();
    }
}

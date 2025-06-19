use std::{cell::RefCell, collections::{HashMap, VecDeque}, rc::Rc};

use crate::node::NodeId;

#[derive(Debug)]
pub enum RegisterNodeError {
    IdAlreadyRegistered,
}

pub struct L1AetherBuffers {
    pub l1_to_aeth: Rc<RefCell<VecDeque<u8>>>,
    pub aeth_to_l1: Rc<RefCell<VecDeque<u8>>>,
}

pub struct TransparentAether {
    buffers: HashMap<NodeId, L1AetherBuffers>,
}

impl TransparentAether {
    pub fn new() -> Self {
        TransparentAether {
            buffers: HashMap::new(),
        }
    }

    pub fn register_node(&mut self, id: NodeId) -> Result<(), RegisterNodeError> {
        if self.buffers.contains_key(&id) {
            return Err(RegisterNodeError::IdAlreadyRegistered);
        }
        self.buffers.insert(
            id,
            L1AetherBuffers {
                l1_to_aeth: Rc::new(RefCell::new(VecDeque::new())),
                aeth_to_l1: Rc::new(RefCell::new(VecDeque::new())),
            }
        );
        Ok(())
    }

    pub fn get_l1_aether_buffers(&mut self, id: NodeId) -> Option<&L1AetherBuffers> {
        self.buffers.get(&id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_l1_to_aeth_buf_for_single_node_returns_writeable_buf() {
        let mut aeth = TransparentAether::new();
        let id = NodeId(123);
        aeth.register_node(id).unwrap();
        let bufs = aeth.get_l1_aether_buffers(id).unwrap();
        bufs.aeth_to_l1.borrow_mut().push_back(5);
        bufs.l1_to_aeth.borrow_mut().push_back(6);
        assert_eq!(bufs.aeth_to_l1.borrow_mut().pop_back().unwrap(), 5);
        assert_eq!(bufs.l1_to_aeth.borrow_mut().pop_back().unwrap(), 6);
    }

    #[test]
    fn get_l1_to_aeth_buf_returns_none_if_node_not_registered() {
        let mut aeth = TransparentAether::new();
        let id = NodeId(123);
        let bufs = aeth.get_l1_aether_buffers(id);
        assert!(bufs.is_none());
    }
}
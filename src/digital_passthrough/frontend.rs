use std::{cell::RefCell, collections::VecDeque, rc::Rc};

pub struct Buffers {
    pub l1_to_aeth: Rc<RefCell<VecDeque<u8>>>,
    pub aeth_to_l1: Rc<RefCell<VecDeque<u8>>>,
}

impl Buffers {
    pub fn new() -> Self {
        Self {
            l1_to_aeth: Rc::new(RefCell::new(VecDeque::new())),
            aeth_to_l1: Rc::new(RefCell::new(VecDeque::new())),
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            l1_to_aeth: Rc::clone(&self.l1_to_aeth),
            aeth_to_l1: Rc::clone(&self.aeth_to_l1),
        }
    }
}

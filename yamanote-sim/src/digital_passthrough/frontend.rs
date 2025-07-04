use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};

pub struct Buffers {
    pub l1_to_aeth: Arc<RwLock<VecDeque<u8>>>,
    pub aeth_to_l1: Arc<RwLock<VecDeque<u8>>>,
}

impl Default for Buffers {
    fn default() -> Self {
        Self::new()
    }
}

impl Buffers {
    pub fn new() -> Self {
        Self {
            l1_to_aeth: Arc::new(RwLock::new(VecDeque::new())),
            aeth_to_l1: Arc::new(RwLock::new(VecDeque::new())),
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            l1_to_aeth: Arc::clone(&self.l1_to_aeth),
            aeth_to_l1: Arc::clone(&self.aeth_to_l1),
        }
    }
}

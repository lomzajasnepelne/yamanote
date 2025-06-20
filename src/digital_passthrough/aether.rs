use crate::digital_passthrough::frontend;

struct Aether {
    nodes_bufs: Vec<frontend::Buffers>,
}

impl Aether {
    pub fn new() -> Self {
        Self {
            nodes_bufs: Vec::new(),
        }
    }

    pub fn register_node(&mut self, bufs: &frontend::Buffers) {
        self.nodes_bufs.push(frontend::Buffers::clone(bufs));
    }

    pub fn propagate(&self) {
        for (i, bufs) in self.nodes_bufs.iter().enumerate() {
            let mut l1_to_aeth = bufs.l1_to_aeth.borrow_mut();
            for (_, target) in
                self.nodes_bufs.iter().enumerate().filter(|(j, _)| i != *j)
            {
                target
                    .aeth_to_l1
                    .borrow_mut()
                    .extend(l1_to_aeth.as_slices().0);
                target
                    .aeth_to_l1
                    .borrow_mut()
                    .extend(l1_to_aeth.as_slices().1);
            }
            l1_to_aeth.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn propagate_clears_tx_buffer() {
        let mut aeth = Aether::new();
        let sender_frontend = frontend::Buffers::new();
        aeth.register_node(&sender_frontend);
        sender_frontend.l1_to_aeth.borrow_mut().extend([1, 2, 3, 4]);
        aeth.propagate();
        sender_frontend.l1_to_aeth.borrow_mut().make_contiguous();
        assert_eq!(sender_frontend.l1_to_aeth.borrow().as_slices().0, []);
    }

    #[test]
    fn propagate_copies_tx_from_one_node_to_every_other_rx() {
        let mut aeth = Aether::new();
        let sender_frontend = frontend::Buffers::new();
        let receiver_one_frontend = frontend::Buffers::new();
        let receiver_two_frontend = frontend::Buffers::new();
        aeth.register_node(&sender_frontend);
        aeth.register_node(&receiver_one_frontend);
        aeth.register_node(&receiver_two_frontend);
        sender_frontend.l1_to_aeth.borrow_mut().extend([1, 2, 3, 4]);
        aeth.propagate();
        sender_frontend.aeth_to_l1.borrow_mut().make_contiguous();
        receiver_one_frontend
            .aeth_to_l1
            .borrow_mut()
            .make_contiguous();
        receiver_two_frontend
            .aeth_to_l1
            .borrow_mut()
            .make_contiguous();
        assert_eq!(sender_frontend.aeth_to_l1.borrow().as_slices().0, []);
        assert_eq!(
            receiver_one_frontend.aeth_to_l1.borrow().as_slices().0,
            [1, 2, 3, 4]
        );
        assert_eq!(
            receiver_two_frontend.aeth_to_l1.borrow().as_slices().0,
            [1, 2, 3, 4]
        );
    }
}

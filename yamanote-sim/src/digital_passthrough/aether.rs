use crate::digital_passthrough::frontend;

pub struct Aether {
    nodes_bufs: Vec<frontend::Buffers>,
}

impl Default for Aether {
    fn default() -> Self {
        Self::new()
    }
}

impl Aether {
    pub fn new() -> Self {
        Self {
            nodes_bufs: Vec::new(),
        }
    }

    pub fn register_l1(&mut self, bufs: &frontend::Buffers) {
        self.nodes_bufs.push(frontend::Buffers::clone(bufs));
    }

    pub fn propagate(&self) {
        for (i, bufs) in self.nodes_bufs.iter().enumerate() {
            let mut l1_to_aeth = bufs.l1_to_aeth.write().unwrap();
            for (_, target) in
                self.nodes_bufs.iter().enumerate().filter(|(j, _)| i != *j)
            {
                target
                    .aeth_to_l1
                    .write()
                    .unwrap()
                    .extend(l1_to_aeth.as_slices().0);
                target
                    .aeth_to_l1
                    .write()
                    .unwrap()
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
        aeth.register_l1(&sender_frontend);
        sender_frontend
            .l1_to_aeth
            .write()
            .unwrap()
            .extend([1, 2, 3, 4]);
        aeth.propagate();
        sender_frontend
            .l1_to_aeth
            .write()
            .unwrap()
            .make_contiguous();
        assert_eq!(
            sender_frontend.l1_to_aeth.read().unwrap().as_slices().0,
            []
        );
    }

    #[test]
    fn propagate_copies_tx_from_one_node_to_every_other_rx() {
        let mut aeth = Aether::new();
        let sender_frontend = frontend::Buffers::new();
        let receiver_one_frontend = frontend::Buffers::new();
        let receiver_two_frontend = frontend::Buffers::new();
        aeth.register_l1(&sender_frontend);
        aeth.register_l1(&receiver_one_frontend);
        aeth.register_l1(&receiver_two_frontend);
        sender_frontend
            .l1_to_aeth
            .write()
            .unwrap()
            .extend([1, 2, 3, 4]);
        aeth.propagate();
        sender_frontend
            .aeth_to_l1
            .write()
            .unwrap()
            .make_contiguous();
        receiver_one_frontend
            .aeth_to_l1
            .write()
            .unwrap()
            .make_contiguous();
        receiver_two_frontend
            .aeth_to_l1
            .write()
            .unwrap()
            .make_contiguous();
        assert_eq!(
            sender_frontend.aeth_to_l1.read().unwrap().as_slices().0,
            []
        );
        assert_eq!(
            receiver_one_frontend
                .aeth_to_l1
                .read()
                .unwrap()
                .as_slices()
                .0,
            [1, 2, 3, 4]
        );
        assert_eq!(
            receiver_two_frontend
                .aeth_to_l1
                .read()
                .unwrap()
                .as_slices()
                .0,
            [1, 2, 3, 4]
        );
    }
}

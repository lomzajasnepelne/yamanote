mod aether;
mod frontend;
mod l1;

pub use aether::Aether;

pub fn create_l1() -> (l1::L1, frontend::Buffers) {
    let bufs = frontend::Buffers::new();
    (l1::L1::new(&bufs), bufs)
}

#[cfg(test)]
mod tests {
    use crate::node::l1::L1 as node_L1;

    use super::*;

    #[test]
    fn send_to_l1_and_propagate_and_receive_from_l1_copies_data_from_sender_to_receiver()
     {
        let mut aeth = aether::Aether::new();
        let (mut sender_l1, sender_frontend) = create_l1();
        let (mut receiver_l1, receiver_frontend) = create_l1();
        aeth.register_node(&sender_frontend);
        aeth.register_node(&receiver_frontend);
        sender_l1.send_to_l1(&[1, 2, 3, 4]).unwrap();
        aeth.propagate();
        let mut buf = [0 as u8; 5];
        assert_eq!(
            receiver_l1.receive_from_l1(&mut buf).unwrap(),
            [1, 2, 3, 4]
        );
        assert_eq!(buf, [1, 2, 3, 4, 0]);
        assert_eq!(sender_l1.receive_from_l1(&mut buf).unwrap(), []);
    }
}

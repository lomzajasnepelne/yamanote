use crate::digital_passthrough::frontend;
use yamanote_node::l1;

pub struct L1 {
    bufs: frontend::Buffers,
}

impl L1 {
    pub fn new(bufs: &frontend::Buffers) -> Self {
        L1 {
            bufs: frontend::Buffers::clone(bufs),
        }
    }
}

impl l1::L1 for L1 {
    fn send_to_l1(&mut self, b: &[u8]) -> Result<(), l1::SendToL1Error> {
        self.bufs.l1_to_aeth.write().unwrap().extend(b);
        Ok(())
    }

    fn receive_from_l1<'a>(
        &mut self,
        b: &'a mut [u8],
    ) -> Result<&'a [u8], l1::ReceiveFromL1Error> {
        let mut aeth_to_l1 = self.bufs.aeth_to_l1.write().unwrap();
        let to_copy = std::cmp::min(b.len(), aeth_to_l1.len());
        for b_i in b.iter_mut().take(to_copy) {
            *b_i = aeth_to_l1.pop_front().unwrap();
        }
        Ok(&b[..to_copy])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn send_to_l1_writes_to_l1_to_aether_buf() {
        let bufs = frontend::Buffers::new();
        let mut l1 = L1::new(&bufs);
        l1::L1::send_to_l1(&mut l1, &[1, 2]).unwrap();
        l1::L1::send_to_l1(&mut l1, &[3, 4]).unwrap();
        bufs.l1_to_aeth.write().unwrap().make_contiguous();
        assert_eq!(bufs.l1_to_aeth.read().unwrap().as_slices().0, [1, 2, 3, 4]);
    }

    #[test]
    fn receive_from_l1_reads_from_aeth_to_l1_buf() {
        let bufs = frontend::Buffers::new();
        let mut l1 = L1::new(&bufs);
        bufs.aeth_to_l1.write().unwrap().extend([1, 2, 3, 4]);
        let mut rx_buf = [0_u8; 2];
        let rx = l1::L1::receive_from_l1(&mut l1, &mut rx_buf).unwrap();
        assert_eq!(rx, [1, 2]);
        let rx = l1::L1::receive_from_l1(&mut l1, &mut rx_buf).unwrap();
        assert_eq!(rx, [3, 4]);
        let rx = l1::L1::receive_from_l1(&mut l1, &mut rx_buf).unwrap();
        assert_eq!(rx, []);
    }
}

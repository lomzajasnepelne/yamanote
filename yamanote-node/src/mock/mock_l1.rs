use std::collections::VecDeque;

use crate::l1;

pub struct MockL1 {
    pub send_to_l1_return_value: Result<(), l1::SendToL1Error>,
    pub send_to_l1_call_args: Vec<Vec<u8>>,
    pub receive_from_l1_return_value: Result<VecDeque<u8>, l1::ReceiveFromL1Error>,
}

impl MockL1 {
    pub fn new() -> Self {
        Self {
            send_to_l1_return_value: Ok(()),
            send_to_l1_call_args: Vec::new(),
            receive_from_l1_return_value: Ok(VecDeque::new()),
        }
    }
}

impl l1::L1 for MockL1 {
    fn send_to_l1(&mut self, b: &[u8]) -> Result<(), l1::SendToL1Error> {
        self.send_to_l1_call_args.push(Vec::from(b));
        Ok(())
    }

    fn receive_from_l1<'a>(
        &mut self,
        b: &'a mut [u8],
    ) -> Result<&'a [u8], l1::ReceiveFromL1Error> {
        match &mut self.receive_from_l1_return_value {
            Ok(v) => {
                let to_copy = std::cmp::min(b.len(), v.len());
                for b_i in b.iter_mut().take(to_copy) {
                    *b_i = v.pop_front().unwrap();
                }
                Ok(&b[..to_copy])
            }
            Err(e) => Err(*e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_mock() {
        _ = MockL1::new();
    }
}

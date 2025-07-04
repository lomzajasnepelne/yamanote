use std::collections::VecDeque;

use crate::l1;

pub struct L1Fake {
    send_to_l1_error: Option<l1::SendToL1Error>,
    receive_from_l1_error: Option<l1::ReceiveFromL1Error>,
    sent: VecDeque<u8>,
    to_be_received: VecDeque<u8>,
}

impl Default for L1Fake {
    fn default() -> Self {
        Self::new()
    }
}

impl L1Fake {
    pub fn new() -> Self {
        Self {
            send_to_l1_error: None,
            receive_from_l1_error: None,
            sent: VecDeque::new(),
            to_be_received: VecDeque::new(),
        }
    }

    pub fn succeed_on_send_to_l1(&mut self) {
        self.send_to_l1_error = None;
    }

    pub fn fail_on_send_to_l1(&mut self, e: l1::SendToL1Error) {
        self.send_to_l1_error = Some(e);
    }

    pub fn succeed_on_receive_from_l1(&mut self) {
        self.receive_from_l1_error = None;
    }

    pub fn fail_on_receive_from_l1(&mut self, e: l1::ReceiveFromL1Error) {
        self.receive_from_l1_error = Some(e);
    }

    pub fn get_sent_to_l1(&mut self) -> Vec<u8> {
        let ret = self.sent.clone().into_iter().collect();
        self.sent.clear();
        ret
    }

    pub fn set_to_be_received_from_l1(&mut self, b: &[u8]) {
        self.to_be_received.extend(b);
    }
}

impl l1::L1 for L1Fake {
    fn send_to_l1(&mut self, b: &[u8]) -> Result<(), l1::SendToL1Error> {
        if let Some(e) = self.send_to_l1_error {
            return Err(e);
        }
        self.sent.extend(b);
        Ok(())
    }

    fn receive_from_l1<'a>(
        &mut self,
        b: &'a mut [u8],
    ) -> Result<&'a [u8], l1::ReceiveFromL1Error> {
        if let Some(e) = self.receive_from_l1_error {
            return Err(e);
        }
        let to_copy = std::cmp::min(b.len(), self.to_be_received.len());
        for b_i in b.iter_mut().take(to_copy) {
            *b_i = self.to_be_received.pop_front().unwrap();
        }
        Ok(&b[..to_copy])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn send_to_l1_fails_when_fail_on_send_to_l1_called() {
        let mut fake = L1Fake::new();
        fake.fail_on_send_to_l1(l1::SendToL1Error::IoError);
        assert_eq!(
            l1::L1::send_to_l1(&mut fake, &[]),
            Err(l1::SendToL1Error::IoError)
        );
        fake.fail_on_send_to_l1(l1::SendToL1Error::BufferFull);
        assert_eq!(
            l1::L1::send_to_l1(&mut fake, &[]),
            Err(l1::SendToL1Error::BufferFull)
        );
    }

    #[test]
    fn send_to_l1_succeeds_when_succeed_on_send_to_l1_called() {
        let mut fake = L1Fake::new();
        fake.fail_on_send_to_l1(l1::SendToL1Error::IoError);
        fake.succeed_on_send_to_l1();
        l1::L1::send_to_l1(&mut fake, &[]).unwrap();
    }

    #[test]
    fn send_to_l1_succeeds_by_default() {
        let mut fake = L1Fake::new();
        l1::L1::send_to_l1(&mut fake, &[]).unwrap();
    }

    #[test]
    fn receive_from_l1_fails_when_fail_on_receive_from_l1_called() {
        let mut fake = L1Fake::new();
        fake.fail_on_receive_from_l1(l1::ReceiveFromL1Error::IoError);
        let mut buf = [0; 10];
        assert_eq!(
            l1::L1::receive_from_l1(&mut fake, &mut buf),
            Err(l1::ReceiveFromL1Error::IoError)
        );
        fake.succeed_on_receive_from_l1();
        let mut buf = [0; 10];
        l1::L1::receive_from_l1(&mut fake, &mut buf).unwrap();
    }

    #[test]
    fn receive_from_l1_succeeds_when_succeed_on_receive_from_l1_called() {
        let mut fake = L1Fake::new();
        fake.fail_on_receive_from_l1(l1::ReceiveFromL1Error::IoError);
        fake.succeed_on_receive_from_l1();
        let mut buf = [0; 10];
        l1::L1::receive_from_l1(&mut fake, &mut buf).unwrap();
    }

    #[test]
    fn receive_from_l1_succeeds_by_default() {
        let mut fake = L1Fake::new();
        let mut buf = [0; 10];
        l1::L1::receive_from_l1(&mut fake, &mut buf).unwrap();
    }

    #[test]
    fn get_sent_to_l1_returns_data_passed_to_send_to_l1() {
        let mut fake = L1Fake::new();
        l1::L1::send_to_l1(&mut fake, &[1, 2]).unwrap();
        l1::L1::send_to_l1(&mut fake, &[3, 4]).unwrap();
        assert_eq!(fake.get_sent_to_l1(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn receive_from_l1_returns_data_passed_to_set_to_be_received_from_l1() {
        let mut fake = L1Fake::new();
        fake.set_to_be_received_from_l1(&[1, 2, 3, 4]);
        let mut buf = [0; 2];
        assert_eq!(
            l1::L1::receive_from_l1(&mut fake, &mut buf).unwrap(),
            [1, 2]
        );
        assert_eq!(buf, [1, 2]);
        assert_eq!(
            l1::L1::receive_from_l1(&mut fake, &mut buf).unwrap(),
            [3, 4]
        );
        assert_eq!(buf, [3, 4]);
        buf = [0, 0];
        assert_eq!(l1::L1::receive_from_l1(&mut fake, &mut buf).unwrap(), []);
        assert_eq!(buf, [0, 0]);
    }
}

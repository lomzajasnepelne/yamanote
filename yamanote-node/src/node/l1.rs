pub enum RecvFromL1Error {
    RxBufferEmpty,
}

pub enum SendToL1Error {
    TxBufferFull,
}

pub trait L1 {
    fn recv_from_l1(&self, b: &mut [u8]) -> Result<(), RecvFromL1Error>;
    fn send_to_l1(&self, b: &[u8]) -> Result<(), SendToL1Error>;
}

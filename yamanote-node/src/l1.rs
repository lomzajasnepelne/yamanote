#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SendToL1Error {
    BufferFull,
    IoError,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReceiveFromL1Error {
    IoError,
}

pub trait L1 {
    fn send_to_l1(&mut self, b: &[u8]) -> Result<(), SendToL1Error>;
    fn receive_from_l1<'a>(
        &mut self,
        b: &'a mut [u8],
    ) -> Result<&'a [u8], ReceiveFromL1Error>;
}

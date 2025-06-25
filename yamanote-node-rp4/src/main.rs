use std::{
    sync::{Arc, Mutex},
    thread, time,
};

struct L1;

impl L1 {
    pub fn new() -> Self {
        Self {}
    }
}

impl yamanote_node::l1::L1 for L1 {
    fn send_to_l1(
        &mut self,
        _b: &[u8],
    ) -> Result<(), yamanote_node::l1::SendToL1Error> {
        Ok(())
    }

    fn receive_from_l1<'a>(
        &mut self,
        _b: &'a mut [u8],
    ) -> Result<&'a [u8], yamanote_node::l1::ReceiveFromL1Error> {
        Ok(&[])
    }
}

struct Sys {
    timer_start: time::Instant,
}

impl Sys {
    pub fn new() -> Self {
        Sys {
            timer_start: time::Instant::now(),
        }
    }
}

impl yamanote_node::sys::Sys for Sys {
    fn lockstep_start(&mut self) -> yamanote_node::sys::ShouldExit {
        false
    }

    fn lockstep_end(&mut self) {}

    fn get_time_mono(&self) -> time::Duration {
        self.timer_start.elapsed()
    }
}

fn main() {
    let l1 = L1::new();
    let sys = Sys::new();
    let abort = Arc::new(Mutex::new(false));
    let abort_move = Arc::clone(&abort);
    let handle = thread::spawn(move || {
        yamanote_node::run(abort_move, l1, sys);
    });
    *abort.lock().unwrap() = true;
    handle.join().unwrap();
}

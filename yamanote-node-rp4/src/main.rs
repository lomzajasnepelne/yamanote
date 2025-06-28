use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
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
    exit_handle: Arc<AtomicBool>,
}

impl Sys {
    pub fn new(exit_handle: Arc<AtomicBool>) -> Self {
        Sys {
            timer_start: time::Instant::now(),
            exit_handle,
        }
    }
}

impl yamanote_node::sys::Sys for Sys {
    fn lockstep_start(&mut self) {}

    fn should_exit(&self) -> bool {
        self.exit_handle.load(Ordering::Relaxed)
    }

    fn lockstep_end(&mut self) {}

    fn get_time_mono(&self) -> time::Duration {
        self.timer_start.elapsed()
    }
}

fn main() {
    let exit_handle = Arc::new(AtomicBool::new(false));
    let l1 = L1::new();
    let sys = Sys::new(Arc::clone(&exit_handle));
    let handle = thread::spawn(move || {
        yamanote_node::run(l1, sys);
    });
    exit_handle.store(true, Ordering::Relaxed);
    handle.join().unwrap();
}

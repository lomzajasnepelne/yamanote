use std::sync::{Arc, Mutex};

pub mod l1;
pub mod sys;

pub fn run(
    abort: Arc<Mutex<bool>>,
    mut l1: impl l1::L1,
    mut sys: impl sys::Sys,
) {
    loop {
        let exit_requested_by_simulation = sys.lockstep_start();
        if exit_requested_by_simulation {
            break;
        }
        let exit_requested_by_handle = *abort.lock().unwrap();
        if exit_requested_by_handle {
            break;
        }
        let mut b = [0; 4];
        let rcvd = l1.receive_from_l1(&mut b).unwrap();
        l1.send_to_l1(rcvd).unwrap();

        sys.lockstep_end();
    }
}

#[cfg(test)]
pub mod fake;

#[cfg(test)]
mod tests {
    use super::*;

    use crate::fake::l1_fake;

    #[test]
    fn run_and_exit_via_lockstep() {
        let l1 = l1_fake::L1Fake::new();
        let mut sys = sys::MockSys::new();
        let abort = Arc::new(Mutex::new(false));
        sys.expect_lockstep_start().return_const(true);
        run(Arc::clone(&abort), l1, sys);
    }

    #[test]
    fn run_and_abort() {
        let l1 = l1_fake::L1Fake::new();
        let mut sys = sys::MockSys::new();
        let abort = Arc::new(Mutex::new(true));
        sys.expect_lockstep_start().return_const(false);
        run(Arc::clone(&abort), l1, sys);
    }
}

use std::sync::{Arc, Mutex};

use crate::node::{l1, sys};

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

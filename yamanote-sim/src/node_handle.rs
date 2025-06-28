use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
};

use yamanote_node::{l1, run, sys};

pub struct NodeHandle {
    abort: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl NodeHandle {
    pub fn create(
        layer1: impl l1::L1 + Send + 'static,
        system: impl sys::Sys + Send + 'static,
    ) -> Self {
        let abort = Arc::new(AtomicBool::new(false));
        let abort_move = Arc::clone(&abort);
        let handle = thread::spawn(move || {
            run(abort_move, layer1, system);
        });
        NodeHandle {
            abort,
            handle: Some(handle),
        }
    }
}

impl Drop for NodeHandle {
    fn drop(&mut self) {
        self.abort.store(true, Ordering::Relaxed);
        self.handle.take().unwrap().join().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::time;

    use super::*;

    struct DummyL1;

    impl l1::L1 for DummyL1 {
        fn send_to_l1(&mut self, _: &[u8]) -> Result<(), l1::SendToL1Error> {
            Ok(())
        }

        fn receive_from_l1<'a>(
            &mut self,
            _: &'a mut [u8],
        ) -> Result<&'a [u8], l1::ReceiveFromL1Error> {
            Ok(&[])
        }
    }

    struct DummySys;

    impl sys::Sys for DummySys {
        fn lockstep_start(&mut self) -> sys::ShouldExit {
            false
        }

        fn lockstep_end(&mut self) {}

        fn get_time_mono(&self) -> time::Duration {
            time::Duration::ZERO
        }
    }

    #[test]
    fn create_and_drop_does_not_panic() {
        let dummy_l1 = DummyL1;
        let dummy_sys = DummySys;
        let node = NodeHandle::create(dummy_l1, dummy_sys);
        std::mem::drop(node);
    }
}

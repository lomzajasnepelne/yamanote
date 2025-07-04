use std::thread;

use yamanote_node::{l1, run, sys};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SimNodeId(pub u64);

pub struct NodeHandle {
    sim_node_id: SimNodeId,
}

impl NodeHandle {
    pub fn create(
        layer1: impl l1::L1 + Send + 'static,
        system: impl sys::Sys + Send + 'static,
        sim_node_id: SimNodeId,
    ) -> (Self, thread::JoinHandle<()>) {
        let h = thread::spawn(move || {
            run(layer1, system);
        });
        (NodeHandle { sim_node_id }, h)
    }

    pub fn sim_node_id(&self) -> SimNodeId {
        self.sim_node_id
    }
}

#[cfg(test)]
mod tests {
    use std::{
        sync::{
            Arc,
            atomic::{AtomicBool, Ordering},
        },
        time,
    };

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

    struct DummySys {
        exit_handle: Arc<AtomicBool>,
    }

    impl DummySys {
        pub fn new(exit_handle: Arc<AtomicBool>) -> Self {
            Self { exit_handle }
        }
    }

    impl sys::Sys for DummySys {
        fn lockstep_start(&mut self) {}

        fn should_exit(&self) -> bool {
            self.exit_handle.load(Ordering::Relaxed)
        }

        fn lockstep_end(&mut self) {}

        fn get_time_mono(&self) -> time::Duration {
            time::Duration::ZERO
        }
    }

    #[test]
    fn create_and_stop_does_not_panic() {
        let exit_handle = Arc::new(AtomicBool::new(false));
        let dummy_l1 = DummyL1;
        let dummy_sys = DummySys::new(Arc::clone(&exit_handle));
        let (_, jh) = NodeHandle::create(dummy_l1, dummy_sys, SimNodeId(0));
        exit_handle.store(true, Ordering::Relaxed);
        jh.join().unwrap();
    }
}

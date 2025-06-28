mod aether;
mod frontend;
mod l1;

use std::{collections::HashMap, time};

use crate::{
    node_handle::NodeHandle,
    sys_fake::{self, SysFakeCtl},
};

pub use aether::Aether;

pub struct Sim {
    aether: aether::Aether,
    node_ctls: HashMap<u64, SysFakeCtl>,
    last_sim_node_id: u64,
    sim_time: time::Duration,
}

impl Default for Sim {
    fn default() -> Self {
        Self::new()
    }
}

impl Sim {
    pub fn new() -> Self {
        Self {
            aether: aether::Aether::new(),
            node_ctls: HashMap::new(),
            last_sim_node_id: 0,
            sim_time: time::Duration::ZERO,
        }
    }

    pub fn create_node(&mut self) -> NodeHandle {
        let (l1, bufs) = Sim::create_l1();
        self.aether.register_l1(&bufs);
        let (sys_fake, sys_fake_ctl) = sys_fake::create_sys_fake();
        let node_id = self.last_sim_node_id + 1;
        self.last_sim_node_id = node_id;
        let n = NodeHandle::create(l1, sys_fake, node_id);
        self.node_ctls.insert(node_id, sys_fake_ctl);
        n
    }

    pub fn destroy_node(&mut self, handle: NodeHandle) {
        self.node_ctls.remove(&handle.sim_node_id()).unwrap();
    }

    pub fn step(&mut self) {
        self.sim_time += time::Duration::from_millis(1);
        for ctl in self.node_ctls.values_mut() {
            ctl.step_run(sys_fake::StepParams {
                time: self.sim_time,
            });
        }
        for ctl in self.node_ctls.values_mut() {
            ctl.step_join();
        }
        self.aether.propagate();
    }

    fn create_l1() -> (l1::L1, frontend::Buffers) {
        let bufs = frontend::Buffers::new();
        (l1::L1::new(&bufs), bufs)
    }
}

impl Drop for Sim {
    fn drop(&mut self) {
        for ctl in self.node_ctls.values_mut() {
            ctl.stop_node();
        }
    }
}

#[cfg(test)]
mod tests {
    use yamanote_node::l1::L1 as node_L1;

    use super::*;

    #[test]
    fn send_to_l1_and_propagate_and_receive_from_l1_copies_data_from_sender_to_receiver()
     {
        let mut aeth = aether::Aether::new();
        let (mut sender_l1, sender_frontend) = Sim::create_l1();
        let (mut receiver_l1, receiver_frontend) = Sim::create_l1();
        aeth.register_l1(&sender_frontend);
        aeth.register_l1(&receiver_frontend);
        sender_l1.send_to_l1(&[1, 2, 3, 4]).unwrap();
        aeth.propagate();
        let mut buf = [0_u8; 5];
        assert_eq!(
            receiver_l1.receive_from_l1(&mut buf).unwrap(),
            [1, 2, 3, 4]
        );
        assert_eq!(buf, [1, 2, 3, 4, 0]);
        assert_eq!(sender_l1.receive_from_l1(&mut buf).unwrap(), []);
    }

    #[test]
    fn destroy_node_does_not_panic() {
        let mut sim = Sim::new();
        let handle = sim.create_node();
        sim.destroy_node(handle);
    }

    #[test]
    fn destroy_sim_without_destroy_node_does_not_panic() {
        let mut sim = Sim::new();
        let _handle = sim.create_node();
        std::mem::drop(sim);
    }

    #[test]
    fn step_does_not_panic() {
        let mut sim = Sim::new();
        let _handle1 = sim.create_node();
        let _handle2 = sim.create_node();
        sim.step();
    }

    #[test]
    #[should_panic]
    fn step_panics_when_handle_dropped() {
        let mut sim = Sim::new();
        sim.create_node();
        sim.step();
    }
}

mod aether;
mod frontend;
mod l1;

use std::{collections::HashMap, thread, time};

use crate::{
    node_handle::{NodeHandle, SimNodeId},
    sys_fake::{self, SysFakeCtl},
};

pub use aether::Aether;

struct SimNodeCtl {
    pub thread_handle: thread::JoinHandle<()>,
    pub sys_fake_ctl: SysFakeCtl,
}

pub struct Sim {
    aether: aether::Aether,
    node_ctls: HashMap<SimNodeId, SimNodeCtl>,
    last_sim_node_id: SimNodeId,
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
            last_sim_node_id: SimNodeId(0),
            sim_time: time::Duration::ZERO,
        }
    }

    pub fn create_node(&mut self) -> NodeHandle {
        let l1 = self.create_l1();
        let (sys_fake, sys_fake_ctl) = sys_fake::create_sys_fake();
        let node_id = self.generate_node_id();
        let (node_handle, thread_handle) =
            NodeHandle::create(l1, sys_fake, node_id);
        let node_ctl = SimNodeCtl {
            thread_handle,
            sys_fake_ctl,
        };
        self.node_ctls.insert(node_id, node_ctl);
        node_handle
    }

    pub fn step(&mut self) {
        self.sim_time += time::Duration::from_millis(1);
        for ctl in self.node_ctls.values_mut() {
            ctl.sys_fake_ctl.step_run(sys_fake::StepParams {
                time: self.sim_time,
            });
        }
        for ctl in self.node_ctls.values_mut() {
            ctl.sys_fake_ctl.step_join();
        }
        self.aether.propagate();
    }

    fn create_l1(&mut self) -> l1::L1 {
        let bufs = frontend::Buffers::new();
        self.aether.register_l1(&bufs);
        l1::L1::new(&bufs)
    }

    fn generate_node_id(&mut self) -> SimNodeId {
        let node_id = SimNodeId(self.last_sim_node_id.0 + 1);
        self.last_sim_node_id = node_id;
        node_id
    }

    fn destroy_node_by_node_ctl(&mut self, mut ctl: SimNodeCtl) {
        ctl.sys_fake_ctl.stop_node();
        ctl.thread_handle.join().unwrap();
    }
}

impl Drop for Sim {
    fn drop(&mut self) {
        let empty = HashMap::new();
        let node_ctls = std::mem::replace(&mut self.node_ctls, empty);
        for ctl in node_ctls.into_values() {
            self.destroy_node_by_node_ctl(ctl);
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
        let mut sim = Sim::new();
        let mut sender_l1 = sim.create_l1();
        let mut receiver_l1 = sim.create_l1();
        sender_l1.send_to_l1(&[1, 2, 3, 4]).unwrap();
        sim.aether.propagate();
        let mut buf = [0_u8; 5];
        assert_eq!(
            receiver_l1.receive_from_l1(&mut buf).unwrap(),
            [1, 2, 3, 4]
        );
        assert_eq!(buf, [1, 2, 3, 4, 0]);
        assert_eq!(sender_l1.receive_from_l1(&mut buf).unwrap(), []);
    }

    #[test]
    fn implicit_drop_does_not_panic() {
        let mut sim = Sim::new();
        let _handle = sim.create_node();
    }

    #[test]
    fn explicit_drop_does_not_panic() {
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
    fn step_does_not_panic_when_handle_dropped_but_sim_not_dropped() {
        let mut sim = Sim::new();
        let handle = sim.create_node();
        std::mem::drop(handle);
        sim.step();
    }
}

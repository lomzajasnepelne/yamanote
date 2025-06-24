use std::time;

use yamanote_sim::{digital_passthrough, node_handle, sys_fake};

fn create_node(
    aeth: &mut digital_passthrough::Aether,
) -> (node_handle::NodeHandle, sys_fake::SysFakeCtl) {
    let (l1, bufs) = digital_passthrough::create_l1();
    aeth.register_node(&bufs);
    let (fake, ctl) = sys_fake::create_sys_fake();
    let n = node_handle::NodeHandle::create(l1, fake);
    (n, ctl)
}

fn destroy_node(_node: node_handle::NodeHandle, _ctl: sys_fake::SysFakeCtl) {}

fn sim_step(
    ctls: &mut [&mut sys_fake::SysFakeCtl],
    aeth: &mut digital_passthrough::Aether,
    curr_time: &mut time::Duration,
) {
    *curr_time += time::Duration::from_millis(1);
    for ctl in ctls {
        ctl.step_run(sys_fake::StepParams { time: *curr_time });
        ctl.step_join();
    }
    aeth.propagate();
}

#[test]
fn test_run_sim() {
    println!("Creating the simulation");
    let mut aeth = digital_passthrough::Aether::new();
    let (node_a, mut ctl_a) = create_node(&mut aeth);
    let (node_b, mut ctl_b) = create_node(&mut aeth);
    let mut sim_time = time::Duration::from_millis(1);
    println!("Step one");
    sim_step(&mut [&mut ctl_a, &mut ctl_b], &mut aeth, &mut sim_time);
    println!("Step two");
    sim_step(&mut [&mut ctl_a, &mut ctl_b], &mut aeth, &mut sim_time);
    println!("Destroying the simulation");
    destroy_node(node_a, ctl_a);
    destroy_node(node_b, ctl_b);
}

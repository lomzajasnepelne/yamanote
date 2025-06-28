use yamanote_sim::digital_passthrough;

#[test]
fn test_run_sim() {
    println!("Creating the simulation");
    let mut sim = digital_passthrough::Sim::new();
    let node_a = sim.create_node();
    let node_b = sim.create_node();
    println!("Step one");
    sim.step();
    println!("Step two");
    sim.step();
    println!("Destroying the simulation");
    sim.destroy_node(node_a);
    sim.destroy_node(node_b);
}

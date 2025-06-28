use yamanote_sim::digital_passthrough;

#[test]
fn test_run_sim() {
    println!("Creating the simulation");
    let mut sim = digital_passthrough::Sim::new();
    let _node_a = sim.create_node();
    let _node_b = sim.create_node();
    println!("Step one");
    sim.step();
    println!("Step two");
    sim.step();
    println!("Destroying the simulation");
}

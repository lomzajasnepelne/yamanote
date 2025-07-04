pub mod l1;
pub mod sys;

pub fn run(mut l1: impl l1::L1, mut sys: impl sys::Sys) {
    loop {
        sys.lockstep_start();
        if sys.should_exit() {
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
    fn run_and_exit() {
        let l1 = l1_fake::L1Fake::new();
        let mut sys = sys::MockSys::new();
        sys.expect_lockstep_start().return_const(());
        sys.expect_should_exit().return_const(true);
        run(l1, sys);
    }
}

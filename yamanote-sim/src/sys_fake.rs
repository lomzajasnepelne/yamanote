use std::{sync::mpsc, time};

use yamanote_node::sys;

pub use crate::sys_fake::ctl::{StepParams, SysFakeCtl};

mod comm;
mod ctl;

pub struct SysFake {
    run: comm::RunReceiver,
    done: comm::DoneSender,
    step_params: StepParams,
    should_exit: bool,
}

impl SysFake {
    pub fn new(run: comm::RunReceiver, done: comm::DoneSender) -> Self {
        SysFake {
            run,
            done,
            step_params: StepParams {
                time: time::Duration::ZERO,
            },
            should_exit: false,
        }
    }
}

impl sys::Sys for SysFake {
    fn lockstep_start(&mut self) {
        let run_command = self.run.recv().unwrap();
        match run_command {
            comm::StepCommand::Exit => {
                self.should_exit = true;
            }
            comm::StepCommand::Run(params) => {
                self.step_params = params;
            }
        }
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }

    fn lockstep_end(&mut self) {
        // If SysFakeCtl has been destructed, send will fail.
        // In that case, it should have sent Exit command on the run channel,
        // so the next call to lockstep_start will stop the thread.
        self.done.send(()).unwrap_or(());
    }

    fn get_time_mono(&self) -> time::Duration {
        self.step_params.time
    }
}

pub fn create_sys_fake() -> (SysFake, SysFakeCtl) {
    let (params_tx, params_rx) = mpsc::channel();
    let (done_tx, done_rx) = mpsc::channel();
    (
        SysFake::new(params_rx, done_tx),
        SysFakeCtl::new(params_tx, done_rx),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lockstep_start_after_stop_node_returns_true() {
        let (mut fake, mut ctl) = create_sys_fake();
        ctl.stop_node();
        sys::Sys::lockstep_start(&mut fake);
        assert!(sys::Sys::should_exit(&fake));
    }

    #[test]
    fn lockstep_start_after_step_run_updates_step_params_and_returns_false() {
        let (mut fake, mut ctl) = create_sys_fake();
        let exp_time = time::Duration::from_micros(123);
        ctl.step_run(StepParams { time: exp_time });
        sys::Sys::lockstep_start(&mut fake);
        assert!(!sys::Sys::should_exit(&fake));
        assert_eq!(sys::Sys::get_time_mono(&fake), exp_time);
    }

    #[test]
    fn lockstep_end_done() {
        let (mut fake, mut ctl) = create_sys_fake();
        ctl.step_run(StepParams {
            time: time::Duration::from_micros(123),
        });
        sys::Sys::lockstep_end(&mut fake);
        ctl.step_join();
    }
}

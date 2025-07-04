use std::time;

use crate::sys_fake::comm;

pub use comm::StepParams;

const DONE_TIMEOUT: time::Duration = time::Duration::from_secs(1);

pub struct SysFakeCtl {
    run: comm::RunSender,
    done: comm::DoneReceiver,
}

impl SysFakeCtl {
    pub fn new(run: comm::RunSender, done: comm::DoneReceiver) -> Self {
        Self { run, done }
    }

    pub fn stop_node(&mut self) {
        // If the node has already stopped, send will fail, but this is fine.
        self.run.send(comm::StepCommand::Exit).unwrap_or(());
    }

    pub fn step_join(&mut self) {
        self.done.recv_timeout(DONE_TIMEOUT).unwrap();
    }

    pub fn step_run(&mut self, params: StepParams) {
        self.run.send(comm::StepCommand::Run(params)).unwrap();
    }
}

impl Drop for SysFakeCtl {
    fn drop(&mut self) {
        self.stop_node();
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;

    use super::*;

    #[test]
    fn step_join_reads_from_done_channel() {
        let (run_tx, _) = mpsc::channel();
        let (done_tx, done_rx) = mpsc::channel();
        let mut ctl = SysFakeCtl::new(run_tx, done_rx);
        done_tx.send(()).unwrap();
        ctl.step_join();
    }

    #[test]
    fn step_run_writes_to_step_params_channel() {
        let (run_tx, run_rx) = mpsc::channel();
        let (_, done_rx) = mpsc::channel();
        let mut ctl = SysFakeCtl::new(run_tx, done_rx);
        let step_params = StepParams {
            time: time::Duration::ZERO,
        };
        ctl.step_run(step_params);
        run_rx.recv_timeout(time::Duration::from_millis(1)).unwrap();
    }
}

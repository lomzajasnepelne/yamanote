use std::{sync::mpsc, time};

pub struct StepParams {
    pub time: time::Duration,
}

pub enum StepCommand {
    Exit,
    Run(StepParams),
}

pub type RunSender = mpsc::Sender<StepCommand>;
pub type RunReceiver = mpsc::Receiver<StepCommand>;
pub type DoneSender = mpsc::Sender<()>;
pub type DoneReceiver = mpsc::Receiver<()>;

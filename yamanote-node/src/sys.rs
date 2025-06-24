use std::time;

pub type ShouldExit = bool;

pub trait Sys {
    fn lockstep_start(&mut self) -> ShouldExit;
    fn lockstep_end(&mut self);
    fn get_time_mono(&self) -> time::Duration;
}

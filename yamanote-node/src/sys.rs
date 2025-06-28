use std::time;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait Sys {
    fn lockstep_start(&mut self);
    fn should_exit(&self) -> bool;
    fn lockstep_end(&mut self);
    fn get_time_mono(&self) -> time::Duration;
}

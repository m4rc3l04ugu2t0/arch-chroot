use crate::steps::{manager_steps, Steps};

pub fn run() {
    loop {
        let mut step = Steps::SetTimezone;
        match manager_steps(step) {
            Steps::Continue => {
                continue;
            }
            Steps::End => {
                break;
            }
            Steps::SetTimezone => {
                step = Steps::End;
            }
            Steps::Error => {}
        }
    }
}

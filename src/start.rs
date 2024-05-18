use crate::conf_sys::{config_system, Steps};

pub fn run() {
    loop {
        let mut step = Steps::SetTimezone;
        match config_system(step) {
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

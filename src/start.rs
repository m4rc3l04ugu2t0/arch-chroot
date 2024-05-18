use std::process::exit;

use crate::{
    conf_sys::config_system,
    config_timezone::set_timezone::set_timezone,
    errors::generic::handle_error,
    global_steps::{GlobalActions, TypeError},
};

pub fn run() {
    loop {
        let mut _action = GlobalActions::ConfigTimezone;
        match config_system(_action) {
            GlobalActions::Successfull(true) => {
                _action = GlobalActions::ChengeSysLanguage;
                continue;
            }
            GlobalActions::ChengeSysLanguage => {
                println!("Change language");
                break;
            }
            GlobalActions::Fix(true) => {
                println!("Ok timezone");
                break;
            }
            GlobalActions::Fix(false) => {
                println!("Error time");
                exit(1)
            }
            _ => {}
        }
    }
}

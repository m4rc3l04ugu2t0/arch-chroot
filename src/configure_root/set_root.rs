use std::process::Command;

use crate::run_commands::run_command;

pub fn set_root() -> Result<(), String> {
    run_command(&mut Command::new("passwd"))?;
    Ok(())
}

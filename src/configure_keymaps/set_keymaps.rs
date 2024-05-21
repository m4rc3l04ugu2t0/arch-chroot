use std::{fs::OpenOptions, io::Write, process::Command};

use crate::{
    config::keymaps::KEYMAPS, functions::get_user_selections::get_user_selections,
    run_commands::run_command,
};

pub fn configure_keymaps() -> Result<(), String> {
    let keymap_selected = get_user_selections(&KEYMAPS, "Selecione o layout de taclado");
    let path = "/etc/vconsole.conf";

    let mut file = match OpenOptions::new().append(true).create(true).open(path) {
        Ok(file) => file,
        Err(err) => return Err(format!("Error: {}", err)),
    };

    if let Err(err) = file.write_all(path.as_bytes()) {
        return Err(format!("Error: {}", err));
    }

    run_command(&mut Command::new("sh").arg("-c").arg(format!(
        "echo KEYMAP={} >> /etc/vconsole.conf",
        keymap_selected[0]
    )))?;

    Ok(())
}

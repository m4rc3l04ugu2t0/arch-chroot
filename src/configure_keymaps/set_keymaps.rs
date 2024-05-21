use std::process::Command;

use crate::{
    config::keymaps::KEYMAPS, functions::get_user_selections::get_user_selections,
    run_commands::run_command,
};

pub fn configure_keymaps() -> Result<(), String> {
    let keymap_selected = get_user_selections(&KEYMAPS, "Selecione o layout de taclado");

    for keymap in keymap_selected {
        run_command(
            &mut Command::new("echo")
                .arg(format!("KEYMAP={}", keymap))
                .arg(">>")
                .arg("/etc/vconsole.conf"),
        )?;
    }

    Ok(())
}

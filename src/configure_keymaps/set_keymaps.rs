use std::process::Command;

use crate::{
    config::keymaps::KEYMAPS, functions::get_user_selections::get_user_selections,
    run_commands::run_command,
};

pub fn set_keymaps() -> Result<(), String> {
    let keymap_selected = get_user_selections(&KEYMAPS, "Selecione o layout de taclado");

    run_command(Command::new("sh").arg("-c").arg(format!(
        "echo KEYMAP={} >> /etc/vconsole.conf",
        keymap_selected[0]
    )))?;

    println!("Sucesso");

    Ok(())
}

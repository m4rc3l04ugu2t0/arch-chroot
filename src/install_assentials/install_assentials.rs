use std::process::Command;

use crate::{functions::get_input_user::get_input_user, run_commands::run_command};

pub fn install_assentials() -> Result<(), String> {
    println!("Digite quais pacotes execiais você deseja: \n\
    (defalut: pacman -Sy dosfstools os-prober mtools network-manager-applet networkmanager wpa_supplicant wireless_tools dialog sudo");

    let input = get_input_user("Digite o nome dos pacotes")?;

    run_command(&mut Command::new("pacman").arg("-S").arg(input))?;
    println!("Sucesso");
    Ok(())
}

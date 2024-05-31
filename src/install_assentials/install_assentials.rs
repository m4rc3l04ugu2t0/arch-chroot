use std::process::Command;

use crate::{functions::get_input_user::get_input_user, run_commands::run_command};

pub fn install_assentials() -> Result<(), String> {
    println!("Digite quais pacotes execiais você deseja: \n\
    (defalut: pacman -Sy dosfstools os-prober mtools network-manager-applet networkmanager wpa_supplicant wireless_tools dialog sudo");
    println!("Enter para default");

    let mut input = get_input_user("Digite o nome dos pacotes")?;

    if input.trim().is_empty() {
        input = "dosfstools os-prober mtools network-manager-applet networkmanager wpa_supplicant wireless_tools dialog sudo".to_owned();
    }

    run_command(
        &mut Command::new("pacman")
            .arg("-S")
            .arg(input)
            .arg("--noconfirm"),
    )?;
    println!("Sucesso");

    println!("Instalando grub e configurando");

    run_command(
        &mut Command::new("pacman")
            .arg("-S")
            .arg("grub")
            .arg("efibootmgr")
            .arg("--noconfirm"),
    )?;

    run_command(&mut Command::new("grub-install").args(&[
        "--target=x86_64-efi",
        "--efi-directory=/boot",
        "--bootloader-id=rustinstallarch",
        "--recheck",
    ]))?;

    println!("Gerando arquivo de configuração do grub");

    run_command(&mut Command::new("grub-mkconfig").args(&["-o", "/boot/grub/grub.cfg"]))?;

    run_command(&mut Command::new("cat").arg("/boot/grub/grub.cfg"))?;
    Ok(())
}

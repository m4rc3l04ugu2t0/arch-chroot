use std::process::Command;

use crate::{functions::get_input_user::get_input_user, run_commands::run_command};

pub fn install_assentials() -> Result<(), String> {
    let default = "dosfstools os-prober mtools network-manager-applet networkmanager wpa_supplicant wireless_tools dialog sudo";
    println!(
        "Enter the name of your essential packages: \n\
    (defalut: {}",
        default
    );
    println!("Press `enter` to default");

    let mut input = get_input_user("Enter packages name.")?;

    if input.trim().is_empty() {
        input = default.into();
    }

    run_command(
        Command::new("pacman")
            .arg("-S")
            .arg(input)
            .arg("--noconfirm"),
    )?;
    println!("Successfully.");

    println!("Configuring grub.");

    run_command(
        Command::new("pacman")
            .arg("-S")
            .arg("grub")
            .arg("efibootmgr")
            .arg("--noconfirm"),
    )?;

    run_command(Command::new("grub-install").args([
        "--target=x86_64-efi",
        "--efi-directory=/boot",
        "--bootloader-id=rustinstallarch",
        "--recheck",
    ]))?;

    println!("Generating grub configuration file.");

    run_command(Command::new("grub-mkconfig").args(["-o", "/boot/grub/grub.cfg"]))?;

    run_command(Command::new("cat").arg("/boot/grub/grub.cfg"))?;

    println!("Grub configured successfully!");
    Ok(())
}

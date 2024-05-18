use std::io;

use crate::set_timezone::set_timezone;

// use std::fs::File;
// use std::io::{self, Write};
// use std::process::Command;
mod run_commands;
mod set_timezone;

fn main() {
    // let dry_run = true; // Mude para false para executar de verdade

    // run_command("pacman", &["-Syu", "--noconfirm"], dry_run);

    // let essential_packages = vec![
    //     "base-devel",
    //     "linux-headers",
    //     "networkmanager",
    //     "vim",
    //     "sudo",
    //     "git",
    // ];

    // for package in essential_packages {
    //     run_command("pacman", &["-S", package, "--noconfirm"], dry_run);
    // }

    // let gui_packages = vec!["xfce4", "xfce4-goodies", "lightdm", "lightdm-gtk-greeter"];

    // for package in gui_packages {
    //     run_command("pacman", &["-S", package, "--noconfirm"], dry_run);
    // }

    // let services = vec!["NetworkManager.service", "lightdm.service"];

    // for service in services {
    //     run_command("systemctl", &["enable", service], dry_run);
    // }
    // println!("Configuração básica do Arch Linux concluída.");
    println!("Configuração do Fuso Horário");
    println!("Selecione o fuso horário (ex: America/Sao_Paulo):");
    let mut timezone = String::new();
    io::stdin()
        .read_line(&mut timezone)
        .expect("Falha ao ler a entrada do usuário.");
    let timezone = timezone.trim(); // Remover caracteres de espaço em branco

    // Configurar fuso horário
    set_timezone(timezone);
}

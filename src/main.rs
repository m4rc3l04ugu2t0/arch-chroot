use configure_system::configure;

// use std::fs::File;
// use std::io::{self, Write};
// use std::process::Command;
mod conf_sys;
mod config_language;
mod config_timezone;
mod configure_system;
mod errors;
mod global_steps;
mod run_commands;

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

    // Configurar fuso horário
    if let Err(err) = configure() {
        eprintln!("Erro na configuração do sistema: {}", err);
    } else {
        println!("Configuração do sistema concluída com sucesso.");
    }
}

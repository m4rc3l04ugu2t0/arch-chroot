use nix::unistd::{chroot, execv, fork, ForkResult};
use std::ffi::CString;
// use std::fs::File;
// use std::io::{self, Write};
// use std::process::Command;

fn main() {
    //  path
    let new_root = "/mnt";

    // Exec command to new root
    if let Err(e) = chroot(new_root) {
        eprintln!("Erro ao mudar para o novo root: {}", e);
        return;
    }

    fn run_command(command: &str, args: &[&str], dry_run: bool) {
        if dry_run {
            println!("Dry run: {} {:?}", command, args);
            return;
        }

        let command_cstr = CString::new(command).expect("CString::new failed");
        let args_cstr: Vec<CString> = args.iter().map(|&arg| CString::new(arg).unwrap()).collect();
        let args_cstr_ref: Vec<&CString> = args_cstr.iter().collect();

        match unsafe { fork() } {
            Ok(ForkResult::Parent { child }) => {
                let _ = nix::sys::wait::waitpid(child, None);
            }
            Ok(ForkResult::Child) => {
                execv(&command_cstr, &args_cstr_ref).expect("execv failed");
            }
            Err(_) => eprintln!("Fork falhou"),
        }
    }

    let dry_run = true; // Mude para false para executar de verdade

    run_command("pacman", &["-Syu", "--noconfirm"], dry_run);

    let essential_packages = vec![
        "base-devel",
        "linux-headers",
        "networkmanager",
        "vim",
        "sudo",
        "git",
    ];

    for package in essential_packages {
        run_command("pacman", &["-S", package, "--noconfirm"], dry_run);
    }

    let gui_packages = vec!["xfce4", "xfce4-goodies", "lightdm", "lightdm-gtk-greeter"];

    for package in gui_packages {
        run_command("pacman", &["-S", package, "--noconfirm"], dry_run);
    }

    let services = vec!["NetworkManager.service", "lightdm.service"];

    for service in services {
        run_command("systemctl", &["enable", service], dry_run);
    }
    println!("Configuração básica do Arch Linux concluída.");
}

use nix::unistd::{execv, fork, ForkResult};
use std::{ffi::CString, path::Path};

pub fn run_command(command: &str, args: &[&str], dry_run: bool) -> Result<(), String> {
    if dry_run {
        println!("Dry run: {} {:?}", command, args);
        return Ok(());
    }

    // Ensure the command exists in the new root
    let command_path = format!("/usr/bin/{}", command);
    if !Path::new(&command_path).exists() {
        let message_error = format!("Command not found: {}", command_path);
        return Err(message_error);
    }

    let command_cstr = CString::new(command_path).expect("CString::new failed");
    let args_cstr: Vec<CString> = args.iter().map(|&arg| CString::new(arg).unwrap()).collect();
    let args_cstr_ref: Vec<&CString> = args_cstr.iter().collect();

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            let _ = nix::sys::wait::waitpid(child, None);
        }
        Ok(ForkResult::Child) => {
            if let Err(err) = execv(&command_cstr, &args_cstr_ref) {
                return Err(err.to_string());
            }
        }
        Err(err) => return Err(format!("Erro ao criar um novo processo: {}", err)),
    }

    Ok(())
}

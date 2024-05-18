use nix::unistd::{execv, fork, ForkResult};
use std::{ffi::CString, path::Path};

pub fn run_command(command: &str, args: &[&str], dry_run: bool) {
    if dry_run {
        println!("Dry run: {} {:?}", command, args);
        return;
    }

    // Ensure the command exists in the new root
    let command_path = format!("/usr/bin/{}", command);
    if !Path::new(&command_path).exists() {
        eprintln!("Command not found: {}", command_path);
        return;
    }

    let command_cstr = CString::new(command_path).expect("CString::new failed");
    let args_cstr: Vec<CString> = args.iter().map(|&arg| CString::new(arg).unwrap()).collect();
    let args_cstr_ref: Vec<&CString> = args_cstr.iter().collect();

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            let _ = nix::sys::wait::waitpid(child, None);
        }
        Ok(ForkResult::Child) => {
            execv(&command_cstr, &args_cstr_ref).expect("execv failed");
        }
        Err(_) => eprintln!("Fork failed"),
    }
}

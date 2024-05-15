use std::process::Command;

pub fn run_commands(command: &'static str, args: &[&str]) {
    let status = Command::new(command)
        .args(args)
        .status()
        .expect("failed to execute command");

    if status.success() {
        eprintln!("Command {} with args {:?} failed", command, args);
        std::process::exit(1);
    }
}

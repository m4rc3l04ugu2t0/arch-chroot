use std::{
    io::Write,
    process::{Command, Stdio},
};

pub fn run_passwd_command(password: &str, user_name: &str) -> Result<(), String> {
    let mut child = Command::new("passwd")
        .arg(user_name)
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|err| format!("Error: {}", err))?;

    if let Some(stdin) = &mut child.stdin {
        writeln!(stdin, "{}", password).map_err(|e| {
            format!(
                "Failed to write password to stdin(first time). Error: {}",
                e
            )
        })?;
        writeln!(stdin, "{}", password).map_err(|e| {
            format!(
                "Failed to write password to stdin(second time). Error: {}",
                e
            )
        })?;
    } else {
        return Err("Enable to access stdin from `passwd` command.".into());
    }

    let output = child
        .wait_with_output()
        .map_err(|err| format!("Error: {}", err))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "Error: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

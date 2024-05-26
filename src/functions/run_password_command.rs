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
        writeln!(stdin, "{}", password)
            .map_err(|e| format!("Falha ao escrever no stdin. Error: {}", e))?;
        writeln!(stdin, "{}", password)
            .map_err(|e| format!("Falha ao escrever no stdin. Error: {}", e))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|err| format!("Error: {}", err))?;

    if output.status.success() {
        Ok(())
    } else {
        Err("Falha ao alterar a senha.".into())
    }
}

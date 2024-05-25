use std::{
    io::Write,
    process::{Command, Stdio},
};

use rpassword::read_password;

fn set_root<T: Fn() -> Result<String, String>, C: Fn(&str) -> Result<(), String>>(
    read_password: T,
    run_command: C,
) -> Result<(), String> {
    println!("Digite a nova senha para o usuário root:");

    let password = read_password().map_err(|err| format!("Error: {}", err))?;
    run_command(&password)?;

    Ok(())
}

fn read_password_user() -> Result<String, String> {
    let password = read_password().map_err(|err| format!("Error: {}", err))?;
    println!("Novamente");
    let check_password = read_password().map_err(|err| format!("Error: {}", err))?;

    if password
        .to_ascii_lowercase()
        .trim()
        .eq(check_password.to_ascii_lowercase().trim())
    {
        Ok(password.trim().to_string())
    } else {
        Err("As senhas nao conferem".into())
    }
}

fn run_passwd_command(password: &str) -> Result<(), String> {
    let mut child = Command::new("passwd")
        .arg("root")
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
        println!("Senha do root foi alterada com sucesso.");
        Ok(())
    } else {
        eprintln!("Falha ao alterar a senha do root.");
        Err("Falha ao alterar a senha do root.".into())
    }
}

pub fn set_root_default() -> Result<(), String> {
    set_root(read_password_user, run_passwd_command)?;
    Ok(())
}

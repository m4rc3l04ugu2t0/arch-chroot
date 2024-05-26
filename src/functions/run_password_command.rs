use std::{
    io::Write,
    process::{Command, Stdio},
};

pub fn run_passwd_command(password: &str, user_name: &str) -> Result<(), String> {
    let mut child = Command::new("passwd")
        .arg(user_name)
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|err| format!("Erro ao iniciar o comando passwd: {}", err))?;

    if let Some(stdin) = &mut child.stdin {
        writeln!(stdin, "{}", password).map_err(|e| {
            format!(
                "Falha ao escrever a senha no stdin (primeira vez). Erro: {}",
                e
            )
        })?;
        writeln!(stdin, "{}", password).map_err(|e| {
            format!(
                "Falha ao escrever a senha no stdin (segunda vez). Erro: {}",
                e
            )
        })?;
    } else {
        return Err("Erro: Não foi possível acessar o stdin do comando passwd".into());
    }

    let output = child
        .wait_with_output()
        .map_err(|err| format!("Erro ao aguardar a saída do comando passwd: {}", err))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "Falha ao alterar a senha. Saída do comando: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

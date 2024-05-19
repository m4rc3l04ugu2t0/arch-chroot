use std::process::{Command, Output};

pub fn run_command(command: &mut Command) -> Result<Output, String> {
    let output = command
        .output()
        .map_err(|e| format!("Falha ao executar comando: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stderr_str = stderr.to_string();

        if is_correctable_error(&stderr_str) {
            if correct_errror(&stderr).is_ok() {
                let retry_output = command
                    .output()
                    .map_err(|e| format!("Falha ao executar comando: {}", e))?;
                if retry_output.status.success() {
                    return Ok(retry_output);
                } else {
                    let retry_stderr = String::from_utf8_lossy(&retry_output.stderr);
                    return Err(format!("Erro no comando após correção: {}", retry_stderr));
                }
            }
        }

        return Err(format!("Erro no comando: {}", stderr));
    }
    Ok(output)
}

pub fn is_correctable_error(stderr: &str) -> bool {
    stderr.contains("dependência faltando") || stderr.contains("não encontrado")
}

pub fn correct_errror(stderr: &str) -> Result<(), String> {
    if stderr.contains("dependencia faltando") {
        let missing_dep = extract_missing_dependency(stderr);

        if let Some(dep) = missing_dep {
            run_command(&mut Command::new("pacman").arg("-S").arg("--noconfirm").arg(dep))
                .map_err(|e| format!("Falha ao corrigir dependência: {}", e))?;

            return Ok(());
        }
    }

    Err("Erro não corrigível".to_string())
}

fn extract_missing_dependency(stderr: &str) -> Option<&str> {
    // Extraia o nome da dependência faltante do stderr
    // Exemplo simples, ajuste conforme necessário
    stderr
        .split_whitespace()
        .find(|&word| word == "dependência")
}

use std::{
    io::{self, Write},
    process::Command,
};

use chrono_tz::Tz;

use crate::{conf_sys::Steps, run_commands::run_command};

pub fn set_timezone() -> Result<Steps, String> {
    println!("Configuração do Fuso Horário");
    println!("Selecione o fuso horário (ex: America/Sao_Paulo):");
    let mut timezone = String::new();
    io::stdout().flush().expect("Error clear buffer");
    io::stdin()
        .read_line(&mut timezone)
        .expect("Falha ao ler a entrada do usuário.");
    let timezone = timezone.trim(); // Remover caracteres de espaço em branco
                                    // Executar o comando para configurar o fuso horário
    if !valid_timezone(timezone) {
        return Err(format!("Fuso horário inválido: {}", timezone));
    }
    let command = "ln";
    let args = [
        "-sf",
        &format!("/usr/share/zoneinfo/{}", timezone),
        "/etc/localtime",
    ];

    if let Err(err) = run_command(&command, &args, false) {
        return Err(format!("Error ao executar o comando: {}", err));
    }

    match get_date_output() {
        Ok(stdout) => println!("Date => {}", stdout),
        Err(err) => return Err(err),
    }

    Ok(Steps::SetTimezone)
}

fn valid_timezone(timezone: &str) -> bool {
    if timezone.parse::<Tz>().is_err() {
        println!("O fuso horário {} existe no sistema.", timezone);
        return false;
    };

    true
}

fn get_date_output() -> Result<String, String> {
    let output = Command::new("date")
        .output()
        .map_err(|e| format!("Falha ao converter a saída do comando para string: {}", e))?;

    if !output.status.success() {
        return Err("O comando date não foi executado com sucesso".to_string());
    }

    let stdout = String::from_utf8(output.stdout)
        .map_err(|e| format!("Falha ao converter a saída do comando para string: {}", e))?;

    Ok(stdout.trim().to_string())
}

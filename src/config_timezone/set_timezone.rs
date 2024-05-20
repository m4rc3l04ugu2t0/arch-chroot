use std::{
    io::{self, Write},
    process::Command,
};

use chrono_tz::Tz;

use crate::run_commands::run_command;

pub fn set_timezone() -> Result<(), String> {
    println!("Configuração do Fuso Horário");
    println!("Selecione o fuso horário (ex: America/Sao_Paulo):");
    let mut timezone = String::new();
    io::stdout().flush().expect("Error clear buffer");
    io::stdin()
        .read_line(&mut timezone)
        .map_err(|e| format!("Falha ao ler entrada: {}", e))?;
    let timezone = timezone.trim(); // Remover caracteres de espaço em branco
                                    // Executar o comando para configurar o fuso horário
    if !valid_timezone(timezone) {
        return Err(format!("Fuso horário inválido: {}", timezone));
    }
    // Define o fuso horário
    run_command(
        &mut Command::new("ln")
            .arg("-sf")
            .arg(format!("/usr/share/zoneinfo/{}", timezone))
            .arg("/etc/localtime"),
    )?;

    println!("Fuso horário configurado com sucesso.");
    get_date_output()?;
    Ok(())
}

fn valid_timezone(timezone: &str) -> bool {
    if timezone.parse::<Tz>().is_err() {
        println!("O fuso horário {} existe no sistema.", timezone);
        return false;
    };

    true
}

fn get_date_output() -> Result<(), String> {
    let output = Command::new("date")
        .output()
        .map_err(|e| format!("Falha ao converter a saída do comando para string: {}", e))?;

    if !output.status.success() {
        return Err("O comando date não foi executado com sucesso".to_string());
    }

    let stdout = String::from_utf8(output.stdout)
        .map_err(|e| format!("Falha ao converter a saída do comando para string: {}", e))?;

    println!("Date: {}", stdout.trim().to_string());
    Ok(())
}
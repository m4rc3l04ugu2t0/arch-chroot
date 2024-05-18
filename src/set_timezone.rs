use std::io;

use chrono_tz::Tz;

use crate::{run_commands::run_command, steps::Steps};

pub fn set_timezone() -> Result<Steps, String> {
    println!("Configuração do Fuso Horário");
    println!("Selecione o fuso horário (ex: America/Sao_Paulo):");
    let mut timezone = String::new();
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

    Ok(Steps::SetTimezone)
}

fn valid_timezone(timezone: &str) -> bool {
    if timezone.parse::<Tz>().is_err() {
        println!("O fuso horário {} existe no sistema.", timezone);
        return false;
    };

    true
}
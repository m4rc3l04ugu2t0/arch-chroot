use std::{
    fs::OpenOptions,
    io::{self, BufRead, BufReader, Write},
    process::Command,
};

use dialoguer::{theme::ColorfulTheme, Select};

use crate::run_commands::run_command;

pub fn set_language() -> Result<(), String> {
    println!("Configurando linguagem do sistema...");
    let languages = vec![
        "English",
        "Español",
        "Português",
        "Français",
        "Deutsch",
        "Italiano",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Selecione uma linguagem")
        .items(&languages)
        .default(6)
        .interact()
        .unwrap();

    println!("Você escolheu: {}", languages[selection]);

    let mut language = String::new();
    io::stdout()
        .flush()
        .map_err(|e| format!("Falha ao ler entrada: {}", e))?;
    io::stdin()
        .read_line(&mut language)
        .map_err(|e| format!("Error: {}", e))?;

    let language = language.trim();

    edit_locale_gen(language)?;

    run_command(&mut Command::new("locale-gen"))?;

    configure_locale_conf(language)?;

    println!("Linguagem do sistema configurada com sucesso.");
    Ok(())
}

fn edit_locale_gen(language: &str) -> Result<(), String> {
    let locale_gen_path = "/etc/locale.gen";
    let file = OpenOptions::new()
        .read(true)
        .open(locale_gen_path)
        .map_err(|e| format!("Falha ao abrir {}: {}", locale_gen_path, e))?;
    let reader = BufReader::new(file);

    let mut new_content = String::new();
    for line in reader.lines() {
        let line = line.map_err(|e| format!("Falha ao ler linha: {}", e))?;
        if line.contains(language) && line.starts_with('#') {
            new_content.push_str(&line[1..]);
        } else {
            new_content.push_str(&line);
        }
        new_content.push('\n');
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(locale_gen_path)
        .map_err(|e| format!("Falha ao abrir {} para escrita: {}", locale_gen_path, e))?;

    file.write_all(new_content.as_bytes())
        .map_err(|e| format!("Falha ao escrever no {}: {}", locale_gen_path, e))?;

    Ok(())
}

fn configure_locale_conf(language: &str) -> Result<(), String> {
    let locale_conf_path = "/etc/locale.conf";
    let content = format!("LANG={}\n", language);

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(locale_conf_path)
        .map_err(|e| format!("Falha ao abrir {} para escrita: {}", locale_conf_path, e))?;
    file.write_all(content.as_bytes())
        .map_err(|e| format!("Falha ao escrever no {}: {}", locale_conf_path, e))?;
    Ok(())
}

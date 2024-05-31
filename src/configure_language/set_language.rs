#![allow(unused)]
use crate::{
    config::languages::LANGUAGES, functions::get_user_selections::get_user_selections,
    run_commands::run_command,
};
use console::style;
use dialoguer::{
    console::Style,
    theme::{ColorfulTheme, Theme},
    MultiSelect,
};
use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    process::Command,
};

pub fn set_language() -> Result<(), String> {
    println!("Configurando linguagem do sistema...");

    let language_selected = get_user_selections(
        &LANGUAGES,
        "Don't select a single ISO without choosing a language.",
    );

    if language_selected.is_empty() {
        return Err("Selecione uma linguagem, nâo apenas uma ISO".to_string());
    }

    edit_locale_gen(language_selected.clone())?;
    run_command(&mut Command::new("locale-gen"))?;

    configure_locale_conf(language_selected.clone())?;

    println!("Language successfully configured");
    Ok(())
}

fn edit_locale_gen(language: Vec<String>) -> Result<(), String> {
    let locale_gen_path = "/etc/locale.gen";
    let file = OpenOptions::new()
        .read(true)
        .open(locale_gen_path)
        .map_err(|e| format!("Failure to open {}: {}", locale_gen_path, e))?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        let mut line = line.map_err(|e| format!("Failed to read line: {}", e))?;
        if line.trim() == format!("#{}", language[0].trim()) {
            line = language[0].to_string();
        }
        lines.push(line);
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(locale_gen_path)
        .map_err(|e| format!("Failure to open {} for reading: {}", locale_gen_path, e))?;

    for line in lines {
        writeln!(file, "{}", line);
    }

    Ok(())
}

fn configure_locale_conf(language: Vec<String>) -> Result<(), String> {
    let locale_conf_path = "/etc/locale.conf";
    let content = format!("LANG={}\n", language[0]);

    let file = File::create_new(locale_conf_path).map_err(|e| format!("sla: {}", e));

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(locale_conf_path)
        .map_err(|e| format!("Failure to open {} from reading: {}", locale_conf_path, e))?;
    file.write_all(content.as_bytes())
        .map_err(|e| format!("Failure to write in {}: {}", locale_conf_path, e))?;
    Ok(())
}

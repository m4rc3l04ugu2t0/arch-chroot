use std::{
    io::{stdin, stdout, Write},
    process::Command,
};

use crate::run_commands::run_command;

pub fn set_new_user() -> Result<(), String> {
    let user_name = get_input_user("Digite seu nome de usuario:")?;

    run_command(&mut Command::new("useradd").arg(format!(
        "-m -g users -G wheel,video,audio,kvm -s /bin/bash {}",
        user_name
    )))?;

    println!("User adicionado com sucesso!");
    Ok(())
}

fn get_input_user(text: &str) -> Result<String, String> {
    println!("{}", text);
    let mut input = String::new();
    stdout()
        .flush()
        .map_err(|err| format!("Error function get_user_input: {}", err))?;
    stdin()
        .read_line(&mut input)
        .map_err(|err| format!("Error function get_input_user: {}", err))?;

    Ok(input)
}

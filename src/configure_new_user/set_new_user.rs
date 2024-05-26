use std::{
    io::{stdin, stdout, Write},
    process::Command,
};

use crate::{
    functions::{read_password::read_password_user, run_password_command::run_passwd_command},
    run_commands::run_command,
};

pub fn set_new_user() -> Result<(), String> {
    let username = get_input_user("Digite seu nome de usuario:")?;

    run_command(
        Command::new("useradd")
            .arg("-m")
            .arg("-g")
            .arg("users")
            .arg("-G")
            .arg("wheel,video,audio,kvm")
            .arg("-s")
            .arg("/bin/bash")
            .arg(&username),
    )?;
    println!("User adicionado com sucesso!");
    set_password_user(&username)?;
    Ok(())
}

pub fn set_password_user(username: &str) -> Result<(), String> {
    let password = read_password_user().map_err(|err| format!("Error: {}", err))?;

    println!("Digite sua senha");

    run_passwd_command(&password, &username)?;

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

    let input = input.trim().to_owned();

    Ok(input)
}

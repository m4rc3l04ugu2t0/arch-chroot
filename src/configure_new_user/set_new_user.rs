use std::process::Command;

use crate::{
    functions::{
        get_input_user::get_input_user, read_password::read_password_user,
        run_password_command::run_passwd_command,
    },
    run_commands::run_command,
};

pub fn set_new_user() -> Result<(), String> {
    println!("Criar new user.");
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
    println!("Digite a senha pro usuario {}", username);
    set_password_user(&username)?;
    println!("Sucesso!");
    Ok(())
}

pub fn set_password_user(username: &str) -> Result<(), String> {
    let password = read_password_user().map_err(|err| format!("Error: {}", err))?;

    println!("Digite sua senha");

    run_passwd_command(&password, username)?;

    Ok(())
}

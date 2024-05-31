use std::process::Command;

use crate::{
    functions::{
        get_input_user::get_input_user, read_password::read_password_user,
        run_password_command::run_passwd_command,
    },
    run_commands::run_command,
};

pub fn set_new_user() -> Result<(), String> {
    println!("Create user.");
    let username = get_input_user("Enter your username:")?;

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
    println!("!");
    println!("Enter password {}", username);
    set_password_user(&username)?;
    println!("User successefully configured!");
    Ok(())
}

pub fn set_password_user(username: &str) -> Result<(), String> {
    let password = read_password_user().map_err(|err| format!("Error: {}", err))?;

    println!("Enter your password");

    run_passwd_command(&password, username)?;

    Ok(())
}

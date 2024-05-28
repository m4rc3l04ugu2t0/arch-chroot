use crate::functions::{
    read_password::read_password_user, run_password_command::run_passwd_command,
};

fn set_root<T: Fn() -> Result<String, String>, C: Fn(&str, &str) -> Result<(), String>>(
    read_password: T,
    run_command: C,
) -> Result<(), String> {
    println!("Digite a nova senha para o usuário root:");

    let password = read_password().map_err(|err| format!("Error: {}", err))?;
    run_command(&password, "root")?;

    Ok(())
}

pub fn set_root_default() -> Result<(), String> {
    set_root(read_password_user, run_passwd_command)?;
    Ok(())
}

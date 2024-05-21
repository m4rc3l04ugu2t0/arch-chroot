use std::{
    io::{self, Write},
    process::Command,
};

use crate::run_commands::run_command;

pub fn set_hostname() -> Result<(), String> {
    println!("Gigite o seu hostname: ");
    let mut hostname = String::new();
    io::stdout()
        .flush()
        .map_err(|err| format!("Error: {}", err))?;

    io::stdin()
        .read_line(&mut hostname)
        .map_err(|err| format!("Error: {}", err))?;

    let hostname = hostname.trim();

    if hostname.is_empty() {
        return Err("Digite um hostname valido".to_string());
    }

    run_command(
        &mut Command::new("sh")
            .arg("-c")
            .arg(format!("echo {} > /etc/hostname", hostname)),
    )?;

    let commands = [
        r#"echo "127.0.0.1    localhost" >> /etc/hosts"#.to_string(),
        r#"echo "::1          localhost" >> /etc/hosts"#.to_string(),
        format!(
            r#"echo "127.0.1.1    {}.localdomain {}" >> /etc/hosts"#,
            hostname, hostname
        ),
    ];

    for command in commands {
        run_command(&mut Command::new("sh").arg("-c").arg(command))?;
    }

    Ok(())
}

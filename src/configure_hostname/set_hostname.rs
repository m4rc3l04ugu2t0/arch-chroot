use std::process::Command;

use crate::{functions::get_input_user::get_input_user, run_commands::run_command};

pub fn set_hostname() -> Result<(), String> {
    let hostname = get_input_user("Digite seu hostname")?;

    let hostname = hostname;

    if hostname.is_empty() {
        return Err("Digite um hostname valido".to_string());
    }

    run_hostname(&hostname)?;

    Ok(())
}

fn run_hostname(hostname: &str) -> Result<(), String> {
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

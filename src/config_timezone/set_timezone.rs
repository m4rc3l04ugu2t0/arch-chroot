use std::{
    io::{self, BufRead, BufReader, Read, Write},
    process::Command,
};

use chrono_tz::Tz;

use crate::run_commands::run_command;

pub fn set_timezone() -> Result<(), String> {
    println!("Starting to configure the timezone system.");
    println!("Select the  timezone => (ex: America/Sao_Paulo):");

    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let timezone = read_user_input(&mut handle)?;
    let timezone = timezone.trim();

    if !valid_timezone(timezone) {
        return Err(format!("Invalid timezone: {}", timezone));
    }

    run_command(
        Command::new("ln")
            .arg("-sf")
            .arg(format!("/usr/share/zoneinfo/{}", timezone))
            .arg("/etc/localtime"),
    )?;

    println!("The timezone was configured successfully.");
    get_date_output()?;
    Ok(())
}

fn read_user_input<R: Read>(reader: &mut R) -> Result<String, String> {
    let mut timezone = String::new();
    io::stdout()
        .flush()
        .map_err(|e| format!("Error configuring the timezone: {}", e))?;
    let mut reader = BufReader::new(reader);
    reader
        .read_line(&mut timezone)
        .map_err(|e| format!("Failed to read input: {}", e))?;
    Ok(timezone)
}

fn valid_timezone(timezone: &str) -> bool {
    timezone.parse::<Tz>().is_ok()
}

fn get_date_output() -> Result<(), String> {
    let output = Command::new("date")
        .output()
        .map_err(|e| format!("Failed to convert command output to string: {}", e))?;

    if !output.status.success() {
        return Err("Failed to run command the date".to_string());
    }

    let stdout = String::from_utf8(output.stdout)
        .map_err(|e| format!("Failed to convert command output to string: {}", e))?;

    println!("Date: {}", stdout.trim());
    Ok(())
}

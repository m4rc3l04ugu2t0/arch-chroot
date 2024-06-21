use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

pub fn run_command(command: &mut Command) -> Result<(), String> {
    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failure to execute commando: {}", e))?;

    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            match line {
                Ok(line) => println!("{}", line),
                Err(err) => {
                    return Err(format!("Error: {}", err));
                }
            }
        }
    }

    if let Some(stderr) = child.stderr.take() {
        let reader = BufReader::new(stderr);

        for line in reader.lines() {
            match line {
                Ok(line) => eprint!("{}", line),
                Err(err) => return Err(format!("Error reading stderr: {}", err)),
            }
        }
    }

    let status = child
        .wait()
        .map_err(|err| format!("Failed to wait on child process: {}", err))?;

    if !status.success() {
        return Err(format!("Command exited with non-zero status: {}", status));
    }

    Ok(())
}

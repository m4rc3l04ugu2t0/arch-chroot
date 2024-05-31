use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

pub fn run_command(command: &mut Command) -> Result<(), String> {
    let mut child = command
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Falha ao executar comando: {}", e))?;

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

    let _ = child
        .wait()
        .map_err(|err| format!("Failed to wait on child process: {}", err))?;

    Ok(())
}

mod test_run_command {
    use super::*;

    #[test]
    fn test_run_command_valid() {
        let result = run_command(&mut Command::new("ls").arg("-a"));

        assert_eq!(Ok(()), result);
    }

    #[test]
    fn test_run_command_invalid() {
        let result = run_command(&mut Command::new("lssssss").arg("-a"));

        assert_eq!(
            Err("Falha ao executar comando: No such file or directory (os error 2)".to_string()),
            result
        );
    }
}

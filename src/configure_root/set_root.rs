use std::{
    io::{stdin, Write},
    process::{Command, Stdio},
};

fn set_root<T: Fn() -> Result<String, String>, C: Fn(&str) -> Result<(), String>>(
    read_password: T,
    run_command: C,
) -> Result<(), String> {
    println!("Digite a nova senha para o usuário root:");

    let password = read_password().map_err(|err| format!("Error: {}", err))?;
    run_command(&password)?;

    Ok(())
}

fn read_password() -> Result<String, String> {
    let mut password = String::new();
    stdin()
        .read_line(&mut password)
        .map_err(|e| e.to_string())?;
    Ok(password.trim().to_string())
}

fn run_passwd_command(password: &str) -> Result<(), String> {
    let mut child = Command::new("passwd")
        .arg("root")
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|err| format!("Error: {}", err))?;

    if let Some(stdin) = &mut child.stdin {
        writeln!(stdin, "{}", password)
            .map_err(|e| format!("Falha ao escrever no stdin. Error: {}", e))?;
        writeln!(stdin, "{}", password)
            .map_err(|e| format!("Falha ao escrever no stdin. Error: {}", e))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|err| format!("Error: {}", err))?;

    if output.status.success() {
        println!("Senha do root foi alterada com sucesso.");
        Ok(())
    } else {
        eprintln!("Falha ao alterar a senha do root.");
        Err("Falha ao alterar a senha do root.".into())
    }
}

pub fn set_root_default() -> Result<(), String> {
    set_root(read_password, run_passwd_command)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::{mock, predicate::*};

    mock! {
        pub Reader {
            fn call(&self) -> Result<String, String>;
        }
    }

    mock! {
        pub CommandRunner {
            fn call(&self, password: &str) -> Result<(), String>;
        }
    }

    #[test]
    fn test_set_root_read_password_error() {
        let mut mock_reader = MockReader::new();
        let mock_command_runner = MockCommandRunner::new();

        mock_reader
            .expect_call()
            .times(1)
            .returning(|| Err("Error reading password".to_string()));

        let result = set_root(|| mock_reader.call(), |pwd| mock_command_runner.call(pwd));
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Error: Error reading password");
    }

    #[test]
    fn test_set_root_command_error() {
        let mut mock_reader = MockReader::new();
        let mut mock_command_runner = MockCommandRunner::new();

        mock_reader
            .expect_call()
            .times(1)
            .returning(|| Ok("mock_password".to_string()));

        mock_command_runner
            .expect_call()
            .with(eq("mock_password"))
            .times(1)
            .returning(|_| Err("Error running command".to_string()));

        let result = set_root(|| mock_reader.call(), |pwd| mock_command_runner.call(pwd));
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Error running command");
    }

    #[test]
    fn test_set_root_success() {
        let mut mock_reader = MockReader::new();
        let mut mock_command_runner = MockCommandRunner::new();

        mock_reader
            .expect_call()
            .times(1)
            .returning(|| Ok("mock_password".to_string()));

        mock_command_runner
            .expect_call()
            .with(eq("mock_password"))
            .times(1)
            .returning(|_| Ok(()));

        let result = set_root(|| mock_reader.call(), |pwd| mock_command_runner.call(pwd));
        assert!(result.is_ok());
    }
}

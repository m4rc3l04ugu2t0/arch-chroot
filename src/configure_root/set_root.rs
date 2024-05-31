use crate::functions::{
    read_password::read_password_user, run_password_command::run_passwd_command,
};

fn set_root<T: Fn() -> Result<String, String>, C: Fn(&str, &str) -> Result<(), String>>(
    read_password: T,
    run_command: C,
) -> Result<(), String> {
    println!("Enter the new password for the root user:");

    let password = read_password().map_err(|err| format!("Error: {}", err))?;
    run_command(&password, "root")?;

    println!("Successfully!");
    Ok(())
}

pub fn set_root_default() -> Result<(), String> {
    set_root(read_password_user, run_passwd_command)?;
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
            fn call(&self, password: &str, user_name: &str) -> Result<(), String>;
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

        let result = set_root(
            || mock_reader.call(),
            |pwd, usr| mock_command_runner.call(pwd, usr),
        );
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
            .with(eq("mock_password"), eq("root"))
            .times(1)
            .returning(|_, _| Err("Error running command".to_string()));

        let result = set_root(
            || mock_reader.call(),
            |pwd, usr| mock_command_runner.call(pwd, usr),
        );
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
            .with(eq("mock_password"), eq("root"))
            .times(1)
            .returning(|_, _| Ok(()));

        let result = set_root(
            || mock_reader.call(),
            |pwd, usr| mock_command_runner.call(pwd, usr),
        );
        assert!(result.is_ok());
    }
}

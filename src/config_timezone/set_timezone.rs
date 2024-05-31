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

#[cfg(test)]
mod test_timezone {
    use std::io::{BufRead, Error, Read};

    use super::*;
    use mockall::predicate::*;

    struct MockStdin {
        lines: Vec<String>,
    }

    impl MockStdin {
        fn new(lines: Vec<&str>) -> Self {
            MockStdin {
                lines: lines.iter().map(|s| s.to_string()).collect(),
            }
        }
    }

    impl Read for MockStdin {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            if let Some(line) = self.lines.pop() {
                let bytes = line.into_bytes();
                let len = bytes.len().min(buf.len());
                buf[..len].copy_from_slice(&bytes[..len]);
                Ok(len)
            } else {
                Ok(0)
            }
        }
    }

    impl BufRead for MockStdin {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            Err(Error::new(io::ErrorKind::Other, "not implemented"))
        }

        fn consume(&mut self, _amt: usize) {}
    }

    impl Write for MockStdin {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            let s = String::from_utf8_lossy(buf);
            self.lines.push(s.trim().to_string());
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_valid_timezone_valid() {
        assert!(valid_timezone("America/Sao_Paulo"));
    }

    #[test]
    fn test_valid_timezone_invalid() {
        assert!(!valid_timezone("Invalid/Timezone"));
    }

    #[test]
    fn test_read_user_input() {
        let input_data = "America/Sao_Paulo\n";
        let mut mock_stdin = MockStdin::new(vec![input_data]);
        let result = read_user_input(&mut mock_stdin).unwrap();
        assert_eq!(result, input_data);
    }
}

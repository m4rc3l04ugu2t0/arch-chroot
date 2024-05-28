use std::io::{stdin, stdout, Write};

pub fn get_input_user(text: &str) -> Result<String, String> {
    println!("{}", text);
    let mut input = String::new();
    stdout().flush().map_err(|err| format!("Error: {}", err))?;
    stdin()
        .read_line(&mut input)
        .map_err(|err| format!("Error: {}", err))?;

    let input = input.trim().to_owned();

    Ok(input)
}

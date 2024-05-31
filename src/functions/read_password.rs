use rpassword::read_password;

pub fn read_password_user() -> Result<String, String> {
    let password = read_password().map_err(|err| format!("Error: {}", err))?;
    println!("Enter your password again.");
    let check_password = read_password().map_err(|err| format!("Error: {}", err))?;

    if password
        .to_ascii_lowercase()
        .trim()
        .eq(check_password.to_ascii_lowercase().trim())
    {
        Ok(password.trim().to_string())
    } else {
        Err("Passwords are differet.".into())
    }
}

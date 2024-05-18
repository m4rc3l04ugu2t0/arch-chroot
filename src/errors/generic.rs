use std::io::{self, Write};

use crate::global_steps::GlobalActions;

pub fn handle_error(error: String) -> GlobalActions {
    let mut input = String::new();
    println!("Error: {}", error);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let test = false;

    if test {
        return GlobalActions::Fix(false);
    }

    GlobalActions::Fix(true)
}

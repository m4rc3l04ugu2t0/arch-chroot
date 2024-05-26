use crate::{
    config_timezone::set_timezone::set_timezone,
    configure_hostname::set_hostname::set_hostname,
    configure_keymaps::set_keymaps::set_keymaps,
    configure_lanaguage::set_language::set_language,
    configure_new_user::set_new_user::set_new_user,
    configure_root::set_root::set_root_default,
    install_assentials::install_assentials::install_assentials,
    run_commands::{correct_errror, is_correctable_error},
};
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_writer};
use std::{
    fs::{self, OpenOptions},
    io::BufReader,
    path::Path,
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct State {
    step: usize,
}

pub fn configure() -> Result<(), String> {
    let mut state = load_state().unwrap_or(State { step: 0 });
    let steps: Vec<Box<dyn Fn() -> Result<(), String>>> = vec![
        Box::new(set_timezone),
        Box::new(set_language),
        Box::new(set_keymaps),
        Box::new(set_hostname),
        Box::new(set_root_default),
        Box::new(set_new_user),
        Box::new(install_assentials),
    ];

    for (i, step) in steps.iter().enumerate().skip(state.step) {
        println!("Executando etapa {}...", i + 1);
        loop {
            match step() {
                Ok(_) => {
                    state.step = i + 1;
                    save_state(&state)?;
                    break;
                }
                Err(err) => {
                    eprintln!("Erro na etapa {}: {}", i + 1, err);

                    if is_correctable_error(&err) {
                        println!("Tentando corrigir o erro na etapa {}...", i + 1);
                        if correct_errror(&err).is_ok() {
                            println!("Erro corrigido, reexecutando etapa {}...", i + 1);
                            continue;
                        } else {
                            eprintln!("Erro não pôde ser corrigido automaticamente.");
                        }
                    }

                    println!("Erro completo: {}", err);
                    save_state(&state)?;
                    return Err(err);
                }
            }
        }
    }

    Ok(())
}

fn load_state() -> Result<State, String> {
    let state_file = "src/state.json";
    if let Ok(file) = OpenOptions::new().read(true).open(state_file) {
        let reader = BufReader::new(file);
        let state: State =
            from_reader(reader).map_err(|e| format!("Falha ao ler estado: {}", e))?;
        Ok(state)
    } else {
        Ok(State { step: 0 })
    }
}

fn save_state(state: &State) -> Result<(), String> {
    let state_file = "src/state.json";

    let state_dir = Path::new(state_file).parent().expect("Error dictory");

    if !state_dir.exists() {
        fs::create_dir_all(state_dir)
            .map_err(|e| format!("Falha ao criar diretório {}: {}", state_dir.display(), e))?;
    }

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(state_file)
        .map_err(|e| format!("Falha ao salvar estado: {}", e))?;
    to_writer(file, state).map_err(|e| format!("Falha ao salvar estado: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod test_system {
    use super::*;

    #[test]
    fn test_save_state_valid() {
        let state = save_state(&State { step: 0 });
        assert_eq!(Ok(()), state);
    }

    #[test]
    fn test_load_state_valid() {
        let state = load_state();
        assert_eq!(Ok(State { step: 0 }), state);
    }
}

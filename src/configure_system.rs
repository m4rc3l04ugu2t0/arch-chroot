#![allow(unused)]
use crate::{
    conf_sys::config_system, config_language::conf_language::conf_sys_language,
    config_timezone::set_timezone::set_timezone, errors::generic::handle_error,
    global_steps::GlobalActions,
};
use serde::{Deserialize, Serialize};
use std::{
    fs::OpenOptions,
    io::{BufReader, Read},
    process::exit,
};

#[derive(Serialize, Deserialize, Debug)]
struct State {
    step: usize,
}

pub fn configure() -> Result<(), String> {
    let mut state = load_state().unwrap_or(State { step: 0 });
    let steps: Vec<Box<dyn Fn() -> GlobalActions>> =
        vec![Box::new(set_timezone), Box::new(set_timezone)];

    for (i, step) in steps.iter().enumerate().skip(state.step) {
        println!("Executando etapa {}...", i + 1);
        match step() {
            GlobalActions::Successfull => {
                state.step = i + 1;
                save_state(&state)?;
            }
            GlobalActions::Error(err) => {
                eprintln!("Erro na etapa {}: {}", i + 1, err);
                println!("{}", err);
            }
            GlobalActions::Fix(true) => {
                state.step = i + 1;
            }
            GlobalActions::Fix(false) => {
                println!("Error fix");
                break;
            }
            GlobalActions::End => {
                break;
            }
        }
    }

    Ok(())
}

fn load_state() -> Result<State, String> {
    let state_file = "state.json";
    if let Ok(file) = OpenOptions::new().read(true).open(state_file) {
        let reader = BufReader::new(file);
        let state: State =
            serde_json::from_reader(reader).map_err(|e| format!("Falha ao ler estado: {}", e))?;
        Ok(state)
    } else {
        Ok(State { step: 0 })
    }
}

fn save_state(state: &State) -> Result<(), String> {
    let state_file = "state.json";
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(state_file)
        .map_err(|e| format!("Falha ao salvar estado: {}", e))?;

    Ok(())
}

use configure_system::configure;

// use std::fs::File;
// use std::io::{self, Write};
// use std::process::Command;
mod conf_sys;
mod config;
mod config_timezone;
mod configure_lanaguage;
mod configure_system;
mod run_commands;

fn main() {
    if let Err(err) = configure() {
        eprintln!("Erro na configuração do sistema: {}", err);
    } else {
        println!("Configuração do sistema concluída com sucesso.");
    }
}

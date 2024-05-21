#[warn(unused_must_use)]
use configure_system::configure;

use crate::configure_lanaguage::set_language::set_language;

// use std::fs::File;
// use std::io::{self, Write};
// use std::process::Command;
mod conf_sys;
mod config_timezone;
mod configure_lanaguage;
mod configure_system;
mod run_commands;

fn main() {
    if let Err(err) = configure() {
        call();
        eprintln!("Erro na configuração do sistema: {}", err);
    } else {
        println!("Configuração do sistema concluída com sucesso.");
    }
}

fn call() {
    if let Err(err) = set_language() {
        eprintln!("Erro na configuração do sistema: {}", err);
    } else {
        println!("Configuração do sistema concluída com sucesso.");
    }
}

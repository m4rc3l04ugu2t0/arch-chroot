use configure_system::configure;

mod config;
mod config_timezone;
mod configure_hostname;
mod configure_keymaps;
mod configure_lanaguage;
mod configure_new_user;
mod configure_root;
mod configure_system;
mod functions;
mod run_commands;

fn main() {
    if let Err(err) = configure() {
        eprintln!("Erro na configuração do sistema: {}", err);
    } else {
        println!("Configuração do sistema concluída com sucesso.");
    }
}

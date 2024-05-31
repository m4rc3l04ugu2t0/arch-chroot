use configure_system::configure;

mod config;
mod config_timezone;
mod configure_hostname;
mod configure_keymaps;
mod configure_language;
mod configure_new_user;
mod configure_root;
mod configure_system;
mod functions;
mod install_packages;
mod run_commands;

fn main() {
    if let Err(err) = configure() {
        eprintln!("Failed to configure the system: {}", err);
    } else {
        println!("System configured successfully.");
    }
}

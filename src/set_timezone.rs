use crate::run_commands::run_command;

pub fn set_timezone(timezone: &str) {
    // Executar o comando para configurar o fuso horário
    let command = "ln";
    let args = [
        "-sf",
        &format!("/usr/share/zoneinfo/{}", timezone),
        "/etc/localtime",
    ];
    run_command(&command, &args, false);
}

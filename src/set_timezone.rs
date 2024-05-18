use crate::run_commands::run_command;

pub fn set_timezone(timezone: &str) {
    let command = format!("ln -sf /usr/share/zoneinfo/{} /etc/localtime", timezone);
    let args = [
        "-sf",
        &format!("/usr/share/zoneinfo/{}", timezone),
        "/etc/localtime",
    ];
    run_command(&command, &args, true);
}

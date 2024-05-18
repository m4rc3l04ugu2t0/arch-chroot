use crate::run_commands::run_command;

pub fn date_conf(region: &'static str, cyte: &'static str) {
    let mut command = vec![
        "ln -sf /usr/share/zoneinfo/",
        "",
        "/",
        "",
        " /etc/localtime",
    ];

    command[1] = region;
    command[3] = cyte;

    let command = command.join("");

    run_command("ln", &["-sf", &command], false);
}

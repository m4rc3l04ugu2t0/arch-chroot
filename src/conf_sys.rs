use crate::{
    config_language::conf_language::conf_sys_language,
    config_timezone::set_timezone::set_timezone,
    errors::generic::handle_error,
    global_steps::{GlobalActions, TypeError},
};

pub fn config_system(next: GlobalActions) -> GlobalActions {
    match next {
        GlobalActions::ConfigTimezone => {
            if let Err(err) = set_timezone() {
                return handle_error(err);
            }
            return GlobalActions::Successfull(true);
        }
        GlobalActions::ChengeSysLanguage => {
            conf_sys_language();
            return GlobalActions::Successfull(true);
        }
        _ => {}
    }

    GlobalActions::End
}

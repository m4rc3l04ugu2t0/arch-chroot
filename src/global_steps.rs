pub enum TypeError {
    TimezoneError(String),
}
pub enum GlobalActions {
    ConfigTimezone,
    ChengeSysLanguage,
    Successfull(bool),
    Fix(bool),
    Error(TypeError),
    End,
}

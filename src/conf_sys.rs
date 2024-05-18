use crate::set_timezone::set_timezone;

pub enum Steps {
    Continue,
    End,
    SetTimezone,
    Error,
}

pub fn config_system(next: Steps) -> Steps {
    match next {
        Steps::SetTimezone => {
            if let Err(err) = set_timezone() {
                eprint!("{}", err);
                return Steps::Error;
            }
            return Steps::SetTimezone;
        }
        Steps::End => {
            return Steps::End;
        }
        _ => {}
    }

    Steps::End
}

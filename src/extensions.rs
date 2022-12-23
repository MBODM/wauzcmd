use std::io::Error;

use crate::types::AppErr;

pub trait AppErrExt {
    fn to_app_err(&self) -> AppErr;
}

impl AppErrExt for Error {
    fn to_app_err(&self) -> AppErr {
        AppErr::from_string(self.to_string())
    }
}

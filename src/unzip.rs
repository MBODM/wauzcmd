use std::{fs, process::{Command, Child}};

use crate::types::AppErr;

const POWERSHELL_APP: &str = "powershell";

pub fn powershell_is_installed() -> bool {
    let powershell = POWERSHELL_APP;
    // Using .output() here, since .status() prints to console.
    let result = Command::new(powershell).arg("/?").output();
    // If there is NOT a NotFound error, this means Powershell is definitely installed.
    // Even if there is some other error, only a NotFound shall produce a false result.
    // It is ok to ignore all the other errors and just fail gracefully when executing.
    // The sole purpose here is to determine if the Powershell app is installed or not.
    match result {
        Ok(_) => true,
        Err(error) => !is_notfound_error(&error),
    }
}

pub fn unzip_file(zip_file: &str, dest_folder: &str) -> Result<Child, AppErr> {
    let zip_file = zip_file.trim();
    let dest_folder = dest_folder.trim();
    assert!(!zip_file.is_empty());
    assert!(!dest_folder.is_empty());
    let powershell = POWERSHELL_APP;
    let powershell_args = format!("Set-Variable ProgressPreference SilentlyContinue ; Expand-Archive \"{zip_file}\" -DestinationPath \"{dest_folder}\" -Force");
    let fuck = Command::new(powershell)
        .arg(powershell_args)
        .spawn()
        .map_err(|err| match is_notfound_error(&err) {
            // An ErrorKind of NotFound means Powershell is definitely not installed, so use own text.
            // Any other ErrorKind means some another error, so return that error´s specific text.
            true => AppErr::new("Powershell is not installed.", 0),
            false => AppErr::new(err.to_string().as_str(), 0),
        })?;
    //let console_output = String::from_utf8(output.stdout)
    // .map_err(|_| AppErr::new("Powershell output format is invalid.", 1))?;
    // let exit_code = output
    //     .status
    //     .code()
    //     .ok_or(AppErr::new("Powershell not returned any exit code.", 1))?;
    // if exit_code != 0 {
    //     let s = format!("Powershell returned failure exit code of {exit_code}");
    //     return Err(AppErr::new(s.as_str(), 1));
    //}
    Ok(fuck)
}

fn is_notfound_error(error: &std::io::Error) -> bool {
    match error.kind() {
        std::io::ErrorKind::NotFound => true,
        _ => false,
    }
}

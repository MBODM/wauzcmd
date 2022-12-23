use std::process::{Child, Command};

use crate::{extensions::AppErrExt, types::AppErr};

const POWERSHELL_APP: &str = "powershell";

pub fn is_available() -> bool {
    // Using .output() here, cause .status() writes the Command output to the terminal.
    let output_result = Command::new(POWERSHELL_APP).arg("/?").output();
    // If there is NOT a NotFound error, this means PowerShell is definitely available.
    // Only the NotFound error shall produce a false result here. No other error shall.
    // It is ok to ignore all other errors here and to just fail gracefully on execute.
    // The sole purpose here is to determine if the PowerShell app is available or not.
    // For the same reason there is no validation here if exit code was SUCCESS or not.
    match output_result {
        Ok(_) => true,
        Err(error) => !is_notfound_error(&error),
    }
}

pub fn execute_command(powershell_args: &str) -> Result<String, AppErr> {
    assert!(!powershell_args.is_empty());
    let output = Command::new(POWERSHELL_APP)
        .arg(powershell_args)
        .output()
        .map_err(|err| err.to_app_err())?;
    if !output.status.success() {
        return Err(AppErr::from_str("PowerShell exit code was not SUCCESS."));
    }
    let stdout_text =
        String::from_utf8(output.stdout).map_err(|err| AppErr::from_string(err.to_string()))?;
    Ok(stdout_text)
}

pub fn spawn_child_process(powershell_args: &str) -> Result<Child, AppErr> {
    assert!(!powershell_args.is_empty());
    let child = Command::new(POWERSHELL_APP)
        .arg(powershell_args)
        .spawn()
        .map_err(|err| err.to_app_err())?;
    Ok(child)
}

fn is_notfound_error(error: &std::io::Error) -> bool {
    match error.kind() {
        std::io::ErrorKind::NotFound => true,
        _ => false,
    }
}

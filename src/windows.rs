use std::process::{Child, Command};

use crate::types::AppErr;

const POWERSHELL_APP: &str = "powershell";

pub fn powershell_is_available() -> bool {
    let powershell = POWERSHELL_APP;
    // Using .output() here, cause .status() writes the Command output to the terminal.
    let output = Command::new(powershell).arg("/?").output();
    // If there is NOT a NotFound error, this means PowerShell is definitely available.
    // Only the NotFound error shall produce a false result here. No other error shall.
    // It is ok to ignore all other errors here and to just fail gracefully on execute.
    // The sole purpose here is to determine if the PowerShell app is available or not.
    // For the same reason there is no validation here if exit code was SUCCESS or not.
    match output {
        Ok(_) => true,
        Err(error) => !is_notfound_error(&error),
    }
}

pub fn spawn_powershell_child_process(powershell_args: &str) -> Result<Child, AppErr> {
    let powershell_args = powershell_args.trim();
    assert!(!powershell_args.is_empty());
    let powershell = POWERSHELL_APP;
    
    if powershell_args.contains("pand"){

        println!("{powershell_args}");
    }

    


    
    let child = Command::new(powershell)
        .arg(powershell_args)
        .spawn()
        .map_err(|error| match is_notfound_error(&error) {
            // An ErrorKind of NotFound means Powershell is definitely not installed, so use custom text.
            // Any other ErrorKind means some other error occurred, so return that error´s specific text.
            true => AppErr::new("PowerShell is not available.", 0),
            false => AppErr::new(error.to_string().as_str(), 0),
        })?;
    Ok(child)
}

fn is_notfound_error(error: &std::io::Error) -> bool {
    match error.kind() {
        std::io::ErrorKind::NotFound => true,
        _ => false,
    }
}

use std::{env, path::Path};

use crate::{filesystem, powershell, types::AppErr, core1, console};

// No need for some code here, to verify name and version from cargo.toml file,
// since cargo will show an error, if name or version contains an empty string.

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHOR: &str = "MBODM";
pub const DATE: &str = "2022-12-15";

// The app result states as consts, for some better readability.

pub const OK_VAL_SUCCESS: u8 = 0;
pub const OK_VAL_NO_ARGUMENTS: u8 = 1;
pub const ERR_VAL_COMMON_ERROR: u8 = 0;
pub const ERR_VAL_ARGUMENT_ERROR: u8 = 1;

pub fn run() -> Result<u8, AppErr> {
    let args = env::args().skip(1).collect::<Vec<String>>();
    let args_count = args.len();
    if args_count > 2 {
        return arg_err("Too many arguments.");
    }
    if args_count == 0 || args.iter().any(|arg| is_help_arg(arg)) {
        return Ok(OK_VAL_NO_ARGUMENTS);
    }
    if args_count == 1 {
        let path = Path::new(args[0].as_str());
        if path.exists() && path.is_dir() {
            return arg_err("Not enough arguments.");
        }
        return arg_err("Unknown argument.");
    }
    let source_path = Path::new(args[0].as_str());
    let dest_path = Path::new(args[1].as_str());
    if !source_path.exists() {
        return err("Given source folder not exists.");
    }
    if !dest_path.exists() {
        return err("Given destination folder not exists.");
    }
    if !source_path.is_dir() {
        return err("Given source argument is not a folder.");
    }
    if !dest_path.is_dir() {
        return err("Given destination argument is not a folder.");
    }
    println!("Given source folder:");
    println!("- {}", filesystem::get_absolute_path(source_path)?);
    println!();
    println!("Given destination folder:");
    println!("- {}", filesystem::get_absolute_path(dest_path)?);
    println!();
    if !powershell::is_available() {
        return err("PowerShell is not available.");
    }
    let zip_files = filesystem::get_zip_files(source_path)?;
    println!("Found {} zip files in source folder:", zip_files.len());
    zip_files
        .iter()
        .for_each(|zip_file| println!("- {}", zip_file.file_name));
    println!();
    print!("Unzip...");
    let success_count = core1::unzip(zip_files.as_slice(), dest_path,  || console::flush("."));
    print!("Done.");
    println!();
    println!("Successfully unzipped {} zip files.", success_count);
    Ok(OK_VAL_SUCCESS)
}

fn is_help_arg(arg: &str) -> bool {
    arg == "-h" || arg == "--help" || arg == "/?"
}

fn err<T>(msg: &str) -> Result<T, AppErr> {
    Err(AppErr::new(msg.to_string(), ERR_VAL_COMMON_ERROR))
}

fn arg_err<T>(msg: &str) -> Result<T, AppErr> {
    Err(AppErr::new(msg.to_string(), ERR_VAL_ARGUMENT_ERROR))
}

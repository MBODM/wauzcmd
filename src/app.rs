use crate::{
    helper,
    types::{AppErr, AppResult},
};

use std::{env, path::Path};

// No need for some code here, to verify name and version from cargo.toml file,
// since cargo will show an error, if name or version contains an empty string.

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHOR: &str = "MBODM";
pub const DATE: &str = "2022-12-15";

pub fn run() -> AppResult {
    let args = env::args().skip(1).collect::<Vec<String>>();
    let args_count = args.len();
    if args_count > 2 {
        return Err(AppErr::new("Too many arguments.", 1));
    }
    if args_count == 0 || args.iter().any(|arg| is_help_arg(arg)) {
        return Ok(0);
    }
    if args_count == 1 {
        let path = Path::new(args[0].as_str());
        if path.exists() && path.is_dir() {
            return Err(AppErr::new("Not enough arguments.", 1));
        }
        return Err(AppErr::new("Unknown argument.", 1));
    }
    let source_path = Path::new(args[0].as_str());
    let dest_path = Path::new(args[1].as_str());
    if !source_path.exists() {
        return err("Given source folder not exists.");
    }
    if !source_path.is_dir() {
        return err("Given source argument is not a folder.");
    }
    if !dest_path.exists() {
        return err("Given destination folder not exists.");
    }
    if !dest_path.is_dir() {
        return err("Given destination argument is not a folder.");
    }
    let absolute_source_path = source_path.canonicalize().or(Err(AppErr::new(
        "Could not determine absolute path of source folder.",
        0,
    )))?;
    let absolute_dest_path = dest_path.canonicalize().or(Err(AppErr::new(
        "Could not determine absolute path of dest folder.",
        0,
    )))?;
    let a_source =
        helper::convert_windows_unc_path(absolute_source_path.to_str().unwrap().to_string());
    let a_dest = helper::convert_windows_unc_path(absolute_dest_path.to_str().unwrap().to_string());
    println!("Source-Folder: {}", a_source);
    println!("Destination-Folder: {}", a_dest);
    println!();
    println!(
        "Sorry, unzipping is not implemented yet! :( Tool will be finished soon! Nonetheless:"
    );
    println!();
    Ok(1)
}

fn is_help_arg(arg: &str) -> bool {
    arg == "-h" || arg == "--help" || arg == "/?"
}

fn err(msg: &str) -> AppResult {
    Err(AppErr::new(msg, 0))
}

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
    let path = source_path.canonicalize().or(Err(AppErr::new(
        "Could not determine absolute folder path.",
        0,
    )))?;
    let x = path.to_str().unwrap();
    let x = helper::convert_windows_unc_path(x.to_string());
    println!("{}", x);

    // let path = fs::canonicalize(source_folder)
    // if !path.is_absolute() {
    //     return Err(String::from("absolute not"));
    // }
    println!("JIPIIIEEEE");
    println!("2 params");
    Ok(1)
}

fn is_help_arg(arg: &str) -> bool {
    arg == "-h" || arg == "--help" || arg == "/?"
}

fn err(msg: &str) -> AppResult {
    Err(AppErr::new(msg, 0))
}

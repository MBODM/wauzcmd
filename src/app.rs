use std::{
    env,
    io::{stdout, Write},
    path::Path,
};

use crate::{
    console, filesystem, platform,
    types::AppErr,
    unzip,
    windows::{self, powershell_is_available},
};

// No need for some code here, to verify name and version from cargo.toml file,
// since cargo will show an error, if name or version contains an empty string.

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const AUTHOR: &str = "MBODM";
pub const DATE: &str = "2022-12-15";

pub fn run() -> Result<usize, AppErr> {
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
    if !dest_path.exists() {
        return err("Given destination folder not exists.");
    }
    if !source_path.is_dir() {
        return err("Given source argument is not a folder.");
    }
    if !dest_path.is_dir() {
        return err("Given destination argument is not a folder.");
    }
    let source_folder = filesystem::get_absolute_path_as_string(source_path);
    let dest_folder = filesystem::get_absolute_path_as_string(dest_path);
    println!("Given source folder:");
    println!("- {source_folder}");
    println!();
    println!("Given destination folder:");
    println!("- {dest_folder}");
    println!();
    if platform::is_windows() && !windows::powershell_is_available() {
        return err("PowerShell is not available.");
    }
    let zip_file_infos = filesystem::get_zip_file_infos_from_folder(source_path);
    let zip_file_infos_count = zip_file_infos.len();
    println!("Found {zip_file_infos_count} zip files in source folder:");
    for zip_file_info in zip_file_infos {
        let zip_file_name = zip_file_info.file_name();
        println!("- {zip_file_name}");
    }
    println!();
    print!("Start unzip...");
    // //let unzipped_files_count = 999;
    // let progress_closure = || console::flush(|| print!("."));
    // let unzipped_files_count = unzip::unzip_all_zip_files(source_path, dest_path, progress_closure);
    // print!("Finished.");
    // println!();
    // println!("Successfully unzipped {unzipped_files_count} zip files.");
    Ok(1)
}

// for p2 in &paths {
//     let f = p2.display().to_string();
//     println!("spawn unzip for {f}");
//     let child = unzip::unzip_file(f.as_str(), &a_dest).unwrap();
//     childs.push();
//     //flush(|| println!("created child with pid {pid}"));
// }
// println!();
// for c in childs.iter_mut() {
//     let pid = c.id();
//     println!("wait for pid {pid}");
//     c.wait();
// }

fn is_help_arg(arg: &str) -> bool {
    arg == "-h" || arg == "--help" || arg == "/?"
}

fn err(msg: &str) -> Result<usize, AppErr> {
    Err(AppErr::new(msg, 0))
}

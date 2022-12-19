use std::{path::Path, process::Child};

use crate::{filesystem, windows, platform, types::ZipFileInfo};

pub fn extract_all<F: Fn()>(zip_file_infos: &[ZipFileInfo], dest_path: &Path, progress_closure: F) -> usize
{
    if !platform::is_windows() {
        let msg = platform::UNSUPPORTED_PLATFORM_ERROR;
        panic!("{msg}");  // <-- TODO error handling
    }
    let dest_folder = filesystem::get_absolute_path_as_string(dest_path);
    let mut child_procs: Vec<Child> = Vec::new();
    for zip_file_info in zip_file_infos {
        let zip_file = zip_file_info.file_path();
        let powershell_args = format!("Set-Variable ProgressPreference SilentlyContinue ; Expand-Archive \"{zip_file}\" -DestinationPath \"{dest_folder}\" -Force");
        let child_proc = windows::spawn_powershell_child_process(&powershell_args).unwrap();  // <-- TODO error handling
        child_procs.push(child_proc);
    }
    for mut child_proc in child_procs {
        let pid = child_proc.id();
        println!("wait for {pid}");
        match child_proc.wait() {
            Ok(_) => progress_closure(),
            Err(_) => eprintln!("TODO ERROR"), // <-- TODO error handling
        }
    }
    zip_file_infos.len()
}

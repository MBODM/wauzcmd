use std::{fs, path::Path, process::Child};

use crate::{types::ZipFile, filesystem, powershell};

pub fn unzip<F: Fn()>(zip_files: &[ZipFile], dest_folder: &Path, progress_closure: F) -> usize {
    assert!(dest_folder.exists() && dest_folder.is_dir());
    let absolute_dest_folder_string = filesystem::get_absolute_path(dest_folder).unwrap(); // <-- TODO error handling
    let mut child_procs: Vec<Child> = Vec::new();
    for zip_file in zip_files {
        let temp_folder = filesystem::get_random_temp_folder().unwrap(); // <-- TODO error handling
        fs::create_dir(temp_folder).unwrap(); // <-- TODO error handling
        let ps_dest = temp_folder;
        let ps_source = &zip_file.absolute_path;
        let ps_args = format!("Set-Variable ProgressPreference SilentlyContinue ; Expand-Archive \"{ps_source}\" -DestinationPath \"{ps_dest}\" -Force");
        let child_proc = powershell::spawn_child_process(&ps_args).unwrap(); // <-- TODO error handling
        child_procs.push(child_proc);
    }
    let mut success_counter = 0;
    for mut child_proc in child_procs {
        match child_proc.wait() {
            Ok(exit_status) => {
                if !exit_status.success() {
                    panic!("TODO"); // <-- TODO error handling
                }

                
                // Todo





                success_counter += 1;
                progress_closure();
            }
            Err(_) => panic!("TODO"), // <-- TODO error handling
        }
    }
    return success_counter;
}

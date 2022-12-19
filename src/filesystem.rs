use std::path::Path;

use crate::{platform, types::ZipFileInfo};

pub fn get_absolute_path_as_string(path: &Path) -> String {
    if !platform::is_windows() {
        let msg = platform::UNSUPPORTED_PLATFORM_ERROR;
        panic!("{msg}"); // <-- TODO error handling
    }
    let unc_path = path.canonicalize().unwrap(); // <-- TODO error handling
    let result = unc_path.display().to_string();
    // Using code here from
    // https://stackoverflow.com/questions/50322817/how-do-i-remove-the-prefix-from-a-canonical-windows-path
    const VERBATIM_PREFIX: &str = r#"\\?\"#;
    if result.starts_with(VERBATIM_PREFIX) {
        return result[VERBATIM_PREFIX.len()..].to_string();
    }
    result
}

pub fn get_zip_file_infos_from_folder(path: &Path) -> Vec<ZipFileInfo> {
    if !path.exists() || !path.is_dir() {
        panic!("Given path is not an existing folder."); // <-- TODO error handling
    }
    let read_dir = path.read_dir().unwrap(); // <-- TODO error handling
    let mut result: Vec<ZipFileInfo> = Vec::new();
    for dir_entry_result in read_dir {
        let dir_entry = dir_entry_result.unwrap(); // <-- TODO error handling
        let path_buf = dir_entry.path();
        let file_ext = path_buf.extension().unwrap(); // <-- TODO error handling
        if file_ext == "zip" {
            let file_path = get_absolute_path_as_string(&path_buf);
            let file_name = dir_entry.file_name().into_string().unwrap(); // <-- TODO error handling
            result.push(ZipFileInfo::new(file_path, file_name));
        }
    }
    result
}

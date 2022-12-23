use std::{
    env,
    path::{Path, PathBuf},
};

use crate::{
    extensions::AppErrExt,
    powershell,
    types::{AppErr, ZipFile},
};

pub fn get_absolute_path(file_or_folder: &Path) -> Result<String, AppErr> {
    assert!(file_or_folder.exists());
    // Using code here from:
    // https://stackoverflow.com/questions/50322817/how-do-i-remove-the-prefix-from-a-canonical-windows-path
    let unc_path = file_or_folder
        .canonicalize()
        .map_err(|err| err.to_app_err())?;
    let unc_path_string = unc_path.display().to_string();
    const VERBATIM_PREFIX: &str = r#"\\?\"#;
    Ok(match unc_path_string.starts_with(VERBATIM_PREFIX) {
        true => unc_path_string[VERBATIM_PREFIX.len()..].to_string(),
        false => unc_path_string,
    })
}

pub fn get_config_folder() -> Result<String, AppErr> {
    let app_data = env::var("LOCALAPPDATA").map_err(|err| AppErr::from_string(err.to_string()))?;
    let mut path_buf = PathBuf::from(app_data);
    path_buf.push("MBODM");
    let config_folder = path_buf.display().to_string();
    Ok(config_folder)
}

pub fn get_random_temp_folder() -> Result<String, AppErr> {
    let ps_args = "[System.IO.Path]::GetRandomFileName()";
    let random_file_name = powershell::execute_command(ps_args)?;
    let folder_name = format!(
        "MBODM.WAUZ.{}.tmp",
        random_file_name.replace(".", "").to_lowercase()
    );
    let mut path_buf = env::temp_dir();
    path_buf.push(folder_name);
    let temp_folder = path_buf.display().to_string();
    Ok(temp_folder)
}

pub fn get_zip_files(folder: &Path) -> Result<Vec<ZipFile>, AppErr> {
    assert!(folder.exists() && folder.is_dir());
    let read_dir = folder.read_dir().map_err(|err| err.to_app_err())?;
    let mut zip_files: Vec<ZipFile> = Vec::new();
    for dir_entry_result in read_dir {
        let dir_entry = dir_entry_result.map_err(|err| err.to_app_err())?;
        let path_buf = dir_entry.path();
        let file_ext = path_buf
            .extension()
            .ok_or(AppErr::from_str("Could not determine zip file extension."))?;
        if file_ext == "zip" {
            let absolute_path = get_absolute_path(&path_buf)?;
            let file_name = dir_entry
                .file_name()
                .into_string()
                .map_err(|_| AppErr::from_str("Could not determine zip file name."))?;
            zip_files.push(ZipFile::new(absolute_path, file_name));
        }
    }
    Ok(zip_files)
}

pub fn move_folder(source_folder: &Path, dest_folder: &Path) -> Result<(), AppErr> {
    assert!(source_folder.exists() && dest_folder.exists());
    let ps_source = get_absolute_path(source_folder)?;
    let ps_dest = get_absolute_path(dest_folder)?;
    let ps_args = format!("Move-Item -Path \"{ps_source}\" -Destination \"{ps_dest}\"");
    powershell::execute_command(ps_args.as_str())?;
    Ok(())
}

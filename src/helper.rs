// Code here is based on:
// https://stackoverflow.com/questions/50322817/how-do-i-remove-the-prefix-from-a-canonical-windows-path

pub fn convert_windows_unc_path(path: String) -> String {
    const VERBATIM_PREFIX: &str = r#"\\?\"#;
    if !path.starts_with(VERBATIM_PREFIX) {
        return path;
    }
    path[VERBATIM_PREFIX.len()..].to_string()
}

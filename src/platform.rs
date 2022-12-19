pub const UNSUPPORTED_PLATFORM_ERROR: &str = "";

pub fn is_windows() -> bool {
    cfg!(windows)
}

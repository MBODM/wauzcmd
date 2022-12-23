#[derive(Debug)]
pub struct AppErr {
    pub msg: String,
    pub val: u8,
}

impl AppErr {
    pub fn new(msg: String, val: u8) -> Self {
        Self { msg, val }
    }

    pub fn from_string(msg: String) -> Self {
        Self { msg, val: 0 }
    }

    pub fn from_str(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
            val: 0,
        }
    }
}

#[derive(Debug)]
pub struct ZipFile {
    pub absolute_path: String,
    pub file_name: String,
}

impl ZipFile {
    pub fn new(absolute_path: String, file_name: String) -> Self {
        Self {
            absolute_path,
            file_name,
        }
    }
}

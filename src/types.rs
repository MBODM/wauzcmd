#[derive(Debug)]
pub struct AppErr {
    pub msg: String,
    pub val: usize,
}

impl AppErr {
    pub fn new(msg: &str, val: usize) -> Self {
        Self {
            msg: msg.to_string(),
            val,
        }
    }
}

#[derive(Debug)]
pub struct ZipFileInfo {
    file_path: String,
    file_name: String,
}

impl ZipFileInfo {
    pub fn new(file_path: String, file_name: String) -> Self {
        Self {
            file_path,
            file_name,
        }
    }

    pub fn file_path(&self) -> String {
        self.file_path
    }

    pub fn file_name(&self) -> String {
        self.file_name
    }
}

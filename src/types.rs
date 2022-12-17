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

pub type AppResult = Result<usize, AppErr>;

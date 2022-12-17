use std::process::ExitCode;

mod app;
mod console;
mod types;
mod helper;

fn main() -> ExitCode {
    console::show_title();
    match app::run() {
        Ok(val) => {
            match val {
                0 => console::show_usage(),
                _ => console::show_outro(),
            }
            ExitCode::SUCCESS
        }
        Err(app_err) => {
            match app_err.val {
                0 | 1 => console::show_error(app_err.msg.as_str(), app_err.val == 1),
                _ => console::show_error("Unknown error occurred.", false),
            }
            ExitCode::FAILURE
        }
    }
}

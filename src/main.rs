use std::process::ExitCode;

mod app;
mod console;
mod core1;
mod extensions;
mod filesystem;
mod powershell;
mod types;

// It is ok to panic() inside match here, because some assert() would do the same.
// And the compiler needs some default inside match, even when an assert() exists.

fn main() -> ExitCode {
    console::show_title();
    match app::run() {
        Ok(val) => {
            match val {
                app::OK_VAL_SUCCESS => console::show_outro(),
                app::OK_VAL_NO_ARGUMENTS => console::show_usage(),
                _ => panic!("The app module returned an unsupported Ok() result."),
            }
            ExitCode::SUCCESS
        }
        Err(app_err) => {
            match app_err.val {
                app::ERR_VAL_COMMON_ERROR => console::show_error(app_err.msg.as_str(), false),
                app::ERR_VAL_ARGUMENT_ERROR => console::show_error(app_err.msg.as_str(), true),
                _ => panic!("The app module returned an unsupported Err() result."),
            }
            ExitCode::FAILURE
        }
    }
}

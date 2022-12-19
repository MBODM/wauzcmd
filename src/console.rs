use std::io::{stdout, Write};

use crate::app;

pub fn show_title() {
    let app_name = app::NAME;
    let app_version = app::VERSION;
    let app_author = app::AUTHOR;
    let app_date = app::DATE;
    println!();
    println!("{app_name} {app_version} (by {app_author} {app_date})");
    println!();
}

pub fn show_usage() {
    let app_name = app::NAME;
    println!("A tiny unzip tool for World of Warcraft addons ");
    println!();
    println!("Start executable with 2 existing folders as arguments and use double quotes (\"\") if a path contains spaces.");
    println!();
    println!("Usage:");
    println!();
    println!("{app_name}.exe \"path-to-SOURCE-folder\" \"path-to-DESTINATION-folder\"");
    println!();
    println!("  SOURCE       The folder which contains the addon zip files (typically a temporary download folder)");
    println!("  DESTINATION  The folder to unzip the addons into (typically the World of Warcraft 'AddOns' folder)");
    println!();
    println!("Have a look at \"https://github.com/mbodm/wauzcmd\" for more information.");
}

pub fn show_outro() {
    println!("Have a nice day.");
}

pub fn show_error(msg: &str, show_hint: bool) {
    println!("Error: {msg}");
    if show_hint {
        println!();
        let app_name = app::NAME;
        println!("Run \"{app_name}.exe --help\" for more information.");
    }
}

pub fn flush<F: FnOnce()>(closure: F) {
    closure();
    stdout().flush().expect("todo");
}

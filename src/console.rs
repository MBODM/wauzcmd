use std::io::{stdout, Write};

use crate::app;

pub fn show_title() {
    println!();
    println!(
        "{} {} (by {} {})",
        app::NAME.to_uppercase(),
        app::VERSION,
        app::AUTHOR,
        app::DATE
    );
    println!();
}

pub fn show_usage() {
    println!("A tiny unzip tool for World of Warcraft addons ");
    println!();
    println!("Start executable with 2 existing folders as arguments and use double quotes (\"\") if a path contains spaces.");
    println!();
    println!("Usage:");
    println!();
    println!(
        "{}.exe \"path-to-SOURCE-folder\" \"path-to-DESTINATION-folder\"",
        app::NAME.to_lowercase()
    );
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
    assert!(!msg.is_empty());
    println!("Error: {}", msg);
    if show_hint {
        println!();
        println!(
            "Run \"{}.exe --help\" for more information.",
            app::NAME.to_lowercase()
        );
    }
}

pub fn flush(msg: &str) {
    print!("{}", msg);
    stdout().flush().expect("TODO"); // <-- TODO: Error handling.
}

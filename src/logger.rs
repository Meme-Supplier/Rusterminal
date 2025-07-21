use std::fs::OpenOptions;
use std::io::Write;

use crate::funcs::load_configs;
use crate::s_vars;

pub fn write_to_file(text: &str, path: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "{text}")?;
    Ok(())
}

pub fn log(text: &str) {
    let time = s_vars::get_time();
    let home = s_vars::get_home();

    match load_configs()
        .get("outputLoggedMessages")
        .map(String::as_str)
    {
        Some("true") => {
            println!("{time}: {text}")
        }
        Some(_) => {}
        None => eprintln!("Setting \"forceUniversalOScompatability\" not found in config!\nTry reloading Rusterminal!")
    }

    if let Err(e) = {
        write_to_file(
            &format!("{time}: {text}"),
            &format!("{home}/rusterminal/log.txt"),
        )
    } {
        eprintln!("Failed to write log: {e}");
    }
}

pub fn init(init: &str) {
    let home = s_vars::get_home();
    if let Err(e) = write_to_file(init, &format!("{home}/rusterminal/log.txt")) {
        eprintln!("Failed to init log: {e}");
    }
}

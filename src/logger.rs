use chrono::Local;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;

use crate::funcs;

pub fn get_home() -> String {
    env::var("HOME").expect("Failed to get HOME directory")
}

pub fn get_time_format() -> String {
    let config = &funcs::load_configs();

    let format = &config
        .get("logTimeAndDateFormat")
        .map(|s| s.as_str())
        .unwrap_or_default()[1..config
        .get("logTimeAndDateFormat")
        .map(|s| s.as_str())
        .unwrap_or_default()[1..]
        .len()];

    format.to_string()
}

pub fn get_time() -> String {
    let format = &get_time_format();
    Local::now().format(format).to_string()
}

pub fn write_to_file(text: &str, path: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "{text}")?;
    Ok(())
}

pub fn log(text: &str) {
    let time = get_time();
    let home = get_home();

    match funcs::load_configs()
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
    let home = get_home();
    if let Err(e) = write_to_file(init, &format!("{home}/rusterminal/log.txt")) {
        eprintln!("Failed to init log: {e}");
    }
}

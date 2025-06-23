#!/usr/bin/env rust-script

use chrono::Local;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;

pub fn get_home() -> String {
    env::var("HOME").expect("Failed to get HOME directory")
}

pub fn get_time() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn write_to_file(text: &str, path: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "{text}")?;
    Ok(())
}

pub fn log(text: &str) {
    let time = get_time();
    let home = get_home();

    if let Err(e) = write_to_file(
        &format!("{time}: {text}"),
        &format!("{home}/rusterminal/log.txt"),
    ) {
        eprintln!("Failed to write log: {e}");
    }
}

pub fn init(init: &str) {
    let home = get_home();
    if let Err(e) = write_to_file(init, &format!("{home}/rusterminal/log.txt")) {
        eprintln!("Failed to init log: {e}");
    }
}

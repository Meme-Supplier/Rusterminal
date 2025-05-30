#!/usr/bin/env rust-script
#[cfg(target_os = "linux")]

use chrono::Local;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

fn get_log_path() -> std::io::Result<PathBuf> {
    let home =
        env::var("HOME").map_err(|e| std::io::Error::new(std::io::ErrorKind::NotFound, e))?;
    let mut path = PathBuf::from(home);
    path.push("rusterminal/log.txt");
    Ok(path)
}

fn write_to_file(text: &str) -> std::io::Result<()> {
    let path = get_log_path()?;
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "{text}")?;
    Ok(())
}

pub fn get_time() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn log(text: &str) {
    let time = get_time();
    if let Err(e) = write_to_file(&format!("{time}: {text}")) {
        eprintln!("Failed to write log: {e}")
    }
}

pub fn init(init: &str) {
    if let Err(e) = write_to_file(init) {
        eprintln!("Failed to init log: {e}")
    }
}

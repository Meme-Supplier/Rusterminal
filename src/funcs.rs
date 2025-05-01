#!/usr/bin/env rust-script
#[cfg(target_os = "linux")]

/* 2025 Meme Supplier
memesupplierbusiness@gmail.com
Maintained by Meme Supplier */

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::{Command, Stdio};

static VERSION: &str = "v0.2.3";

pub fn load() -> HashMap<String, String> {
    // Get the home directory
    let home_dir = env::var("HOME").expect("Failed to get HOME directory");

    // Read the config file
    let content = fs::read_to_string(format!("{home_dir}/rusterminal/src/settings.conf"))
        .expect("Failed to read config");

    // Create a HashMap to store configurations
    let mut config = HashMap::new();

    // Parse each line
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue; // skip empty lines or comments
        }
        if let Some((key, value)) = line.split_once('=') {
            config.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    config // <--- Return the config HashMap
}

pub fn xray() {
    let home_dir = env::var("HOME").expect("Failed to get HOME directory");
    let python_script = format!("{home_dir}/rusterminal/src/xray.py");

    // Run the Python script using 'python3'
    let _ = Command::new("python3")
        .arg(python_script)
        .status()
        .expect("Failed to execute Python script");
}

pub fn fmtdsk() {
    let home_dir = env::var("HOME").expect("Failed to get HOME directory");
    let python_script = format!("{home_dir}/rusterminal/src/diskfmt.py");

    // Run the Python script using 'python3'
    let _ = Command::new("python3")
        .arg(python_script)
        .status()
        .expect("Failed to execute Python script");
}

pub fn credits() {
    println!("\nCredits:\n\nMaintainer: Meme Supplier\nLead programmer: Meme Supplier\n");
}

pub fn new_dir(dir: &str) {
    run_shell_command(&format!("mkdir {dir}"));
}

pub fn input(str: &str) {
    let mut input = String::new();

    // Print the prompt and flush stdout
    println!("{str}");
    io::stdout().flush().expect("Failed to flush");

    // Read user input
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}

pub fn clean() {
    if detect_package_manager().as_str() == "apt" {
        // Debian/Ubuntu
        run_shell_command("sudo apt autoremove -y");
    } else if detect_package_manager().as_str() == "dnf" {
        // Fedora
        run_shell_command("sudo dnf autoremove -y");
    } else {
        // Arch
        run_shell_command("sudo pacman -Rns $(pacman -Qdtq) --noconfirm");
        match load().get("considerYayAsAPackageManager").map(String::as_str) {
            Some("true") => run_shell_command("yay -Rns $(pacman -Qdtq) --noconfirm"),
            Some(_) => {},
            None => println!("Setting 'considerYayAsAPackageManager' not found in config!\nTry reloading Rusterminal!"),
        }
    }
}

pub fn copy(path: &str) {
    run_shell_command(&format!("cp {path}"));
}

pub fn edit(file: &str) {
    run_shell_command(&format!("nano {file}"));
}

pub fn set_window_title(title: &str) {
    print!("\x1b]0;{title}\x07");
    io::stdout().flush().unwrap();
}

pub fn del(file: &str) {
    run_shell_command(&format!("rm {file}"));
}

pub fn ls(path: &str) {
    run_shell_command(&format!("ls {path}"));
}

pub fn ping(add: &str) {
    run_shell_command(&format!("ping {add}"));
}

pub fn wait(time: &str) {
    run_shell_command(&format!("sleep {time}"));
}

pub fn update() {
    let package_manager = detect_package_manager();

    if package_manager == "apt" {
        // Debian/Ubuntu
        run_shell_command("sudo apt update && sudo apt upgrade");
    } else if package_manager == "dnf" {
        // Fedora
        run_shell_command("sudo dnf update");
    } else {
        // Arch
        run_shell_command("sudo pacman -Syu");

        // Use Yay if enabled
        match load().get("considerYayAsAPackageManager").map(String::as_str) {
            Some("true") => {
                run_shell_command("yay -Syu")
            }
            Some(_) => {},
            None => println!("Setting 'considerYayAsAPackageManager' not found in config!\nTry reloading Rusterminal!"),
        }
    }
}

pub fn web(url: &str) {
    Command::new("xdg-open")
        .arg(url)
        .spawn()
        .expect("Failed to open webpage");
}

pub fn ver() {
    println!("\nRusterminal version: {VERSION}");
    println!("Rust version: {}", rustc_version::version().unwrap());

    match load().get("showSystemInformationInVerCMD").map(String::as_str) {
        Some("true") => {
            let home_dir = env::var("HOME").expect("Failed to get HOME directory");
            let python_script = format!("{home_dir}/rusterminal/src/ver.py");

            let _ = Command::new("python3")
                .arg(python_script)
                .status()
                .expect("Failed to execute Python script");
        }
        Some(_) => println!(),
        None => println!("Setting 'showSystemInformationInVerCMD' not found in config!\nTry reloading Rusterminal!"),
    }
}

pub fn run_shell_command(cmd: &str) {
    if cmd.trim().is_empty() {
        return;
    }

    // Use a shell (`sh -c`) so multi-word commands and colors work properly
    let _ = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdin(Stdio::inherit()) // Allows interactive commands (like sudo pacman -Syu)
        .stdout(Stdio::inherit()) // Preserves output colors
        .stderr(Stdio::inherit()) // Preserves error messages
        .status(); // Run the command and wait
}

pub fn detect_package_manager() -> String {
    if Command::new("pacman").output().is_ok() {
        "pacman".to_string()
    } else if Command::new("dnf").output().is_ok() {
        "dnf".to_string()
    } else if Command::new("apt").output().is_ok() {
        "apt".to_string()
    } else {
        "none".to_string()
    }
}

pub fn help() {
    let rustver = rustc_version::version().unwrap();
    println!("Rusterminal {VERSION} (Rustc {rustver})");
    println!("Type \"cmds\" or \"credits\" for more information.\n");
}

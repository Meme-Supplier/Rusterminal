#!/usr/bin/env rust-script
#[cfg(target_os = "linux")]

/* 2025 Meme Supplier
memesupplierbusiness@gmail.com
Maintained by Meme Supplier */

use std::env;
use std::io::{self, Write};
use std::process::{Command, Stdio};

static VERSION: &str = "v0.2.1";

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

pub fn new_dir(x: &str) {
    let y = format!("mkdir {x}");
    run_shell_command(&y);
}

pub fn input(x: &str) {
    let mut input = String::new();

    // Print the prompt and flush stdout
    println!("{x}");
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
        run_shell_command("sudo pacman -Rns $(pacman -Qdtq) --noconfirm")
    }
}

pub fn copy(x: &str) {
    let command = format!("cp {x}");
    run_shell_command(&command);
}

pub fn edit(x: &str) {
    let y = format!("nano {x}");
    run_shell_command(&y);
}

pub fn set_window_title(title: &str) {
    print!("\x1b]0;{title}\x07");
    io::stdout().flush().unwrap(); // Ensure the escape sequence is sent immediately
}

pub fn del(x: &str) {
    let y = format!("rm {x}");
    run_shell_command(&y);
}

pub fn ls(x: &str) {
    let y = format!("ls {x}");
    run_shell_command(&y);
}

pub fn ping(add: &str) {
    let addr = format!("ping {add}");
    run_shell_command(&addr);
}

pub fn wait(time: &str) {
    let cmd = format!("sleep {time}");
    run_shell_command(&cmd);
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

    // Resolve home directory
    let home_dir = env::var("HOME").expect("Failed to get HOME directory");
    let python_script = format!("{home_dir}/rusterminal/src/ver.py");

    // Run the Python script using 'python3'
    let _ = Command::new("python3")
        .arg(python_script)
        .status()
        .expect("Failed to execute Python script");
}

pub fn run(cmd: &str) {
    // Use sh to execute commands that may require shell features
    let _ = Command::new("sh")
        .arg("-c") // Use the -c option to pass the command as a string to the shell
        .arg(cmd) // Pass the command to be executed
        .stdin(Stdio::inherit()) // Allows interactive commands like sudo
        .stdout(Stdio::inherit()) // Preserves colored output
        .stderr(Stdio::inherit()) // Preserves error messages
        .status();
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
    println!("Welcome to Rusterminal {VERSION}!");
    println!("Type \"cmds\" for a list of commands!\n")
}

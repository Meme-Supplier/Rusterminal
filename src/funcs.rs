#!/usr/bin/env rust-script
#[cfg(target_os = "linux")]

/* 2025 Meme Supplier
memesupplierbusiness@gmail.com
Maintained by Meme Supplier */

use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process::exit;
use std::process::{Command, Stdio};
use regex::Regex;
use rustc_version_runtime::version;

use crate::process_input;

use crate::logger::get_time;
use crate::logger::init;
use crate::logger::log;

pub const VERSION: &str = "v0.3.2-beta4";

pub fn load_configs() -> HashMap<String, String> {
    let home_dir = env::var("HOME").expect("Failed to get HOME directory");

    let content = fs::read_to_string(format!("{home_dir}/.config/rusterminal/settings.conf"))
        .expect("Failed to read config");

    let mut config = HashMap::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue; // skip empty lines or comments
        }
        if let Some((key, value)) = line.split_once('=') {
            config.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    config
}

pub fn run_rusterminal_script(path: &str) {
    log(&format!(
        "funcs::run_rusterminal_script(): Running Rusterminal script: {path}"
    ));

    let file = File::open(path);
    if let Ok(file) = file {
        let reader = BufReader::new(file);
        for line_result in reader.lines() {
            if let Ok(line) = line_result {
                process_input(&line)
            } else {
                eprintln!("Failed to read a line in {path}");
                log(&format!("funcs::run_rusterminal_script(): Failed to read a line in Rusterminal script: {path}"))
            }
        }
    } else {
        eprintln!("Failed to open script file: {path}");
        log(&format!("funcs::run_rusterminal_script(): Failed to open script file in Rusterminal script: {path}"))
    }
}

pub fn exit_rusterminal() {
    match load_configs()
        .get("cleanCompileOnStartup")
        .map(String::as_str)
    {
        Some("true") => {
            log("funcs::exit_rusterminal(): Cleaning up Rusterminal before closing...");
            run_shell_command("rm -rf $HOME/rusterminal/target");
            exit(0)
        }
        Some(_) => {
            log("funcs::exit_rusterminal(): Exiting Rusterminal...");
            init(&format!("\n===== End Session {} =====\n", get_time()));
            exit(0)
        }
        None => {
            println!(
                "Setting 'cleanCompileOnStartup' not found in config!\nTry reloading Rusterminal!"
            );
            log("funcs::exit_rusterminal(): Setting 'cleanCompileOnStartup' not found in config!")
        }
    }
}

pub fn run_python(script: &str) {
    log(&format!(
        "funcs::run_python(): Running Python script: {script}"
    ));

    let _ = Command::new("python3")
        .arg(script)
        .status()
        .expect("Failed to execute Python script");
}

pub fn fmtdsk() {
    log("funcs::fmtdsk(): Running disk formatter script.");

    let home_dir = &env::var("HOME").expect("Failed to get HOME directory");
    run_python(&format!("{home_dir}/rusterminal/src/diskfmt.py"));

    log("funcs::fmtdsk(): Disk formatting successful.")
}

pub fn new_dir(dir: &str) {
    log(&format!("funcs::new_dir(): Creating directory: {dir}"));
    run_shell_command(&format!("mkdir {dir}"))
}

pub fn input(str: &str) {
    let mut input = String::new();

    println!("{str}");
    io::stdout().flush().expect("Failed to flush");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    print!("{input}")
}

pub fn clean() {
    if detect_package_manager().as_str() == "apt" {
        run_shell_command("sudo apt autoremove -y")
    } else if detect_package_manager().as_str() == "dnf" {
        run_shell_command("sudo dnf autoremove -y")
    } else {
        run_shell_command("sudo pacman -Rns $(pacman -Qdtq) --noconfirm");
        match load_configs()
            .get("considerYayAsAPackageManager")
            .map(String::as_str)
        {
            Some("true") => run_shell_command("yay -Rns $(yay -Qdtq) --noconfirm"),
            Some(_) => {}
            None => {
                println!("Setting 'considerYayAsAPackageManager' not found in config!\nTry reloading Rusterminal!");
                log("funcs::clean(): Setting 'considerYayAsAPackageManager' not found in config!")
            }
        }

        match load_configs()
            .get("considerParuAsAPackageManager")
            .map(String::as_str)
        {
            Some("true") => run_shell_command("paru -Rns $(paru -Qdtq) --noconfirm"),
            Some(_) => {}
            None => {
                println!("Setting 'considerParuAsAPackageManager' not found in config!\nTry reloading Rusterminal!");
                log("funcs::clean(): Setting 'considerParuAsAPackageManager' not found in config!")
            }
        }
    }

    log("funcs::clean(): Cleaning up system cache...");
    run_shell_command("sudo rm -rf ~/.cache || exit")
}

pub fn echo(text: &str) {
    log(&format!("funcs::echo(): Running echoing text: {text}"));
    run_shell_command(&format!("echo -e {text}"))
}

pub fn copy(path: &str) {
    log(&format!("funcs::copy(): Copying file: {path}"));
    run_shell_command(&format!("cp {path}"))
}

pub fn edit(file: &str) {
    log(&format!("funcs::edit(): Editing file: {file}"));
    run_shell_command(&format!("nano {file}"))
}

pub fn set_window_title(title: &str) {
    log(&format!(
        "funcs::set_window_title(): Setting window title: {title}"
    ));
    print!("\x1b]0;{title}\x07");
    io::stdout().flush().unwrap()
}

pub fn del(file: &str) {
    log(&format!("funcs::del(): Deleting file: {file}"));
    run_shell_command(&format!("rm {file}"))
}

pub fn ls(path: &str) {
    log(&format!("funcs::ls(): Listing directory: {path}"));
    run_shell_command(&format!("ls {path}"))
}

pub fn ping(add: &str) {
    log(&format!("funcs::ping(): Pinging web address: {add}"));
    run_shell_command(&format!("ping {add}"))
}

pub fn wait(time: &str) {
    log(&format!("funcs::wait(): Waiting {time} seconds..."));
    run_shell_command(&format!("sleep {time}"))
}

pub fn update() {
    let package_manager = detect_package_manager();

    if package_manager == "apt" {
        run_shell_command("sudo apt update && sudo apt upgrade") // Debian/Ubuntu
    } else if package_manager == "dnf" {
        run_shell_command("sudo dnf update") // Fedora
    } else {
        run_shell_command("sudo pacman -Syyu"); // Arch

        match load_configs()
            .get("considerYayAsAPackageManager")
            .map(String::as_str)
        {
            Some("true") => run_shell_command("yay -Syyu"),
            Some(_) => {}
            None => {
                println!("Setting 'considerYayAsAPackageManager' not found in config!\nTry reloading Rusterminal!");
                log("Setting 'considerYayAsAPackageManager' not found in config!")
            }
        }

        match load_configs()
            .get("considerParuAsAPackageManager")
            .map(String::as_str)
        {
            Some("true") => run_shell_command("paru -Syyu"),
            Some(_) => {}
            None => {
                println!("Setting 'considerParuAsAPackageManager' not found in config!\nTry reloading Rusterminal!");
                log("funcs::update(): Setting 'considerParuAsAPackageManager' not found in config!")
            }
        }
    }

    log("funcs::update(): Updated system.")
}

pub fn web(url: &str) {
    log(&format!("funcs::web(): Opening webpage: {url}"));

    Command::new("xdg-open")
        .arg(url)
        .spawn()
        .expect("Failed to open webpage");
}

pub fn ver() {
    println!("\nRusterminal version: {VERSION}");
    println!("Rust version: {}", rustc_version::version().unwrap());

    match load_configs()
        .get("showSystemInformationInVerCMD")
        .map(String::as_str)
    {
        Some("true") => {
            let home_dir = env::var("HOME").expect("Failed to get HOME directory");
            run_python(&format!("{home_dir}/rusterminal/src/ver.py"))
        }
        Some(_) => {}
        None => {
            println!("Setting \"showSystemInformationInVerCMD\" not found in config!\nTry reloading Rusterminal!");
            log("funcs::ver(): Setting \"showSystemInformationInVerCMD\" not found in config!")
        }
    }
}

pub fn run_shell_command(cmd: &str) {
    log(&format!(
        "funcs::run_shell_command(): Running shell command: {cmd}"
    ));

    if cmd.trim().is_empty() {
        return;
    }

    let _ = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();
}

pub fn detect_package_manager() -> String {
    match load_configs()
        .get("forceDisablePackageManagerCheck")
        .map(String::as_str)
    {
        Some("false") => {
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
        Some(_) => "none".to_string(),
        None => {
            log("funcs::detect_package_manager(): Setting 'forceDisablePackageManagerCheck' not found in config!");
            println!("Setting 'forceDisablePackageManagerCheck' not found in config!\nTry reloading Rusterminal!");
            "none".to_string()
        }
    }
}

pub fn help() {
    let output = Command::new("python3")
        .arg("--version")
        .output()
        .expect("failed to execute process");

    let raw_output = if output.stdout.is_empty() {
        String::from_utf8_lossy(&output.stderr).to_string()
    } else {
        String::from_utf8_lossy(&output.stdout).to_string()
    };

    let re = Regex::new(r"\b(\d+\.\d+\.\d+)\b").unwrap();
    let python_version = re
        .captures(&raw_output)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str())
        .unwrap_or("unknown");

    let rust_version = version();

    println!("Rusterminal {VERSION} (Rustc {rust_version}) (Python {python_version})");
    println!("Type \"rusterminal\" to get started.\n")
}

#!/usr/bin/env rust-script
#[cfg(target_os = "linux")]

/* 2025 Meme Supplier
memesupplierbusiness@gmail.com
Maintained by Meme Supplier */

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;
use std::process::{Command, Stdio};
use reqwest;
use regex::Regex;
use rustc_version_runtime::version;
use once_cell::sync::OnceCell;


static LATEST: OnceCell<&'static str> = OnceCell::new();
static BETA: OnceCell<&'static str> = OnceCell::new();

pub const VERSION: &str = "v0.3.1-beta4"; // Replace with actual version

pub async fn init_versions() {
    let latest_url = "https://raw.githubusercontent.com/Meme-Supplier/Rusterminal/main/VERSION";
    let beta_url = "https://raw.githubusercontent.com/Meme-Supplier/Rusterminal/beta/VERSION";

    let latest = fetch_version_online(latest_url).await.unwrap_or_else(|_| "unknown".to_string());
    let beta = fetch_version_online(beta_url).await.unwrap_or_else(|_| "unknown".to_string());

    let _ = LATEST.set(Box::leak(latest.into_boxed_str()));
    let _ = BETA.set(Box::leak(beta.into_boxed_str()));
}

pub async fn fetch_version_online(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    response.text().await
}

pub fn get_latest_version() -> Option<&'static str> {
    LATEST.get().copied()
}

pub fn get_beta_version() -> Option<&'static str> {
    BETA.get().copied()
}

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

pub fn exit_rusterminal() {
    match load_configs()
        .get("cleanCompileOnStartup")
        .map(String::as_str)
    {
        Some("true") => {
            run_shell_command("rm -rf $HOME/rusterminal/target");
            exit(0)
        }
        Some(_) => exit(0),
        None => println!(
            "Setting 'cleanCompileOnStartup' not found in config!\nTry reloading Rusterminal!"
        ),
    }
}

pub fn run_python(script: &str) {
    let _ = Command::new("python3")
        .arg(script)
        .status()
        .expect("Failed to execute Python script");
}

pub fn fmtdsk() {
    let home_dir = &env::var("HOME").expect("Failed to get HOME directory");
    run_python(&format!("{home_dir}/rusterminal/src/diskfmt.py"));
}

pub fn new_dir(dir: &str) {
    run_shell_command(&format!("mkdir {dir}"));
}

pub fn input(str: &str) {
    let mut input = String::new();

    println!("{str}");
    io::stdout().flush().expect("Failed to flush");

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
        match load_configs().get("considerYayAsAPackageManager").map(String::as_str) {
            Some("true") => run_shell_command("yay -Rns $(pacman -Qdtq) --noconfirm"),
            Some(_) => {},
            None => println!("Setting 'considerYayAsAPackageManager' not found in config!\nTry reloading Rusterminal!"),
        }

        match load_configs().get("considerParuAsAPackageManager").map(String::as_str) {
            Some("true") => run_shell_command("paru -Rns $(pacman -Qdtq) --noconfirm"),
            Some(_) => {},
            None => println!("Setting 'considerParuAsAPackageManager' not found in config!\nTry reloading Rusterminal!"),
        }
    }

    run_shell_command("sudo rm -rf ~/.cache || exit");
}

pub fn echo(text: &str) {
    run_shell_command(&format!("echo -e {text}"));
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
        run_shell_command("sudo pacman -Syyu");

        match load_configs().get("considerYayAsAPackageManager").map(String::as_str) {
            Some("true") => run_shell_command("yay -Syyu"),
            Some(_) => {},
            None => println!("Setting 'considerYayAsAPackageManager' not found in config!\nTry reloading Rusterminal!"),
        }

        match load_configs().get("considerParuAsAPackageManager").map(String::as_str) {
            Some("true") => run_shell_command("paru -Syyu"),
            Some(_) => {},
            None => println!("Setting 'considerParuAsAPackageManager' not found in config!\nTry reloading Rusterminal!"),
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

    match load_configs().get("showSystemInformationInVerCMD").map(String::as_str) {
        Some("true") => {
            let home_dir = env::var("HOME").expect("Failed to get HOME directory");
            run_python(&format!("{home_dir}/rusterminal/src/ver.py"));
        }
        Some(_) => {},
        None => println!("Setting 'showSystemInformationInVerCMD' not found in config!\nTry reloading Rusterminal!"),
    }
}

pub fn run_shell_command(cmd: &str) {
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
    println!("Type \"rusterminal\" to get started.\n");
}

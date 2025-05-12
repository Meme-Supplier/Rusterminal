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
use rustc_version_runtime::version; // Add rustc_version_runtime to Cargo.toml
use regex::Regex;
use std::process::exit;

static VERSION: &str = "v0.2.8";

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

pub fn man(command: &str) {
    match command {
        "build" => println!("Usage: Builds Rusterminal to the path specified in \"~/.config/rusterminal/settings.conf\""),
        "clean" => println!("Usage: Cleans your system and deletes temporary files."),
        "clear" => println!("Usage: Clears the screen."),
        "credits" => println!("Usage: Provides information about the creator."),
        "cmds" => println!("Usage: Lists the available commands."),
        "copy <path>" => println!("Usage: Copies the specified path to the specified directory."),
        "del <path>" => println!("Usage: Deletes the specified file/directory."),
        "echo <text>" => println!("Usage: Prints your desired output to the terminal."),
        "edit <path>" => println!("Usage: Uses GNU Nano to edit your desired file."),
        "exit" => println!("Usage: Copies the specified path to the specified directory."),
        "expr <equation>" => println!("Usage: Calculates an equation."),
        "fmtdsk" => println!("Usage: Utility to format USB drives."),
        "help" => println!("Usage: Something to get you started with Rusterminal"),
        "ls <path>" => println!("Usage: Lists the available directories/files in your specified directory."),
        "newdir <path>" => println!("Usage: Creates a new directory in your desired path."),
        "ping <site>" => println!("Usage: Pings an internet address. Useful for testing your internet connectivity."),
        "python / python3" => println!("Usage: Runs Python."),
        "run <command>" => println!("Usage: Runs a normal shell command."),
        "restart" => println!("Usage: Restarts your system."),
        "rmtitle" => println!("Usage: Resets the terminal window name."),
        "settings" => println!("Usage: Allows you to change Rusterminal's configurations."),
        "shutdown" => println!("Usage: Powers down your system."),
        "title <title>" => println!("Usage: Allows you to set the title of the terminal window."),
        "uninstall" => println!("Usage: Uninstalls Rusterminal."),
        "update" => println!("Usage: Fully updates your system."),
        "upgrade" => println!("Usage: Updates Rusterminal."),
        "ver" => println!("Usage: Shows system information."),
        "wait <time>" => println!("Usage: Waits your specified amount of time (in seconds)."),
        "xray" => println!("Usage: Allows you to edit Rusterminal's source code."),
        _ => println!("Unknown command: {command}"),
    }
}

pub fn exit_rusterminal() {
    match load_configs().get("cleanCompileOnStartup").map(String::as_str) {
        Some("true") => {
            run_shell_command("rm -rf $HOME/rusterminal/target");
            exit(0)
        },
        Some(_) => exit(0),
        None => println!("Setting 'cleanCompileOnStartup' not found in config!\nTry reloading Rusterminal!"),
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

    run_shell_command("sudo apt autoremove -y");
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
        run_shell_command("sudo pacman -Syu");

        match load_configs().get("considerYayAsAPackageManager").map(String::as_str) {
            Some("true") => run_shell_command("yay -Syu"),
            Some(_) => {},
            None => println!("Setting 'considerYayAsAPackageManager' not found in config!\nTry reloading Rusterminal!"),
        }

        match load_configs().get("considerParuAsAPackageManager").map(String::as_str) {
            Some("true") => run_shell_command("paru -Syu"),
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
        Some(_) => println!(),
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
    match load_configs().get("forceDisablePackageManagerCheck").map(String::as_str) {
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
    println!("Type \"cmds\" or \"credits\" for more information.\n");
}

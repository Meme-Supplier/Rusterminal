#!/usr/bin/env rust-script

/*
2025 Meme Supplier
memesupplierbusiness@gmail.com
Maintained by Meme Supplier
*/

use std::env;
use std::io::{self, Write};
use std::process::{Command, Stdio};

static VERSION: &str = "v0.1.6";

#[cfg(target_os = "linux")]

pub fn cmds() {
    let lines: [&str; 28] = [
        "",
        "clear",
        "exit",
        "cmds",
        "help",
        "echo <text>",
        "sh <command>",
        "expr <equation>",
        "restart",
        "shutdown",
        "ver",
        "uptime",
        "update",
        "python / python3",
        "xray",
        "uninstall",
        "wait",
        "ping <site>",
        "ls <path>",
        "del <path>",
        "title <title>",
        "rmtitle",
        "edit <path>",
        "copy <path>",
        "upgrade",
        "clean",
        "newdir <path>",
        "",
    ];

    for line in lines.iter() {
        println!("{}", line);
    }
}

pub fn new_dir(x: &str) {
    let y = format!("mkdir {}", x);
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
    let y = format!("cp {}", x);
    run_shell_command(&y);
}

pub fn edit(x: &str) {
    let y = format!("nano {}", x);
    run_shell_command(&y);
}

pub fn set_window_title(title: &str) {
    print!("\x1b]0;{}\x07", title);
    io::stdout().flush().unwrap(); // Ensure the escape sequence is sent immediately
}

pub fn del(x: &str) {
    let y = format!("rm {}", x);
    run_shell_command(&y);
}

pub fn ls(x: &str) {
    let y = format!("ls {}", x);
    run_shell_command(&y);
}

pub fn ping(add: &str) {
    let addr = format!("ping {}", add);
    run_shell_command(&addr);
}

pub fn wait(time: &str) {
    let cmd = format!("sleep {}", time);
    run_shell_command(&cmd);
}

pub fn update() {
    let package_manager = detect_package_manager();

    if package_manager == "apt" {
        // Debian/Ubuntu
        run_shell_command("sudo apt update");
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
    println!("\nRusterminal version: {}", VERSION);
    println!("Rust version: {}", rustc_version::version().unwrap());

    // Resolve home directory
    let home_dir = env::var("HOME").expect("Failed to get HOME directory");
    let python_script = format!("{}/rusterminal/src/ver.py", home_dir);

    // Run the Python script using 'python3'
    let _ = Command::new("python3")
        .arg(python_script)
        .status()
        .expect("Failed to execute Python script");
}

pub fn sh(cmd: &str) {
    // Use sh to execute commands that may require shell features
    let _ = Command::new("sh")
        .arg("-c") // Use the -c option to pass the command as a string to the shell
        .arg(cmd) // Pass the command to be executed
        .stdin(Stdio::inherit()) // Allows interactive commands like sudo
        .stdout(Stdio::inherit()) // Preserves colored output
        .stderr(Stdio::inherit()) // Preserves error messages
        .status(); // Use `.status()` instead of `.output()`
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
    let package_managers = [
        ("pacman", "pacman --version"),
        ("apt", "apt --version"),
        ("dnf", "dnf --version"),
    ];

    for (packman, command) in &package_managers {
        if let Ok(output) = Command::new("sh").arg("-c").arg(command).output() {
            if output.status.success() {
                return packman.to_string();
            }
        }
    }

    "Unknown".to_string() // If the package manager can't be detected
}

pub fn help() {
    println!("Welcome to Rusterminal {}!", VERSION);
    println!("Type \"cmds\" for a list of commands!\n")
}

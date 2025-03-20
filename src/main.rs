#!/usr/bin/env rust-script

/*
2025 Meme Supplier
memesupplierbusiness@gmail.com
Maintained by Meme Supplier
*/

use gethostname::gethostname;
use std::env;
use std::io::{self, Write};
use std::process::{exit, Command, Stdio};

static VERSION: &str = "v0.1.35";

#[cfg(target_os = "linux")]

fn process_input(input: &str) {
    for command in input.split("&&").map(|s| s.trim()) {
        if command.is_empty() { continue; }

        match command {
            "clear" => run_shell_command("clear"),
            "exit" => { exit(0); }
            "cmds" => cmds(),
            "ver" => ver(),
            "reload" => run_shell_command("cargo run main.rs"),
            "help" => help(),
            "shutdown" => run_shell_command("sudo shutdown now"),
            "restart" => run_shell_command("sudo reboot"),
            "uptime" => run_shell_command("uptime"),
            "python" | "python3" => run_shell_command("python3"),
            "update" => update(),
            "xray" => xray(),

            // Commands with arguments
            _ if command.starts_with("echo ") => println!("{}", &command[5..]),
            _ if command.starts_with("sh ") => sh(&command[3..]),
            _ if command.starts_with("web ") => web(&command[4..]),
            _ if command.starts_with("expr ") => sh(command),
            _ if command.starts_with("wait ") => wait(&command[5..]),

            _ => println!("{}: command not found", command),
        }
    }
}

fn xray() {
    run_shell_command("nano ./main.rs");
}

fn wait(time: &str) {
    let cmd = format!("sleep {}", time);
    run_shell_command(&cmd);
}

fn update() {
    if detect_package_manager().as_str() == "apt" { // Debian/Ubuntu
        run_shell_command("sudo apt update");
    } else if detect_package_manager().as_str() == "dnf" { // Fedora
        run_shell_command("sudo dnf update");
    } else { // Arch
        run_shell_command("sudo pacman -Syu")
    }
}

fn web(url: &str) {
    Command::new("xdg-open")
        .arg(url)
        .spawn()
        .expect("Failed to open webpage");
}

fn ver() {
    println!("\nRusterminal version: {}", VERSION);
    println!("Rust version: {}", rustc_version::version().unwrap());

    // Path to the Python file
    let python_script = "./ver.py"; // Replace with your Python file name

    // Run the Python script using 'python' or 'python3'
    let _ = Command::new("python3") // or "python3"
        .arg(python_script)
        .status(); // Run the command without checking status
}

fn sh(cmd: &str) {
    // Use sh to execute commands that may require shell features
    let _ = Command::new("sh")
        .arg("-c") // Use the -c option to pass the command as a string to the shell
        .arg(cmd) // Pass the command to be executed
        .stdin(Stdio::inherit()) // Allows interactive commands like sudo
        .stdout(Stdio::inherit()) // Preserves colored output
        .stderr(Stdio::inherit()) // Preserves error messages
        .status(); // Use `.status()` instead of `.output()`
}

fn run_shell_command(cmd: &str) {
    if cmd.trim().is_empty() { return; }

    // Use a shell (`sh -c`) so multi-word commands and colors work properly
    let _ = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdin(Stdio::inherit()) // Allows interactive commands (like sudo pacman -Syu)
        .stdout(Stdio::inherit()) // Preserves output colors
        .stderr(Stdio::inherit()) // Preserves error messages
        .status(); // Run the command and wait
}

fn detect_package_manager() -> String {
    let package_managers = [
        ("pacman", "pacman --version"),
        ("apt", "apt --version"),
        ("dnf", "dnf --version"),
    ];

    for (packman, command) in &package_managers {
        if Command::new("sh").arg("-c").arg(command).output().is_ok() {
            return packman.to_string();
        }
    }

    "Unknown".to_string() // If the package manager can't be detected
}

fn help() {
    println!("Welcome to Rusterminal {}!", VERSION);
    println!("Type \"cmds\" for a list of commands!\n")
}

fn cmds() {
    let lines: [&str; 17] = [
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
        "reload",
        "uptime",
        "update",
        "python / python3",
        "xray",
        "",
    ];

    for line in lines.iter() {
        println!("{}", line);
    }
}

fn main() {
    // Attempt to clear the screen
    run_shell_command("clear");

    if env::consts::OS == "linux" {
        // Do nothing if it's Linux
    } else {
        println!("Rusterminal is designed for Linux only!\nExiting...");
        exit(0);
    }

    if matches!(detect_package_manager().as_str(), "apt" | "dnf" | "pacman") {
        // Do nothing if a supported package manager
    } else {
        println!("Unsupported package manager! Rusterminal only supports Apt, Dnf, and Pacman.");
        exit(0);
    }

    help();

    loop {
        let mut input = String::new();

        // Print the prompt and flush stdout
        print!(
            "{}@{}$~: ",
            gethostname().to_string_lossy(),
            gethostname().to_string_lossy()
        );
        io::stdout().flush().expect("Failed to flush");

        // Read user input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Process the input
        process_input(input.trim());
    }
}

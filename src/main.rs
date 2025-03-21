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

static VERSION: &str = "v0.1.4";

#[cfg(target_os = "linux")]

fn process_input(input: &str) {
    for command in input.split("&&").map(|s| s.trim()) {
        if command.is_empty() {
            continue;
        }

        match command {
            "clear" => run_shell_command("clear"),
            "exit" => {
                exit(0);
            }
            "cmds" => cmds(),
            "ver" => ver(),
            "reload" | "rusterminal" => run_shell_command("cargo run main.rs"),
            "help" => help(),
            "shutdown" => run_shell_command("sudo shutdown now"),
            "restart" => run_shell_command("sudo reboot"),
            "uptime" => run_shell_command("uptime"),
            "python" | "python3" => run_shell_command("python3"),
            "update" => update(),
            "xray" => run_shell_command("nano main.rs"),
            "uninstall" => run_shell_command("cd ~/rusterminal/src/ && bash uninstall.sh"),
            "rmtitle" => set_window_title("Rusterminal"),

            // Commands that require syntax
            "echo" => println!("Usage: echo <text>"),
            "sh" => println!("Usage: sh <command>"),
            "web" => println!("Usage: web <website>"),
            "expr" => println!("Usage: expr <equation>"),
            "wait" => println!("Usage: wait <time>"),
            "ping" => println!("Usage: ping <domain>"),
            "ls" => println!("Usage: ls <directory>"),
            "del" => println!("Usage: del <flag> <file/directory>"),
            "title" => println!("Usage: title <string>"),
            "edit" => println!("Usage: edit <path>"),
            "copy" => println!("Usage: copy <flag> <path>"),

            // Commands with arguments
            _ if command.starts_with("echo ") => println!("{}", &command[5..]),
            _ if command.starts_with("sh ") => sh(&command[3..]),
            _ if command.starts_with("web ") => web(&command[4..]),
            _ if command.starts_with("expr ") => sh(command),
            _ if command.starts_with("wait ") => wait(&command[5..]),
            _ if command.starts_with("ping ") => ping(&command[5..]),
            _ if command.starts_with("ls ") => ls(&command[3..]),
            _ if command.starts_with("del ") => del(&command[4..]),
            _ if command.starts_with("title ") => set_window_title(&command[6..]),
            _ if command.starts_with("edit ") => edit(&command[5..]),
            _ if command.starts_with("copy ") => copy(&command[5..]),

            _ => println!("{}: command not found", command),
        }
    }
}

fn cmds() {
    let lines: [&str; 26] = [
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
        "reload / rusterminal",
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
        "",
    ];

    for line in lines.iter() {
        println!("{}", line);
    }
}

fn copy(x: &str) {
    let y = format!("cp {}", x);
    run_shell_command(&y);
}

fn edit(x: &str) {
    let y = format!("nano {}", x);
    run_shell_command(&y);
}

fn set_window_title(title: &str) {
    print!("\x1b]0;{}\x07", title);
    io::stdout().flush().unwrap(); // Ensure the escape sequence is sent immediately
}

fn del(x: &str) {
    let y = format!("del {}", x);
    run_shell_command(&y);
}

fn ls(x: &str) {
    let y = format!("ls {}", x);
    run_shell_command(&y);
}

fn ping(add: &str) {
    let addr = format!("ping {}", add);
    run_shell_command(&addr);
}

fn wait(time: &str) {
    let cmd = format!("sleep {}", time);
    run_shell_command(&cmd);
}

fn update() {
    if detect_package_manager().as_str() == "apt" {
        // Debian/Ubuntu
        run_shell_command("sudo apt update");
    } else if detect_package_manager().as_str() == "dnf" {
        // Fedora
        run_shell_command("sudo dnf update");
    } else {
        // Arch
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
    let python_script = "~/rusterminal/src/ver.py"; // Replace with your Python file name

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

    set_window_title("Rusterminal");
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
